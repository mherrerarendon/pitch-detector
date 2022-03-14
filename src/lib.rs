pub mod autocorrelation;
pub mod cepstrum;
pub mod raw_fft;

mod constants;
mod fft_space;
mod peak_iter;
mod utils;

use self::{
    autocorrelation::AutocorrelationDetector, cepstrum::PowerCepstrum, raw_fft::RawFftDetector,
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait FrequencyDetector {
    fn detect_frequency<I: IntoIterator>(&mut self, signal: I) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>;

    fn spectrum(&self) -> Vec<(usize, f64)>;

    #[cfg(test)]
    fn name(&self) -> &'static str;
}

#[enum_dispatch(FrequencyDetector)]
pub enum Detector {
    RawFftDetector,
    PowerCepstrum,
    AutocorrelationDetector,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Partial {
    pub freq: f64,
    pub intensity: f64,
}

impl Default for Partial {
    fn default() -> Self {
        Self {
            freq: 0.0,
            intensity: 0.0,
        }
    }
}
