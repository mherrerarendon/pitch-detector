use std::ops::Range;

use crate::core::fft_space::FftSpace;
use rustfft::{num_complex::Complex, FftPlanner};

use super::{PitchDetector, SignalToSpectrum};

pub struct PowerCepstrum {
    fft_space: Option<FftSpace>,
}

impl PitchDetector for PowerCepstrum {}

impl Default for PowerCepstrum {
    fn default() -> Self {
        Self { fft_space: None }
    }
}

impl PowerCepstrum {
    fn unscaled_spectrum(&self, bin_range: (usize, usize)) -> Box<dyn Iterator<Item = f64> + '_> {
        if let Some(ref fft_space) = self.fft_space {
            let (lower_limit, upper_limit) = bin_range;
            Box::new(
                fft_space
                    .freq_domain(false)
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

impl SignalToSpectrum for PowerCepstrum {
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

    fn name(&self) -> &'static str {
        "power"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::test_fundamental_freq;

    #[test]
    fn test_power() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum::default();

        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.418)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.730)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.214)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.476)?;
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
