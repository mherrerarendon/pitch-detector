mod plot;

use crate::plot::plot_spectrum;
use pitch_detector::{
    core::{
        into_frequency_domain::IntoFrequencyDomain, test_utils::test_signal,
        utils::mixed_wave_signal,
    },
    pitch::{HannedFftDetector, PitchDetector, PowerCepstrum},
};

const TEST_FILE_SAMPLE_RATE: f64 = 44000.;
pub const MAX_FREQ: f64 = 1046.50; // C6
pub const MIN_FREQ: f64 = 32.7; // C1

fn plot_detector_for_files<D: PitchDetector + IntoFrequencyDomain>(
    mut detector: D,
    title: &str,
    test_files: &[&str],
) -> anyhow::Result<()> {
    for test_file in test_files {
        let test_signal = test_signal(test_file)?;
        plot_spectrum(
            &mut detector,
            &test_signal,
            MIN_FREQ..MAX_FREQ,
            TEST_FILE_SAMPLE_RATE,
            title,
            test_file,
        )?;
    }
    Ok(())
}

fn plot_detector_for_freqs<D: PitchDetector + IntoFrequencyDomain>(
    mut detector: D,
    title: &str,
    freq: Vec<f64>,
) -> anyhow::Result<()> {
    const TEST_FILE_SAMPLE_RATE: f64 = 44100.;
    const NUM_SAMPLES: usize = 16384;
    let test_signal = mixed_wave_signal(NUM_SAMPLES, freq, TEST_FILE_SAMPLE_RATE);
    plot_spectrum(
        &mut detector,
        &test_signal,
        MIN_FREQ..MAX_FREQ,
        TEST_FILE_SAMPLE_RATE,
        title,
        "sine wave",
    )
}
fn main() -> anyhow::Result<()> {
    let test_files = [
        "cello_open_a.json",
        "cello_open_d.json",
        "cello_open_g.json",
        "cello_open_c.json",
        "tuner_c5.json",
        // "noise.json",
    ];

    plot_detector_for_files(PowerCepstrum::default(), "Power Cepstrum", &test_files)?;
    plot_detector_for_files(HannedFftDetector::default(), "Hanned", &test_files)?;

    plot_detector_for_freqs(HannedFftDetector::default(), "Hanned", vec![440.])?;
    plot_detector_for_freqs(HannedFftDetector::default(), "Hannded", vec![440., 523.])?;
    Ok(())
}
