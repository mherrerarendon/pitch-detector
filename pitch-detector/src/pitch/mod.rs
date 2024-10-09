//! The recommended algorithm is the [HannedFftDetector](crate::pitch::hanned_fft::HannedFftDetector),
//! which is the most versatile. [PowerCepstrum](crate::pitch::cepstrum::PowerCepstrum) on the other hand, is less versatile,
//! but it is able to detect a fundamental from a sample that includes many harmonics. This means that [PowerCepstrum](crate::pitch::cepstrum::PowerCepstrum)
//! is good for detecting sounds that are rich in harmonics, as well as low pitched sounds, but bad at detecting samples
//! with fewer partials.
//!
mod autocorrelation;
mod cepstrum;
mod hanned_fft;

pub use cepstrum::PowerCepstrum;
pub use hanned_fft::HannedFftDetector;

use std::ops::Range;

use crate::core::{error::PitchError, into_frequency_domain::IntoFrequencyDomain};

pub trait PitchDetector {
    /// The default implementation will detect within a conventional range of frequencies (20Hz to nyquist).
    /// If you want to detect a pitch in a specific range, use the [detect_pitch_in_range](Self::detect_pitch_in_range) method
    fn detect_pitch(&mut self, signal: &[f64], sample_rate: f64) -> Result<f64, PitchError> {
        let nyquist_freq = sample_rate / 2.;
        let min_freq = 20.; // Conventional minimum frequency for human hearing
        self.detect_pitch_in_range(signal, sample_rate, min_freq..nyquist_freq)
    }

    /// Default implementation to detect a pitch within the specified frequency range.
    fn detect_pitch_in_range(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range: Range<f64>,
    ) -> Result<f64, PitchError>;
}
