mod peak_detector;
mod peak_iter;
use std::ops::Range;

use crate::{
    core::{utils::interpolated_peak_at, NoteName},
    note::hinted::peak_detector::{PeakDetector, ZScoreDetector},
    pitch::SignalToSpectrum,
};

use super::note_detection_result::NoteDetectionResult;

pub trait HintedNoteDetector: SignalToSpectrum {
    fn detect_note_with_hint(
        &mut self,
        note_hint: NoteName,
        signal: &[f64],
        sample_rate: f64,
        freq_range_hint: Option<Range<f64>>,
    ) -> Option<NoteDetectionResult> {
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
                let freq = self.bin_to_freq((bin.bin + start_bin) as f64, sample_rate);
                let result = NoteDetectionResult::try_from(freq);
                if let Ok(result) = result {
                    result.note_name == note_hint
                } else {
                    false
                }
            })
            .and_then(|bin| interpolated_peak_at(&spectrum, bin.bin))
            .and_then(|fft_point| {
                let freq = self.bin_to_freq(fft_point.x + start_bin as f64, sample_rate);
                NoteDetectionResult::try_from(freq).ok()
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::test_utils::hinted::{assert_hinted_detector, assert_hinted_detector_sine_waves},
        pitch::hanned_fft::HannedFftDetector,
    };

    use super::*;

    #[test]
    fn test_hinted_detector() -> anyhow::Result<()> {
        pub const TEST_SAMPLE_RATE: f64 = 44000.0;
        let mut detector = HannedFftDetector::default();
        assert_hinted_detector(
            &mut detector,
            "tuner_c5.json",
            TEST_SAMPLE_RATE,
            NoteName::C,
        )?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_a.json",
            TEST_SAMPLE_RATE,
            NoteName::A,
        )?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_d.json",
            TEST_SAMPLE_RATE,
            NoteName::D,
        )?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_g.json",
            TEST_SAMPLE_RATE,
            NoteName::G,
        )?;
        assert_hinted_detector(
            &mut detector,
            "cello_open_c.json",
            TEST_SAMPLE_RATE,
            NoteName::C,
        )?;
        Ok(())
    }

    // #[test]
    // fn test_with_mixed_wave_signal() -> anyhow::Result<()> {
    //     let mut detector = HannedFftDetector::default();
    //     assert_hinted_detector_sine_waves(&mut detector, NoteName::A, vec![440., 523.25])?;
    //     Ok(())
    // }
}
