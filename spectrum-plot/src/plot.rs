use std::ops::Range;

use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries},
    style::{IntoFont, RED, WHITE},
};

use pitch_detector::{core::into_frequency_domain::ToFrequencyDomain, pitch::PitchDetector};

pub fn plot_spectrum<D>(
    detector: &mut D,
    signal: &[f64],
    freq_range: Range<f64>,
    sample_rate: f64,
    algorithm_name: &str,
    plot_name: &str,
) -> anyhow::Result<()>
where
    D: PitchDetector + ToFrequencyDomain,
{
    let max_freq = detector.detect_pitch_in_range(signal, sample_rate, freq_range.clone())?;
    let max_bin = detector.freq_to_bin(max_freq, sample_rate);
    let plot_title = format!(
        "{} - {} - {:.2} max bin",
        algorithm_name, plot_name, max_bin
    );
    let output_file = format!(
        "{}/test_data/results/{} - {}.png",
        env!("CARGO_MANIFEST_DIR"),
        algorithm_name,
        plot_name
    );

    let (start_bin, y_vals) = detector.to_frequency_domain(signal, Some((freq_range, sample_rate)));
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
        .build_cartesian_2d(x_vals[0]..x_vals[x_vals.len() - 1], y_min..y_max)?;

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
