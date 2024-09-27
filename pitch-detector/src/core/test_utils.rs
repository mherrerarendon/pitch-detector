use float_cmp::ApproxEq;

use serde::Deserialize;
use std::fs;

use crate::{
    core::{
        constants::{MAX_FREQ, MIN_FREQ},
        utils::sine_wave_signal,
    },
    pitch::PitchDetector,
};

use super::utils::audio_buffer_to_signal;

// All test files have a buffer size of 17,600 samples, and a sample rate of 44,000 Hz.

#[derive(Deserialize)]
pub struct SampleData {
    pub data: Option<Vec<u8>>,
}

pub fn test_signal(filename: &str) -> anyhow::Result<Vec<f64>> {
    let file_path = format!("{}/test_data/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let mut sample_data: SampleData = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
    let buffer = sample_data.data.take().unwrap();
    Ok(audio_buffer_to_signal(&buffer).collect())
}

#[cfg(feature = "hinted")]
pub mod hinted {
    use crate::{
        core::{
            constants::{MAX_FREQ, MIN_FREQ},
            test_utils::test_signal,
            utils::mixed_wave_signal,
            NoteName,
        },
        note::hinted::HintedNoteDetector,
    };

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
                    expected_note.clone(),
                    &signal,
                    file_sample_rate,
                    Some(MIN_FREQ..MAX_FREQ)
                )
                .ok_or(anyhow::anyhow!("error"))?
                .note_name,
            expected_note
        );
        Ok(())
    }

    pub fn assert_hinted_detector_sine_waves<D: HintedNoteDetector>(
        detector: &mut D,
        expected_note: NoteName,
        freqs: Vec<f64>,
    ) -> anyhow::Result<()> {
        const SAMPLE_RATE: f64 = 44100.0;
        let signal = mixed_wave_signal(16384, freqs, SAMPLE_RATE);
        assert_eq!(
            detector
                .detect_note_with_hint_and_range(
                    expected_note.clone(),
                    &signal,
                    SAMPLE_RATE,
                    Some(MIN_FREQ..MAX_FREQ)
                )
                .ok_or(anyhow::anyhow!("Failed to detect note with hint"))?
                .note_name,
            expected_note
        );
        Ok(())
    }
}

pub fn test_fundamental_freq<D: PitchDetector>(
    detector: &mut D,
    samples_file: &str,
    expected_freq: f64,
) -> anyhow::Result<()> {
    pub const TEST_SAMPLE_RATE: f64 = 44000.0;
    let signal = test_signal(samples_file)?;

    let freq = detector
        .detect_pitch_in_range(&signal, TEST_SAMPLE_RATE, MIN_FREQ..MAX_FREQ)
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;

    assert!(
        freq.approx_eq(expected_freq, (0.02, 2)),
        "Expected freq: {}, Actual freq: {}",
        expected_freq,
        freq
    );
    Ok(())
}

pub fn test_sine_wave<D: PitchDetector>(detector: &mut D, freq: f64) -> anyhow::Result<()> {
    const SAMPLE_RATE: f64 = 44100.0;
    let signal = sine_wave_signal(16384, 440., SAMPLE_RATE);

    let actual_freq = detector
        .detect_pitch_in_range(&signal, SAMPLE_RATE, MIN_FREQ..MAX_FREQ)
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;

    assert!(
        actual_freq.approx_eq(freq, (0.2, 2)),
        "Expected freq: {}, Actual freq: {}",
        freq,
        actual_freq
    );
    Ok(())
}
