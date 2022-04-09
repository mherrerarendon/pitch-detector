use std::ops::Range;

use crate::{
    core::{utils::interpolated_peak_at, NoteName},
    note::peak_detector::{PeakDetector, ZScoreDetector},
    pitch::SignalToSpectrum,
};

use super::note_detection_result::NoteDetectionResult;

pub trait HintedNoteDetector: SignalToSpectrum {
    fn detect_note_with_hint<I>(
        &mut self,
        note_hint: NoteName,
        signal: &[f64],
        sample_rate: f64,
        freq_range_hint: Option<Range<f64>>,
    ) -> Option<NoteDetectionResult>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let (start_bin, spectrum) =
            self.signal_to_spectrum(signal, freq_range_hint.map(|r| (r, sample_rate)));
        const LAG: usize = 40;
        const THRESHOLD: f64 = 6.;
        const INFLUENCE: f64 = 0.;
        let peak_detector = ZScoreDetector::new(LAG, THRESHOLD, INFLUENCE);
        let mut candidates = peak_detector.detect_peaks(&spectrum);
        candidates.sort_by(|a, b| b.partial_cmp(&a).unwrap());
        candidates
            .iter()
            .find(|bin| {
                let result = NoteDetectionResult::try_from(bin.bin);
                if let Ok(result) = result {
                    result.note_name == note_hint
                } else {
                    false
                }
            })
            .and_then(|bin| interpolated_peak_at(&spectrum, bin.bin))
            .and_then(|fft_point| {
                NoteDetectionResult::try_from(fft_point.x + start_bin as f64).ok()
            })
    }
}
