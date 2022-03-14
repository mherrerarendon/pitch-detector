pub fn audio_buffer_to_samples(byte_buffer: &[u8]) -> Vec<i16> {
    byte_buffer
        .chunks_exact(2)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .collect()
}

pub fn calc_optimized_fft_space_size(num_samples: usize) -> usize {
    let mut optimized_sum_samples = (2usize).pow(10);
    loop {
        if optimized_sum_samples < num_samples {
            optimized_sum_samples *= 2;
        } else {
            break optimized_sum_samples;
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    pub(crate) fn audio_buffer_to_signal(byte_buffer: &[u8]) -> Vec<f64> {
        audio_buffer_to_samples(byte_buffer)
            .into_iter()
            .map(|x| x as f64)
            .collect()
    }
    use float_cmp::ApproxEq;

    use super::*;
    use crate::{tests::FrequencyDetectorTest, FrequencyDetector};
    use plotters::prelude::*;
    use serde::Deserialize;
    use std::fs;

    pub(crate) const TEST_FFT_SPACE_SIZE: usize = 32768;
    const PLOT: bool = false;

    #[derive(Deserialize)]
    pub struct SampleData {
        pub data: Option<Vec<u8>>,
    }

    pub fn test_signal(filename: &str) -> anyhow::Result<Vec<f64>> {
        let file_path = format!("{}/test_data/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let mut sample_data: SampleData = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
        let buffer = sample_data.data.take().unwrap();
        Ok(audio_buffer_to_signal(&buffer))
    }

    fn plot(
        spectrum: &[(usize, f64)],
        detector_name: &str,
        samples_file: &str,
        expected_freq: f64,
    ) -> anyhow::Result<()> {
        let plot_title = format!(
            "{} - {} - {:?} Hz",
            detector_name, samples_file, expected_freq
        );
        let output_file = format!(
            "{}/test_data/results/{}.png",
            env!("CARGO_MANIFEST_DIR"),
            format!("{} - {}", detector_name, samples_file)
        );
        let (x_vals, y_vals): (Vec<f64>, Vec<f64>) =
            spectrum.iter().map(|i| (i.0 as f64, i.1)).unzip();
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

    pub fn test_fundamental_freq<D: FrequencyDetector + FrequencyDetectorTest>(
        detector: &mut D,
        samples_file: &str,
        expected_freq: f64,
    ) -> anyhow::Result<()> {
        println!(
            "{} - {} - {:?} Hz",
            detector.name(),
            samples_file,
            expected_freq
        );
        let signal = test_signal(samples_file)?;
        let fft_space_size = calc_optimized_fft_space_size(signal.len());

        // Sanity check
        assert_eq!(fft_space_size, TEST_FFT_SPACE_SIZE);

        let freq = detector
            .detect_frequency(&signal)
            .ok_or(anyhow::anyhow!("Did not get pitch"))?;

        if PLOT {
            let spectrum = detector.spectrum();
            let detector_name = detector.name();
            plot(&spectrum, detector_name, samples_file, expected_freq)?;
        }

        assert!(
            freq.approx_eq(expected_freq, (0.02, 2)),
            "Expected freq: {}, Actual freq: {}",
            expected_freq,
            freq
        );
        Ok(())
    }
}
