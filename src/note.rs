mod note_detection_result;

#[cfg(feature = "hinted")]
mod hinted;
#[cfg(feature = "hinted")]
pub mod peak_detector;

use std::ops::Range;

use crate::pitch::PitchDetector;

use self::note_detection_result::NoteDetectionResult;

pub fn detect_note<D: PitchDetector>(
    signal: &[f64],
    freq_detector: &mut D,
    sample_rate: f64,
    freq_range_hint: Option<Range<f64>>,
) -> Option<NoteDetectionResult> {
    freq_detector
        .detect_pitch(signal, sample_rate, freq_range_hint)
        .and_then(|f| f.try_into().ok())
}
