use std::ops::Range;

/// This trait provides the necessary methods to analyze the frequency make-up of a signal. Note that the bin values of
/// the resulting frequency domain might not always correspond to the traditional output of FFT, which is why additional
/// methods like `bin_to_freq` and `freq_to_bin` are required.
pub trait IntoFrequencyDomain {
    /// Creates a frequency domain Vec with the given signal and optional frequency range. If frequency range is provided,
    /// the second parameter to the tuple is the sample rate, which is needed to correlate the frequency range to
    /// a bin range.
    fn into_frequency_domain(
        &mut self,
        signal: &[f64],
        freq_range: Option<(Range<f64>, f64)>,
    ) -> (usize, Vec<f64>);

    /// Translates frequency bin in frequency domain to frequency in hertz
    fn bin_to_freq(&self, bin: f64, sample_rate: f64) -> f64;

    /// Translates frequency in hertz to frequency bin in frequency domain
    fn freq_to_bin(&self, freq: f64, sample_rate: f64) -> f64;

    fn name(&self) -> &'static str;
}
