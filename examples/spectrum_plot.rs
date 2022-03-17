use freq_detector::{
    core::test_utils::test_signal,
    frequency::{
        autocorrelation::AutocorrelationDetector, cepstrum::PowerCepstrum, raw_fft::RawFftDetector,
        FrequencyDetector, FrequencyDetectorTest,
    },
};
use plotters::prelude::*;

fn plot<D, I>(detector: &D, signal: I, plot_name: &str, fft_x: f64) -> anyhow::Result<()>
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
    let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = detector
        .spectrum(signal, 44000.)
        .iter()
        .map(|i| (i.0 as f64, i.1))
        .unzip();
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
    const TEST_FILE_SAMPLE_RATE: f64 = 44000.;
    for test_file in test_files {
        let test_signal = test_signal(test_file)?;
        let fft_point = detector
            .detect_unscaled_freq(&test_signal, TEST_FILE_SAMPLE_RATE)
            .ok_or(anyhow::anyhow!(""))?;
        plot(&detector, test_signal, test_file, fft_point.x)?;
    }
    Ok(())
}
fn main() -> anyhow::Result<()> {
    let test_files = [
        "cello_open_a.json",
        "cello_open_c.json",
        "cello_open_g.json",
        "cello_open_c.json",
        "tuner_c5.json",
        // "noise.json",
    ];
    plot_detector_for_files(AutocorrelationDetector, &test_files)?;
    plot_detector_for_files(PowerCepstrum, &test_files)?;
    plot_detector_for_files(RawFftDetector, &test_files)?;
    Ok(())
}
