use std::ops::Range;

use crate::core::error::PitchError;
use crate::core::fft_space::FftSpace;
use crate::core::utils::interpolated_peak_at;
use crate::core::FftPoint;
use crate::pitch::IntoFrequencyDomain;
use rustfft::FftPlanner;

use super::PitchDetector;

#[derive(Debug, Clone)]
pub struct HannedFftDetector {
    fft_space: Option<FftSpace>,
}

impl Default for HannedFftDetector {
    fn default() -> Self {
        Self { fft_space: None }
    }
}

impl HannedFftDetector {
    fn unscaled_spectrum(&self, bin_range: (usize, usize)) -> Box<dyn Iterator<Item = f64> + '_> {
        if let Some(ref fft_space) = self.fft_space {
            let (lower_limit, upper_limit) = bin_range;
            let normalize = 1. / (fft_space.padded_len() as f64).sqrt();
            Box::new(
                fft_space
                    .freq_domain_iter(true)
                    .skip(lower_limit)
                    .take(upper_limit - lower_limit)
                    .map(move |(amplitude, _)| amplitude * normalize),
            )
        } else {
            panic!("FFT space not initialized");
        }
    }

    fn process_fft(&mut self) {
        if let Some(ref mut fft_space) = self.fft_space {
            let mut planner = FftPlanner::new();
            let fft_len = fft_space.padded_len();
            let signal_len = fft_space.signal_len();
            let fft = planner.plan_fft_forward(fft_len);

            let (space, scratch) = fft_space.workspace();
            let hanning = apodize::hanning_iter(signal_len);
            space.iter_mut().zip(hanning).for_each(|(s, h)| s.re *= h);
            fft.process_with_scratch(space, scratch);
        } else {
            panic!("fft_space is None");
        }
    }
}

impl IntoFrequencyDomain for HannedFftDetector {
    fn into_frequency_domain(
        &mut self,
        signal: &[f64],
        freq_range: Option<(Range<f64>, f64)>,
    ) -> (usize, Vec<f64>) {
        if self.fft_space.is_none() {
            self.fft_space = Some(FftSpace::new(signal.len()));
        }
        self.fft_space
            .as_mut()
            .unwrap()
            .init_with_signal(signal.iter());
        self.process_fft();
        let bin_range = match freq_range {
            Some((r, sample_rate)) => (
                self.freq_to_bin(r.start, sample_rate).round() as usize,
                self.freq_to_bin(r.end, sample_rate).round() as usize,
            ),
            // The first half of the fft spectrum is conventionally the only important part.
            None => (0, signal.len() / 2),
        };
        (bin_range.0, self.unscaled_spectrum(bin_range).collect())
    }

    fn bin_to_freq(&self, bin: f64, sample_rate: f64) -> f64 {
        if let Some(ref fft_space) = self.fft_space {
            bin * sample_rate / fft_space.padded_len() as f64
        } else {
            panic!("RawFftDetector needs to be initialized with a FftSpace first");
        }
    }
    fn freq_to_bin(&self, freq: f64, sample_rate: f64) -> f64 {
        if let Some(ref fft_space) = self.fft_space {
            freq * fft_space.padded_len() as f64 / sample_rate
        } else {
            panic!("RawFftDetector needs to be initialized with a FftSpace first");
        }
    }
}

impl PitchDetector for HannedFftDetector {
    fn detect_pitch_in_range(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range: Range<f64>,
    ) -> Result<f64, PitchError> {
        let (start_bin, spectrum) =
            self.into_frequency_domain(signal, Some((freq_range, sample_rate)));
        let max_bin = spectrum
            .iter()
            .enumerate()
            .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
            .ok_or(PitchError::IncorrectParameters(
                "Spectrum had no elements".to_string(),
            ))?;

        let FftPoint { x: bin, .. } = interpolated_peak_at(&spectrum, max_bin.0)?;
        Ok(self.bin_to_freq(bin + start_bin as f64, sample_rate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{test_fundamental_freq, test_sine_wave};

    #[test]
    fn test_from_sample_files() -> anyhow::Result<()> {
        let mut detector = HannedFftDetector::default();

        test_fundamental_freq(&mut detector, "tuner_c5.wav", 523.242)?;
        test_fundamental_freq(&mut detector, "cello_open_a.wav", 219.383)?;
        test_fundamental_freq(&mut detector, "cello_open_d.wav", 146.732)?;
        test_fundamental_freq(&mut detector, "cello_open_g.wav", 97.209)?;

        // Fails to detect open C, which should be around 64 Hz
        test_fundamental_freq(&mut detector, "cello_open_c.wav", 129.046)?;
        Ok(())
    }

    #[test]
    fn test_from_sine_wave() -> anyhow::Result<()> {
        let mut detector = HannedFftDetector::default();
        test_sine_wave(&mut detector, 440.)?;
        Ok(())
    }
}
