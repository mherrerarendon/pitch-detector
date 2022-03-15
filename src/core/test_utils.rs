pub fn audio_buffer_to_samples(byte_buffer: &[u8]) -> Vec<i16> {
    byte_buffer
        .chunks_exact(2)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .collect()
}
pub(crate) fn audio_buffer_to_signal(byte_buffer: &[u8]) -> Vec<f64> {
    audio_buffer_to_samples(byte_buffer)
        .into_iter()
        .map(|x| x as f64)
        .collect()
}
use float_cmp::ApproxEq;

use crate::FrequencyDetector;
use serde::Deserialize;
use std::fs;

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
    samples_file: &str,
    expected_freq: f64,
) -> anyhow::Result<()> {
    let signal = test_signal(samples_file)?;

    let freq = detector
        .detect_frequency(&signal)
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;

    assert!(
        freq.approx_eq(expected_freq, (0.02, 2)),
        "Expected freq: {}, Actual freq: {}",
        expected_freq,
        freq
    );
    Ok(())
}
