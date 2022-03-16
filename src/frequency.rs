pub mod autocorrelation;
pub mod cepstrum;
pub mod raw_fft;

use crate::core::fft_space::FftSpace;

pub trait FrequencyDetector {
    fn detect_frequency<I>(&mut self, signal: I, sample_rate: f64) -> Option<f64>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let signal_iter = signal.into_iter();
        let mut fft_space = FftSpace::new_padded(
            signal_iter
                .size_hint()
                .1
                .expect("Signal length is not known"),
        );
        self.detect_frequency_with_fft_space(signal_iter, sample_rate, &mut fft_space)
    }

    fn detect_frequency_with_fft_space<I>(
        &mut self,
        signal: I,
        sample_rate: f64,
        fft_space: &mut FftSpace,
    ) -> Option<f64>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>;
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
    fn spectrum<'a, I>(&self, signal: I, sample_rate: f64) -> Vec<(usize, f64)>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
        I: IntoIterator + 'a;
    fn detect_unscaled_freq<I: IntoIterator>(
        &mut self,
        signal: I,
        sample_rate: f64,
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>;
    fn name(&self) -> &'static str;
}
