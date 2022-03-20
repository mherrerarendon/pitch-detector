pub mod autocorrelation;
pub mod cepstrum;
pub mod raw_fft;

use crate::core::fft_space::FftSpace;

pub trait PitchDetector {
    fn detect(&mut self, signal: &[f64], sample_rate: f64) -> Option<f64>
where {
        let mut fft_space = FftSpace::new(signal.len());
        fft_space.init_with_signal(signal);
        self.detect_with_fft_space(sample_rate, &mut fft_space)
    }

    fn detect_with_fft_space(&mut self, sample_rate: f64, fft_space: &mut FftSpace) -> Option<f64>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FftPoint {
    pub x: f64,
    pub y: f64,
}

impl Default for FftPoint {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[cfg(feature = "test_utils")]
pub trait FrequencyDetectorTest {
    fn unscaled_spectrum(&self, signal: &[f64], fft_range: (usize, usize)) -> Vec<f64>;

    fn relevant_fft_range(&self, fft_space_len: usize, sample_rate: f64) -> (usize, usize);

    fn detect_unscaled_freq(
        &mut self,
        signal: &[f64],
        fft_range: (usize, usize),
    ) -> Option<FftPoint> {
        let mut fft_space = FftSpace::new(signal.len());
        fft_space.init_with_signal(signal);
        self.detect_unscaled_freq_with_space(fft_range, &mut fft_space)
    }

    fn detect_unscaled_freq_with_space(
        &mut self,
        fft_range: (usize, usize),
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint>;
    fn name(&self) -> &'static str;
}
