use std::ops::Range;

use pitch_detector::{
    core::{
        test_utils::test_signal,
        utils::{mixed_wave_signal, sine_wave_signal},
    },
    pitch::{
        cepstrum::PowerCepstrum, hanned_fft::HannedFftDetector, PitchDetector, SignalToSpectrum,
    },
};
use plotters::prelude::*;

const TEST_FILE_SAMPLE_RATE: f64 = 44000.;
pub const MAX_FREQ: f64 = 1046.50; // C6
pub const MIN_FREQ: f64 = 32.7; // C1

fn plot<D>(
    detector: &mut D,
    signal: &[f64],
    freq_range: Range<f64>,
    sample_rate: f64,
    plot_name: &str,
) -> anyhow::Result<()>
where
    D: PitchDetector + SignalToSpectrum,
{
    let max_freq = detector
        .detect_pitch(signal, sample_rate, Some(freq_range.clone()))
        .ok_or(anyhow::anyhow!("No pitch"))?;
    let max_bin = detector.freq_to_bin(max_freq, sample_rate);
    let plot_title = format!(
        "{} - {} - {:.2} max bin",
        detector.name(),
        plot_name,
        max_bin
    );
    let output_file = format!(
        "{}/test_data/results/{}.png",
        env!("CARGO_MANIFEST_DIR"),
        format!("{} - {}", detector.name(), plot_name)
    );

    let (start_bin, y_vals) = detector.signal_to_spectrum(signal, Some((freq_range, sample_rate)));
    let x_vals: Vec<f64> = (start_bin..y_vals.len() + start_bin)
        .map(|x| x as f64)
        .collect();
    assert_eq!(
        x_vals.len(),
        y_vals.len(),
        "x and y values are not the same length"
    );
    let y_min = y_vals.iter().cloned().reduce(f64::min).unwrap();
    let y_max = y_vals.iter().cloned().reduce(f64::max).unwrap();
    let root = BitMapBackend::new(&output_file, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(90)
        .build_cartesian_2d(x_vals[0]..x_vals[x_vals.len() - 1] as f64, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_labels(15)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    chart.draw_series(LineSeries::new(
        x_vals.iter().zip(y_vals).map(|(x, y)| (*x, y)),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

fn plot_detector_for_files<D: PitchDetector + SignalToSpectrum>(
    mut detector: D,
    test_files: &[&str],
) -> anyhow::Result<()> {
    for test_file in test_files {
        let test_signal = test_signal(test_file)?;
        plot(
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
    plot(
        &mut detector,
        &test_signal,
        MIN_FREQ..MAX_FREQ,
        TEST_FILE_SAMPLE_RATE,
        "A440",
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

    // plot_detector_for_files(AutocorrelationDetector, &test_files)?;
    plot_detector_for_files(PowerCepstrum::default(), &test_files)?;
    plot_detector_for_files(HannedFftDetector::default(), &test_files)?;

    // plot_detector_for_freq(AutocorrelationDetector, 440.)?;
    // plot_detector_for_freq(PowerCepstrum, 440.)?;
    plot_detector_for_freqs(HannedFftDetector::default(), vec![440.])?;
    Ok(())
}
