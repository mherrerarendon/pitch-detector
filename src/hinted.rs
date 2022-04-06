mod note;
pub mod peak_detector;

use crate::core::{fft_space::FftSpace, NoteName};

use self::note::NoteDetectionResult;

pub trait HintedNoteDetector {
    // Returns list of bins and magnitudes.
    fn unscaled_candidates(&mut self, fft_space: ) -> Vec<(f64, f64)>;

    fn detect_with_hint_and_fft_space<I>(
        &mut self,
        note_hint: NoteName,
        signal: &[f64],
        sample_rate: f64,
        fft_space: &mut FftSpace,
    ) -> Option<NoteDetectionResult>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let mut candidates = self.unscaled_candidates(signal, sample_rate);
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        candidates
            .iter()
            .find(|c| {
                let result = NoteDetectionResult::try_from(c.0);
                if let Ok(result) = result {
                    result.note_name == note_hint
                } else {
                    false
                }
            })
            .and_then(|c| NoteDetectionResult::try_from(c.0).ok())
    }
}
