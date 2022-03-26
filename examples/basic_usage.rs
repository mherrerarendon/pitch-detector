#![feature(iterator_try_reduce)]

use anyhow::Result;
use float_cmp::ApproxEq;
use num_traits::Float;
use pitch_detector::{
    core::{fft_space::FftSpace, utils::sine_wave_signal},
    pitch::{raw_fft::RawFftDetector, PitchDetector},
};

const NUM_SAMPLES: usize = 16384;
const SAMPLE_RATE: f64 = 44100.0;
const A440: f64 = 440.0;

fn example_detect_frequency() -> Result<()> {
    let mut detector = RawFftDetector;
    let signal = sine_wave_signal(NUM_SAMPLES, A440, SAMPLE_RATE);
    let freq = detector
        .detect(&signal, SAMPLE_RATE)
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;
    assert!(
        freq.approx_eq(A440, (0.02, 2)),
        "Expected freq: {}, actual freq: {}",
        A440,
        freq
    );
    Ok(())
}

fn example_detect_frequency_reduced_alloc() -> Result<()> {
    let mut detector = RawFftDetector;
    let mut fft_space = FftSpace::new(NUM_SAMPLES);
    for i in 0..10 {
        let freq = A440 + i as f64;
        let signal = sine_wave_signal(NUM_SAMPLES, freq, SAMPLE_RATE);
        fft_space.init_with_signal(signal);
        let actual_freq = detector
            .detect_with_fft_space(SAMPLE_RATE, &mut fft_space)
            .ok_or(anyhow::anyhow!("Did not get pitch"))?;
        assert!(
            actual_freq.approx_eq(freq, (0.1, 1)),
            "Expected: {}, Actual: {}",
            freq,
            actual_freq
        );
    }
    Ok(())
}

fn fitting_bug() -> Result<()> {
    let y_vals = vec![-1., 1., -1.];
    let sum_y_pow2: f64 = y_vals.iter().map(|y| y.powi(2)).sum();
    let sum_y_pow2: f64 = y_vals
        .into_iter()
        .try_fold(0.0_f64, |accum, elem| {
            if elem.is_sign_positive() {
                Some(accum + elem.powi(2))
            } else {
                None
            }
        })
        .ok_or(anyhow::anyhow!("No sum"))?;
    // .ok_or(anyhow::anyhow!("No sum"))?;
    let (mu, _, a) = fitting::gaussian::fit(vec![1., 2., 3.].into(), vec![-1., 1., -1.].into())?;
    assert_eq!(mu, f64::NAN);
    Ok(())
}

fn main() -> Result<()> {
    example_detect_frequency()?;
    example_detect_frequency_reduced_alloc()?;
    fitting_bug()?;
    Ok(())
}
