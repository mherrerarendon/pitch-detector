use anyhow::Result;
use float_cmp::ApproxEq;
use pitch_detector::{
    core::{utils::sine_wave_signal, NoteName},
    note::{detect_note, hinted::HintedNoteDetector},
    pitch::{hanned_fft::HannedFftDetector, PitchDetector},
};

const NUM_SAMPLES: usize = 16384;
const SAMPLE_RATE: f64 = 44100.0;
const MAX_FREQ: f64 = 1046.50; // C6
const MIN_FREQ: f64 = 32.7; // C1

fn example_detect_frequency() -> Result<()> {
    let mut detector = HannedFftDetector::default();
    let signal = sine_wave_signal(NUM_SAMPLES, 440., SAMPLE_RATE);
    let freq = detector
        .detect_pitch(&signal, SAMPLE_RATE, Some(MIN_FREQ..MAX_FREQ))
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;
    assert!(
        freq.approx_eq(440., (0.02, 2)),
        "Expected freq: {}, actual freq: {}",
        440.,
        freq
    );
    Ok(())
}

fn example_detect_note() -> Result<()> {
    let mut detector = HannedFftDetector::default();
    let slightly_sharp_a = 448.;
    let signal = sine_wave_signal(NUM_SAMPLES, slightly_sharp_a, SAMPLE_RATE);
    let note = detect_note(
        &signal,
        &mut detector,
        SAMPLE_RATE,
        Some(MIN_FREQ..MAX_FREQ),
    )
    .ok_or(anyhow::anyhow!("Did not get note"))?;
    assert_eq!(note.note_name, NoteName::A);
    assert!(note.cents_offset > 0.);
    Ok(())
}

fn example_hinted_note() -> anyhow::Result<()> {
    let mut detector = HannedFftDetector::default();
    let slightly_sharp_a = 448.;
    let in_tune_c = 523.25;
    let signal_a = sine_wave_signal(NUM_SAMPLES, slightly_sharp_a, SAMPLE_RATE);
    let signal_c = sine_wave_signal(NUM_SAMPLES, in_tune_c, SAMPLE_RATE);
    let combined_signal: Vec<f64> = signal_a.iter().zip(signal_c).map(|(a, c)| a + c).collect();
    let note = detector
        .detect_note_with_hint(
            NoteName::A,
            &combined_signal,
            SAMPLE_RATE,
            Some(MIN_FREQ..MAX_FREQ),
        )
        .ok_or(anyhow::anyhow!("Did not get note"))?;
    assert_eq!(note.note_name, NoteName::A);
    assert!(note.cents_offset > 0.);
    Ok(())
}

fn main() -> Result<()> {
    example_detect_frequency()?;
    example_detect_note()?;
    example_hinted_note()?;
    Ok(())
}
