pub fn sine_wave_signal(num_samples: usize, freq: f64, sample_rate: f64) -> Vec<f64> {
    (0..num_samples)
        .map(|r| (2.0 * std::f64::consts::PI * r as f64 * freq / sample_rate).sin())
        .collect()
}

pub fn audio_buffer_to_samples(byte_buffer: &[u8]) -> Vec<i16> {
    byte_buffer
        .chunks_exact(2)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .collect()
}
pub fn audio_buffer_to_signal(byte_buffer: &[u8]) -> Vec<f64> {
    audio_buffer_to_samples(byte_buffer)
        .into_iter()
        .map(|x| x as f64)
        .collect()
}
