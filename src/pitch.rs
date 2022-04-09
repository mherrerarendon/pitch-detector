// pub mod autocorrelation;
pub mod cepstrum;
pub mod core;
pub mod raw_fft;

use std::ops::Range;

use crate::core::{fft_space::FftSpace, utils::interpolated_peak_at};

use self::core::FftPoint;

pub trait PitchDetector: SignalToSpectrum {
    fn detect(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range_hint: Option<Range<f64>>,
    ) -> Option<f64> {
        let (start_bin, spectrum) =
            self.signal_to_spectrum(signal, freq_range_hint.map(|r| (r, sample_rate)));
        let max_bin =
            spectrum.iter().enumerate().reduce(
                |accum, item| {
                    if item.1 > accum.1 {
                        item
                    } else {
                        accum
                    }
                },
            )?;

        let FftPoint { x: bin, .. } = interpolated_peak_at(&spectrum, max_bin.0)?;
        Some(self.bin_to_freq(bin + start_bin as f64, sample_rate))
    }
}

pub trait SignalToSpectrum {
    fn signal_to_spectrum(
        &mut self,
        signal: &[f64],
        freq_range: Option<(Range<f64>, f64)>,
    ) -> (usize, Vec<f64>);

    // Bin may be float resolution
    fn bin_to_freq(&self, bin: f64, sample_rate: f64) -> f64;
    fn freq_to_bin(&self, freq: f64, sample_rate: f64) -> f64;

    fn name(&self) -> &'static str;
    // fn relevant_bin_range(&self, fft_space_len: usize, sample_rate: f64) -> (usize, usize);

    // fn detect_max_bin(&mut self, signal: &[f64], bin_range: (usize, usize)) -> Option<FftPoint> {
    //     let mut fft_space = FftSpace::new(signal.len());
    //     fft_space.init_with_signal(signal);
    //     self.detect_max_bin_with_fft_space(bin_range, &mut fft_space)
    // }

    // fn detect_max_bin_with_fft_space(
    //     &mut self,
    //     bin_range: (usize, usize),
    //     fft_space: &mut FftSpace,
    // ) -> Option<FftPoint>;
}
