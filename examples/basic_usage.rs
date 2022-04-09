use anyhow::Result;
use float_cmp::ApproxEq;
use pitch_detector::{
    core::utils::sine_wave_signal,
    pitch::{hanned_fft::HannedFftDetector, PitchDetector},
};

const NUM_SAMPLES: usize = 16384;
const SAMPLE_RATE: f64 = 44100.0;
const A440: f64 = 440.0;
const MAX_FREQ: f64 = 1046.50; // C6
const MIN_FREQ: f64 = 32.7; // C1

fn example_detect_frequency() -> Result<()> {
    let mut detector = HannedFftDetector::default();
    let signal = sine_wave_signal(NUM_SAMPLES, A440, SAMPLE_RATE);
    let freq = detector
        .detect_pitch(&signal, SAMPLE_RATE, Some(MIN_FREQ..MAX_FREQ))
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;
    assert!(
        freq.approx_eq(A440, (0.02, 2)),
        "Expected freq: {}, actual freq: {}",
        A440,
        freq
    );
    Ok(())
}

fn main() -> Result<()> {
    example_detect_frequency()?;
    Ok(())
}
