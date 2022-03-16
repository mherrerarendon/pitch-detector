use freq_detector::{
    core::test_utils::test_signal,
    frequency::{
        autocorrelation::AutocorrelationDetector, FrequencyDetector, FrequencyDetectorTest,
    },
};
use plotters::prelude::*;

fn plot<D: FrequencyDetector + FrequencyDetectorTest>(
    detector: &D,
    samples_file: &str,
    expected_freq: f64,
) -> anyhow::Result<()> {
    let plot_title = format!(
        "{} - {} - {:?} Hz",
        detector.name(),
        samples_file,
        expected_freq
    );
    let output_file = format!(
        "{}/test_data/results/{}.png",
        env!("CARGO_MANIFEST_DIR"),
        format!("{} - {}", detector.name(), samples_file)
    );
    let signal = test_signal(samples_file)?;
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
fn main() -> anyhow::Result<()> {
    let mut detector = AutocorrelationDetector;
    plot(&mut detector, "tuner_c5.json", 523.)?;
    Ok(())
}
