use crate::{constants::*, fft_space::FftSpace, peak_iter::FftPeaks, FrequencyDetector, Partial};
use rustfft::{num_complex::Complex, FftPlanner};

pub struct PowerCepstrum {
    fft_space: FftSpace,
}

impl FrequencyDetector for PowerCepstrum {
    fn detect_frequency<I: IntoIterator>(&mut self, signal: I) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);
        self.fft_space
            .map(|f| Complex::new(f.norm_sqr().log(std::f64::consts::E), 0.0));
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);

        self.spectrum()
            .into_iter()
            .fft_peaks(60, 10.)
            .reduce(|accum, quefrency| {
                if quefrency.1 > accum.1 {
                    quefrency
                } else {
                    accum
                }
            })
            .map(|(mu, amplitude)| Partial {
                freq: SAMPLE_RATE / mu,
                intensity: amplitude,
            })
            .map(|partial| partial.freq)
    }

    fn spectrum(&self) -> Vec<(usize, f64)> {
        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;

        self.fft_space
            .freq_domain(false)
            .map(|(amplitude, _)| amplitude)
            .enumerate()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .collect()
    }

    #[cfg(test)]
    fn name(&self) -> &'static str {
        POWER_CEPSTRUM_ALGORITHM
    }
}

impl PowerCepstrum {
    pub fn new(fft_space_size: usize) -> Self {
        PowerCepstrum {
            fft_space: FftSpace::new(fft_space_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_power() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum::new(TEST_FFT_SPACE_SIZE);

        // Power cepstrum fails to detect the C5 note, which should be at around 523Hz
        test_fundamental_freq(&mut detector, "tuner_c5.json", 261.591)?;

        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.418)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.730)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.214)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.454)?;
        Ok(())
    }
}
