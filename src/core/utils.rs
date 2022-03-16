pub fn sine_wave_signal(num_samples: usize, freq: f64, sample_rate: f64) -> Vec<f64> {
    (0..num_samples)
        .map(|r| (2.0 * std::f64::consts::PI * r as f64 * freq / sample_rate).sin())
        .collect()
}
