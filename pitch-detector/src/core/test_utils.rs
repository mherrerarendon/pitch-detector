use float_cmp::ApproxEq;

use crate::{
    core::{
        constants::{MAX_FREQ, MIN_FREQ},
        utils::sine_wave_signal,
    },
    pitch::PitchDetector,
};

pub fn test_signal(filename: &str) -> anyhow::Result<Vec<f64>> {
    let file_path = format!(
        "{}/test_data/audio_recordings/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    );
    let mut reader = hound::WavReader::open(file_path).unwrap();
    Ok(reader.samples::<i16>().map(|s| s.unwrap() as f64).collect())
}

pub mod hinted {
    use crate::{
        core::{
            constants::{MAX_FREQ, MIN_FREQ},
            utils::mixed_wave_signal,
            NoteName,
        },
        note::hinted::HintedNoteDetector,
    };

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
                )?
                .note_name,
            expected_note
        );
        Ok(())
    }
}

macro_rules! test_freq {
    ($name:ident: {detector: $detector:expr, file: $file:expr, expected_freq: $expected_freq:expr}) => {
        #[test]
        fn $name() {
            use float_cmp::ApproxEq;
            pub const TEST_SAMPLE_RATE: f64 = 44100.0;
            let signal = crate::core::test_utils::test_signal($file).unwrap();

            let freq = $detector
                .detect_pitch_in_range(
                    &signal,
                    TEST_SAMPLE_RATE,
                    $crate::core::constants::MIN_FREQ..$crate::core::constants::MAX_FREQ,
                )
                .unwrap();

            assert!(
                freq.approx_eq($expected_freq, (0.02, 2)),
                "Expected freq: {}, Actual freq: {}",
                $expected_freq,
                freq
            );
        }
    };
}

pub(crate) use test_freq;

pub fn test_fundamental_freq<D: PitchDetector>(
    detector: &mut D,
    samples_file: &str,
    expected_freq: f64,
) -> anyhow::Result<()> {
    pub const TEST_SAMPLE_RATE: f64 = 44100.0;
    let signal = test_signal(samples_file)?;

    let freq = detector.detect_pitch_in_range(&signal, TEST_SAMPLE_RATE, MIN_FREQ..MAX_FREQ)?;

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

    let actual_freq = detector.detect_pitch_in_range(&signal, SAMPLE_RATE, MIN_FREQ..MAX_FREQ)?;

    assert!(
        actual_freq.approx_eq(freq, (0.2, 2)),
        "Expected freq: {}, Actual freq: {}",
        freq,
        actual_freq
    );
    Ok(())
}
