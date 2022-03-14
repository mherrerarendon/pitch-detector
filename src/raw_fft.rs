use crate::{constants::*, fft_space::FftSpace, FrequencyDetector, Partial};
use rustfft::FftPlanner;
use std::borrow::Borrow;

use super::peak_iter::FftPeaks;

pub struct RawFftDetector {
    fft_space: FftSpace,
}

impl FrequencyDetector for RawFftDetector {
    fn detect_frequency<I: IntoIterator>(&mut self, signal: I) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.fft_space.len());
        let signal_iter = signal.into_iter();
        let signal_size = signal_iter
            .size_hint()
            .1
            .expect("Failed to get size hint for signal");
        self.fft_space.init_fft_space(
            signal_iter
                .zip(apodize::hanning_iter(signal_size))
                .map(|(x, y)| x.borrow() * y),
        );

        let (fft_space, scratch) = self.fft_space.workspace();
        fft.process_with_scratch(fft_space, scratch);
        self.spectrum()
            .into_iter()
            .fft_peaks(40, 10.)
            .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
            .map(|item| Partial {
                freq: item.0 * SAMPLE_RATE / self.fft_space.len() as f64,
                intensity: item.1,
            })
            .map(|partial| partial.freq)
    }

    fn spectrum(&self) -> Vec<(usize, f64)> {
        let lower_limit = (MIN_FREQ * self.fft_space.len() as f64 / SAMPLE_RATE).round() as usize;
        let upper_limit = (MAX_FREQ * self.fft_space.len() as f64 / SAMPLE_RATE).round() as usize;
        self.fft_space
            .freq_domain(true)
            .enumerate()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .map(|(i, (amplitude, _))| (i, amplitude))
            .collect()
    }

    #[cfg(test)]
    fn name(&self) -> &'static str {
        RAW_FFT_ALGORITHM
    }
}

impl RawFftDetector {
    pub fn new(fft_space_size: usize) -> Self {
        RawFftDetector {
            fft_space: FftSpace::new(fft_space_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_raw_fft() -> anyhow::Result<()> {
        let mut detector = RawFftDetector::new(TEST_FFT_SPACE_SIZE);

        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.383)?;

        // Fails to detect open d, which should be at around 146 Hz
        test_fundamental_freq(&mut detector, "cello_open_d.json", 293.390)?;

        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.209)?;

        // Fails to detect open C, which should be around 64 Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 129.046)?;
        Ok(())
    }
}
