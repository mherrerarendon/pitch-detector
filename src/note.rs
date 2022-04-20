mod note_detection_result;

#[cfg(feature = "hinted")]
pub mod hinted;

use std::ops::Range;

use crate::pitch::PitchDetector;

pub use self::note_detection_result::NoteDetectionResult;

/// Returns the predominant note of the given signal. It will detect within a conventional
/// range of frequencies (20Hz to nyquist). If you want to detect a note in a specific range,
/// use the [detect_note_in_range] method
/// ## Examples
/// ```rust
/// use pitch_detector::{
///     core::{utils::sine_wave_signal, NoteName},
///     note::{detect_note},
///     pitch::{HannedFftDetector, PitchDetector},
/// };
/// # fn example_detect_note() -> Option<()> {
/// # const NUM_SAMPLES: usize = 16384;
/// # const SAMPLE_RATE: f64 = 44100.0;
//
/// let mut detector = HannedFftDetector::default();
/// let slightly_sharp_a = 448.;
/// let signal = sine_wave_signal(NUM_SAMPLES, slightly_sharp_a, SAMPLE_RATE);
/// let note = detect_note(
///     &signal,
///     &mut detector,
///     SAMPLE_RATE,
/// )?;
//
/// assert_eq!(note.note_name, NoteName::A);
/// assert!(note.cents_offset > 0.);
/// # None
/// # }
/// ```
pub fn detect_note<D: PitchDetector>(
    signal: &[f64],
    freq_detector: &mut D,
    sample_rate: f64,
) -> Option<NoteDetectionResult> {
    let nyquist_freq = sample_rate / 2.;
    let min_freq = 20.; // Conventional minimum frequency for human hearing
    detect_note_in_range(signal, freq_detector, sample_rate, min_freq..nyquist_freq)
}

/// Returns the predominant note of the given signal within the specified range.
/// ## Examples
/// ```rust
/// use pitch_detector::{
///     core::{utils::sine_wave_signal, NoteName},
///     note::{detect_note_in_range},
///     pitch::{HannedFftDetector, PitchDetector},
/// };
/// # fn example_detect_note() -> Option<()> {
/// # const NUM_SAMPLES: usize = 16384;
/// # const SAMPLE_RATE: f64 = 44100.0;
/// const MAX_FREQ: f64 = 1046.50; // C6
/// const MIN_FREQ: f64 = 32.7; // C1
///
/// let mut detector = HannedFftDetector::default();
/// let slightly_sharp_a = 448.;
/// let signal = sine_wave_signal(NUM_SAMPLES, slightly_sharp_a, SAMPLE_RATE);
/// let note = detect_note_in_range(
///     &signal,
///     &mut detector,
///     SAMPLE_RATE,
///     MIN_FREQ..MAX_FREQ,
/// )?;
//
/// assert_eq!(note.note_name, NoteName::A);
/// assert!(note.cents_offset > 0.);
///
/// let high_pitch = sine_wave_signal(NUM_SAMPLES, 2000., SAMPLE_RATE);
/// let note = detect_note_in_range(
///     &signal,
///     &mut detector,
///     SAMPLE_RATE,
///     MIN_FREQ..MAX_FREQ,
/// );
/// assert!(note.is_none());
/// # None
/// # }
/// ```
pub fn detect_note_in_range<D: PitchDetector>(
    signal: &[f64],
    freq_detector: &mut D,
    sample_rate: f64,
    freq_range: Range<f64>,
) -> Option<NoteDetectionResult> {
    freq_detector
        .detect_pitch_in_range(signal, sample_rate, freq_range)
        .and_then(|f| f.try_into().ok())
}
