use std::ops::Range;

use crate::{
    core::{error::PitchError, fft_space::FftSpace, utils::interpolated_peak_at, FftPoint},
    note::peak_detector::{PeakDetector, PeakFinderDetector},
};
use rustfft::{num_complex::Complex, FftPlanner};

use super::{IntoFrequencyDomain, PitchDetector};

#[derive(Debug, Clone)]
pub struct PowerCepstrum {
    fft_space: Option<FftSpace>,

    /// `sigmas` is used to determine how many standard deviations should be used to determine the significance of the
    /// peaks in the fft spectrum. For example, if a value of 1 is used, then all peak heights within one standard
    /// deviation will be considered as candidates for pitch detection
    sigmas: f64,

    /// Of the candidate peaks that will be used for pitch determination, how much prominent should the best candidate
    /// be compared to the next candidate in order to be considered a pitch detection
    prominence_threshold: f64,
}

impl PowerCepstrum {
    /// Get reasonable `sigmas` and `prominence_threshold` values for reasonable pitch detection success
    pub fn new_with_defaults() -> Self {
        Self {
            sigmas: 6.,
            prominence_threshold: 1.25,
            ..Default::default()
        }
    }

    pub fn new(sigmas: f64, prominence_threshold: f64) -> Self {
        Self {
            fft_space: None,
            sigmas,
            prominence_threshold,
        }
    }

    pub fn with_sigmas(self, sigmas: f64) -> Self {
        Self { sigmas, ..self }
    }

    pub fn with_promince_threshold(self, prominence_threshold: f64) -> Self {
        Self {
            prominence_threshold,
            ..self
        }
    }
}

impl Default for PowerCepstrum {
    fn default() -> Self {
        Self {
            fft_space: None,
            sigmas: 0.,
            prominence_threshold: 0.,
        }
    }
}

impl PowerCepstrum {
    fn unscaled_spectrum(&self, bin_range: (usize, usize)) -> impl Iterator<Item = f64> + use<'_> {
        if let Some(ref fft_space) = self.fft_space {
            let (lower_limit, upper_limit) = bin_range;

            fft_space
                .freq_domain_iter(false)
                .skip(lower_limit)
                .take(upper_limit - lower_limit)
                .map(|(amplitude, _)| amplitude)
        } else {
            panic!("FFT space not initialized");
        }
    }

    fn process_fft(&mut self) {
        if let Some(ref mut fft_space) = self.fft_space {
            let mut planner = FftPlanner::new();
            let forward_fft = planner.plan_fft_forward(fft_space.padded_len());

            let (space, scratch) = fft_space.workspace();
            forward_fft.process_with_scratch(space, scratch);
            fft_space.map(|f| Complex::new(f.norm_sqr().log(std::f64::consts::E), 0.0));
            let (space, scratch) = fft_space.workspace();
            let inverse_fft = planner.plan_fft_inverse(space.len());
            inverse_fft.process_with_scratch(space, scratch);
        } else {
            panic!("FFT space not initialized");
        }
    }
}

impl IntoFrequencyDomain for PowerCepstrum {
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
                self.freq_to_bin(r.end, sample_rate).round() as usize,
                self.freq_to_bin(r.start, sample_rate).round() as usize,
            ),
            // Nyquist limit in this case is 3
            // The second half of a traditional fft corresponds to the first 2 bins (I think?)
            // Conventionally, only the first half of the traditional fft is relevant.
            None => (3, signal.len()),
        };
        (bin_range.0, self.unscaled_spectrum(bin_range).collect())
    }

    fn bin_to_freq(&self, bin: f64, sample_rate: f64) -> f64 {
        sample_rate / bin
    }
    fn freq_to_bin(&self, freq: f64, sample_rate: f64) -> f64 {
        sample_rate / freq
    }
}

impl PitchDetector for PowerCepstrum {
    fn detect_pitch_in_range(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range: Range<f64>,
    ) -> Result<f64, PitchError> {
        let (start_bin, spectrum) =
            self.into_frequency_domain(signal, Some((freq_range, sample_rate)));
        let peak_detector = PeakFinderDetector::new(self.sigmas);
        let mut candidates = peak_detector.detect_peaks(&spectrum);
        candidates.sort_by(|a, b| b.partial_cmp(&a).unwrap());
        match (candidates.get(0), candidates.get(1)) {
            (Some(freq_bin), Some(freq_bin_2)) => {
                if freq_bin.magnitude / freq_bin_2.magnitude > self.prominence_threshold {
                    let FftPoint { x: bin, .. } = interpolated_peak_at(&spectrum, freq_bin.bin)?;
                    Ok(self.bin_to_freq(bin + start_bin as f64, sample_rate))
                } else {
                    Err(PitchError::NoPitchDetected("Dominant pitch did not exceed threshold to be considered a pitch detection".to_string()))
                }
            }
            (Some(freq_bin), None) => {
                let FftPoint { x: bin, .. } = interpolated_peak_at(&spectrum, freq_bin.bin)?;
                Ok(self.bin_to_freq(bin + start_bin as f64, sample_rate))
            }
            _ => Err(PitchError::IncorrectParameters(
                "Expected to have at least one bin value".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::test_fundamental_freq;

    #[test]
    fn test_power() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum::default();

        test_fundamental_freq(&mut detector, "cello_open_a.wav", 219.418)?;
        test_fundamental_freq(&mut detector, "cello_open_d.wav", 146.730)?;
        test_fundamental_freq(&mut detector, "cello_open_g.wav", 97.214)?;
        test_fundamental_freq(&mut detector, "cello_open_c.wav", 64.476)?;
        Ok(())
    }

    // Power cepstrum doesn't work with sine waves since it looks for a harmonic sequence.
    // #[test]
    // fn test_raw_fft_sine() -> anyhow::Result<()> {
    //     let mut detector = PowerCepstrum;
    //     test_sine_wave(&mut detector, 440.)?;
    //     Ok(())
    // }
}
