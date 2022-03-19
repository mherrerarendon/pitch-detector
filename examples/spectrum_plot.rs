use freq_detector::{
    core::{fft_space::FftSpace, test_utils::test_signal, utils::sine_wave_signal},
    frequency::{
        autocorrelation::AutocorrelationDetector, cepstrum::PowerCepstrum, raw_fft::RawFftDetector,
        FrequencyDetector, FrequencyDetectorTest,
    },
};
use plotters::prelude::*;

const TEST_FILE_SAMPLE_RATE: f64 = 44000.;

fn plot<D, I>(
    detector: &D,
    signal: I,
    fft_range: (usize, usize),
    plot_name: &str,
    fft_x: f64,
) -> anyhow::Result<()>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    D: FrequencyDetector + FrequencyDetectorTest,
{
    let plot_title = format!("{} - {} - {:.2} fft_x", detector.name(), plot_name, fft_x);
    let output_file = format!(
        "{}/test_data/results/{}.png",
        env!("CARGO_MANIFEST_DIR"),
        format!("{} - {}", detector.name(), plot_name)
    );
    let signal_iter = signal.into_iter();

    let y_vals: Vec<f64> = detector.unscaled_spectrum(signal_iter, fft_range);
    let x_vals: Vec<f64> = (0..y_vals.len()).map(|x| x as f64).collect();
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

fn plot_detector_for_files<D: FrequencyDetector + FrequencyDetectorTest>(
    mut detector: D,
    test_files: &[&str],
) -> anyhow::Result<()> {
    for test_file in test_files {
        let test_signal = test_signal(test_file)?;
        let mut fft_space = FftSpace::new_padded(test_signal.len());
        let fft_range = detector.relevant_fft_range(fft_space.len(), TEST_FILE_SAMPLE_RATE);
        let fft_point_x = detector
            .detect_unscaled_freq_with_space(&test_signal, fft_range, &mut fft_space)
            .map(|p| p.x)
            .ok_or(anyhow::anyhow!(""))?;
        plot(&detector, test_signal, fft_range, test_file, fft_point_x)?;
    }
    Ok(())
}

fn plot_detector_for_freq<D: FrequencyDetector + FrequencyDetectorTest>(
    mut detector: D,
    freq: f64,
) -> anyhow::Result<()> {
    const TEST_FILE_SAMPLE_RATE: f64 = 44100.;
    const NUM_SAMPLES: usize = 16384;
    let test_signal = sine_wave_signal(NUM_SAMPLES, freq, TEST_FILE_SAMPLE_RATE);
    let mut fft_space = FftSpace::new_padded(test_signal.len());
    let fft_range = detector.relevant_fft_range(fft_space.len(), TEST_FILE_SAMPLE_RATE);
    let fft_point_x = detector
        .detect_unscaled_freq_with_space(&test_signal, fft_range, &mut fft_space)
        .map(|p| p.x)
        .ok_or(anyhow::anyhow!(
            "Failed to detect unscaled frequency with space"
        ))?;
    plot(&detector, test_signal, fft_range, "A440", fft_point_x)?;
    Ok(())
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
    // I'm not sure why the raw fft x values look wrong in the plot.

    plot_detector_for_files(AutocorrelationDetector, &test_files)?;
    plot_detector_for_files(PowerCepstrum, &test_files)?;
    plot_detector_for_files(RawFftDetector, &test_files)?;

    plot_detector_for_freq(AutocorrelationDetector, 440.)?;
    // plot_detector_for_freq(PowerCepstrum, 440.)?;
    plot_detector_for_freq(RawFftDetector, 440.)?;
    Ok(())
}
