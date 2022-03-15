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
    use crate::{fft_space::FftSpace, tests::FrequencyDetectorTest, FrequencyDetector};
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

    pub fn test_fundamental_freq<D: FrequencyDetector>(
        detector: &mut D,
        fft_space: &mut FftSpace,
        samples_file: &str,
        expected_freq: f64,
    ) -> anyhow::Result<()> {
        let signal = test_signal(samples_file)?;
        let fft_space_size = calc_optimized_fft_space_size(signal.len());

        // Sanity check
        assert_eq!(fft_space_size, TEST_FFT_SPACE_SIZE);

        let freq = detector
            .detect_frequency_with_fft_space(&signal, fft_space)
            .ok_or(anyhow::anyhow!("Did not get pitch"))?;

        assert!(
            freq.approx_eq(expected_freq, (0.02, 2)),
            "Expected freq: {}, Actual freq: {}",
            expected_freq,
            freq
        );
        Ok(())
    }
}
