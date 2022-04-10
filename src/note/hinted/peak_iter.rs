use smoothed_z_score::{Peak, PeaksDetector, PeaksFilter};

use crate::core::FftBin;

pub(crate) struct PeakIter<I: Iterator<Item = (usize, f64)>> {
    signal: I,
    lag: usize,
    threshold: f64,
    influence: f64,
}

pub(crate) trait FftPeaks<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    fn fft_peaks(self, lag: usize, threshold: f64, influence: f64) -> PeakIter<I>;
}

impl<I> FftPeaks<I> for I
where
    I: Iterator<Item = (usize, f64)>,
{
    fn fft_peaks(self, lag: usize, threshold: f64, influence: f64) -> PeakIter<I> {
        PeakIter {
            signal: self,
            lag,
            threshold,
            influence,
        }
    }
}

impl<I> Iterator for PeakIter<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    type Item = FftBin;

    fn next(&mut self) -> Option<Self::Item> {
        let fft_bins: Vec<FftBin> = self
            .signal
            .by_ref()
            .peaks(PeaksDetector::new(self.lag, self.threshold, 0.0), |e| e.1)
            .skip_while(|(_, peak)| *peak == Peak::None)
            .take_while(|(_, peak)| *peak == Peak::High)
            .map(|(bin, _)| FftBin {
                bin: bin.0,
                magnitude: bin.1,
            })
            .collect();
        fft_bins
            .iter()
            .reduce(|accum, item| {
                if item.magnitude > accum.magnitude {
                    item
                } else {
                    accum
                }
            })
            .map(|fft_bin| fft_bin.clone())
    }
}
