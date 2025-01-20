//! For scenarios where an audio sample might contain multiple frequencies, especially when those other frequencies
//! might be very prominent, `HintedNoteDetector` supports identifying the accuracy of a specified note.
//! See the top level documentation for more information and examples

use std::ops::Range;

use crate::{
    core::{
        error::PitchError, into_frequency_domain::ToFrequencyDomain, utils::interpolated_peak_at,
        NoteName,
    },
    note::peak_detector::{PeakDetector, PeakFinderDetector},
};

use super::note_detection_result::NoteDetection;

pub trait HintedNoteDetector {
    fn detect_note_with_hint(
        &mut self,
        note_hint: NoteName,
        signal: &[f64],
        sample_rate: f64,
    ) -> Result<NoteDetection, PitchError> {
        self.detect_note_with_hint_and_range(note_hint, signal, sample_rate, None)
    }

    fn detect_note_with_hint_and_range(
        &mut self,
        note_hint: NoteName,
        signal: &[f64],
        sample_rate: f64,
        freq_range_hint: Option<Range<f64>>,
    ) -> Result<NoteDetection, PitchError>;
}

impl<T> HintedNoteDetector for T
where
    T: ToFrequencyDomain,
{
    fn detect_note_with_hint_and_range(
        &mut self,
        note_hint: NoteName,
        signal: &[f64],
        sample_rate: f64,
        freq_range_hint: Option<Range<f64>>,
    ) -> Result<NoteDetection, PitchError> {
        let (start_bin, spectrum) =
            self.to_frequency_domain(signal, freq_range_hint.map(|r| (r, sample_rate)));
        const THRESHOLD: f64 = 6.;
        let peak_detector = PeakFinderDetector::new(THRESHOLD);
        let mut candidates = peak_detector.detect_peaks(&spectrum);
        candidates.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let bin = candidates
            .iter()
            .find(|bin| {
                let freq = self.bin_to_freq((bin.bin + start_bin) as f64, sample_rate);
                let result = NoteDetection::try_from(freq);
                if let Ok(result) = result {
                    result.note_name == note_hint
                } else {
                    false
                }
            })
            .ok_or(PitchError::NoPitchDetected(
                "Did not find pitch that matches hint".to_string(),
            ))?;
        let fft_point = interpolated_peak_at(&spectrum, bin.bin)?;
        let freq = self.bin_to_freq(fft_point.x + start_bin as f64, sample_rate);
        NoteDetection::try_from(freq)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{
            constants::{MAX_FREQ, MIN_FREQ},
            test_utils::{hinted::assert_hinted_detector_sine_waves, test_signal},
        },
        pitch::HannedFftDetector,
    };

    use super::*;

    pub fn assert_hinted_detector<D: HintedNoteDetector>(
        detector: &mut D,
        samples_file: &str,
        file_sample_rate: f64,
        expected_note: NoteName,
    ) -> anyhow::Result<()> {
        let signal = test_signal(samples_file)?;
        assert_eq!(
            detector
                .detect_note_with_hint_and_range(
                    expected_note,
                    &signal,
                    file_sample_rate,
                    Some(MIN_FREQ..MAX_FREQ)
                )?
                .note_name,
            expected_note
        );
        Ok(())
    }

    #[test]
    fn test_hinted_detector() -> anyhow::Result<()> {
        pub const TEST_SAMPLE_RATE: f64 = 44100.0;
        let mut detector = HannedFftDetector::default();
        assert_hinted_detector(&mut detector, "tuner_c5.wav", TEST_SAMPLE_RATE, NoteName::C)?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_a.wav",
            TEST_SAMPLE_RATE,
            NoteName::A,
        )?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_d.wav",
            TEST_SAMPLE_RATE,
            NoteName::D,
        )?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_g.wav",
            TEST_SAMPLE_RATE,
            NoteName::G,
        )?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_c.wav",
            TEST_SAMPLE_RATE,
            NoteName::C,
        )?;
        Ok(())
    }

    #[test]
    fn test_with_mixed_wave_signal() -> anyhow::Result<()> {
        let mut detector = HannedFftDetector::default();
        assert_hinted_detector_sine_waves(&mut detector, NoteName::A, vec![440., 523.25])?;
        Ok(())
    }
}
