pub mod autocorrelation;
pub mod cepstrum;
pub mod core;
pub mod raw_fft;

use crate::core::fft_space::FftSpace;

use self::core::FftPoint;

pub trait PitchDetector {
    fn detect(&mut self, signal: &[f64], sample_rate: f64) -> Option<f64>
where {
        let mut fft_space = FftSpace::new(signal.len());
        fft_space.init_with_signal(signal);
        self.detect_with_fft_space(sample_rate, &mut fft_space)
    }

    fn detect_with_fft_space(&mut self, sample_rate: f64, fft_space: &mut FftSpace) -> Option<f64>;
}

#[cfg(feature = "test_utils")]
pub trait FftBinData {
    fn calc_bin_magnitudes(&self, signal: &[f64], bin_range: (usize, usize)) -> Vec<f64>;

    fn relevant_bin_range(&self, fft_space_len: usize, sample_rate: f64) -> (usize, usize);

    fn detect_max_bin(&mut self, signal: &[f64], bin_range: (usize, usize)) -> Option<FftPoint> {
        let mut fft_space = FftSpace::new(signal.len());
        fft_space.init_with_signal(signal);
        self.detect_max_bin_with_fft_space(bin_range, &mut fft_space)
    }

    fn detect_max_bin_with_fft_space(
        &mut self,
        bin_range: (usize, usize),
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint>;

    fn name(&self) -> &'static str;
}
