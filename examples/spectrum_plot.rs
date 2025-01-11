use pitch_detector::{
    core::{test_utils::test_signal, utils::mixed_wave_signal},
    pitch::{HannedFftDetector, PitchDetector, PowerCepstrum, SignalToSpectrum},
    plot::plot_spectrum,
};

const TEST_FILE_SAMPLE_RATE: f64 = 44000.;
pub const MAX_FREQ: f64 = 1046.50; // C6
pub const MIN_FREQ: f64 = 32.7; // C1

fn plot_detector_for_files<D: PitchDetector + SignalToSpectrum>(
    mut detector: D,
    test_files: &[&str],
) -> anyhow::Result<()> {
    for test_file in test_files {
        let test_signal = test_signal(test_file)?;
        plot_spectrum(
            &mut detector,
            &test_signal,
            MIN_FREQ..MAX_FREQ,
            TEST_FILE_SAMPLE_RATE,
            test_file,
        )?;
    }
    Ok(())
}

fn plot_detector_for_freqs<D: PitchDetector + SignalToSpectrum>(
    mut detector: D,
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

    plot_detector_for_files(PowerCepstrum::default(), &test_files)?;
    plot_detector_for_files(HannedFftDetector::default(), &test_files)?;

    plot_detector_for_freqs(HannedFftDetector::default(), vec![440.])?;
    plot_detector_for_freqs(HannedFftDetector::default(), vec![440., 523.])?;

    let wav_file = format!("{}/test_data/wav/banjo.wav", env!("CARGO_MANIFEST_DIR"));
    let mut reader = hound::WavReader::open(wav_file).unwrap();
    let wav_spec = reader.spec();
    println!("sample formats: {:?}", wav_spec.sample_format);
    let sample_rate = reader.spec().sample_rate;
    let samples = reader
        .samples::<i32>()
        .map(|s| s.unwrap() as f64)
        .collect::<Vec<f64>>();

    let mut detector = HannedFftDetector::default();
    plot_spectrum(
        &mut detector,
        &samples,
        20.0..1046.4,
        sample_rate.into(),
        "banjo",
    )?;
    Ok(())
}
