pub const MAX_FREQ: f64 = 2093.0; // C7
pub const MIN_FREQ: f64 = 32.7; // C1
pub const MIN_ZERO_CROSSING_RATE: f64 = 0.003;
pub const A4_FREQ: f64 = 440.0;
pub const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

// Noticable pitch difference starts at around 10-25 cents
pub const MAX_CENTS_OFFSET: f64 = 10.0;

#[cfg(feature = "test_utils")]
pub(crate) mod test_utils {
    pub const RAW_FFT_ALGORITHM: &str = "rawfft";
    pub const POWER_CEPSTRUM_ALGORITHM: &str = "power";
    pub const AUTOCORRELATION_ALGORITHM: &str = "autocorrelation";
}
