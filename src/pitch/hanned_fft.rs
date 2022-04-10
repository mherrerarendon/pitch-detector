use std::ops::Range;

use crate::core::fft_space::FftSpace;
use crate::pitch::SignalToSpectrum;
use rustfft::FftPlanner;

use super::PitchDetector;

#[derive(Debug, Clone)]
pub struct HannedFftDetector {
    fft_space: Option<FftSpace>,
}

impl PitchDetector for HannedFftDetector {}

impl Default for HannedFftDetector {
    fn default() -> Self {
        Self { fft_space: None }
    }
}

impl HannedFftDetector {
    fn unscaled_spectrum(&self, bin_range: (usize, usize)) -> Box<dyn Iterator<Item = f64> + '_> {
        if let Some(ref fft_space) = self.fft_space {
            let (lower_limit, upper_limit) = bin_range;
            Box::new(
                fft_space
                    .freq_domain(true)
                    .skip(lower_limit)
                    .take(upper_limit - lower_limit)
                    .map(|(amplitude, _)| amplitude),
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

impl SignalToSpectrum for HannedFftDetector {
    fn signal_to_spectrum(
        &mut self,
        signal: &[f64],
        freq_range: Option<(Range<f64>, f64)>,
    ) -> (usize, Vec<f64>) {
        if self.fft_space.is_none() {
            self.fft_space = Some(FftSpace::new(signal.len()));
        }
        self.fft_space.as_mut().unwrap().init_with_signal(signal);
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

    fn name(&self) -> &'static str {
        "rawfft"
    }
}
#[cfg(feature = "hinted")]
mod hinted {
    use crate::note::hinted::HintedNoteDetector;

    use super::HannedFftDetector;
    impl HintedNoteDetector for HannedFftDetector {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{test_fundamental_freq, test_signal, test_sine_wave};

    #[test]
    fn test_raw_fft() -> anyhow::Result<()> {
        let mut detector = HannedFftDetector::default();

        test_fundamental_freq(&mut detector, "tuner_c5.json", 523.242)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.383)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.732)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.209)?;

        // Fails to detect open C, which should be around 64 Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 129.046)?;
        Ok(())
    }

    // #[test]
    fn test_noise() -> anyhow::Result<()> {
        pub const TEST_SAMPLE_RATE: f64 = 44000.0;
        let signal = test_signal("noise.json")?;

        let mut detector = HannedFftDetector::default();
        assert!(detector
            .detect_pitch(&signal, TEST_SAMPLE_RATE, None)
            .is_none());

        Ok(())
    }

    #[test]
    fn test_raw_fft_sine() -> anyhow::Result<()> {
        let mut detector = HannedFftDetector::default();
        test_sine_wave(&mut detector, 440.)?;
        Ok(())
    }
}
