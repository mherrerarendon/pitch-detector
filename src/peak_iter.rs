use fitting::gaussian::fit;
use smoothed_z_score::{Peak, PeaksDetector, PeaksFilter};

pub(crate) struct PeakIter<I: Iterator<Item = (usize, f64)>> {
    signal: I,
    lag: usize,
    threshold: f64,
}

pub(crate) trait FftPeaks<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    fn fft_peaks(self, lag: usize, threshold: f64) -> PeakIter<I>;
}

impl<I> FftPeaks<I> for I
where
    I: Iterator<Item = (usize, f64)>,
{
    fn fft_peaks(self, lag: usize, threshold: f64) -> PeakIter<I> {
        PeakIter {
            signal: self,
            lag,
            threshold,
        }
    }
}

impl<I> Iterator for PeakIter<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    // mu, sigma, a
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = self
            .signal
            .by_ref()
            .peaks(PeaksDetector::new(self.lag, self.threshold, 0.0), |e| e.1)
            .skip_while(|(_, peak)| *peak == Peak::None)
            .take_while(|(_, peak)| *peak == Peak::High)
            .map(|(quefrency, _)| (quefrency.0 as f64, quefrency.1))
            .unzip();

        // Useful for debugging
        // println!("{:?}", x_vals);
        match x_vals.len() {
            0 => None,
            1 => Some((x_vals[0], y_vals[0])),
            2 => {
                if y_vals[0] > y_vals[1] {
                    Some((x_vals[0], y_vals[0]))
                } else {
                    Some((x_vals[1], y_vals[1]))
                }
            }
            _ => {
                if let Ok((mu, _, amplitude)) = fit(x_vals.into(), y_vals.into()) {
                    Some((mu, amplitude))
                } else {
                    assert!(false, "should not get here");
                    None
                }
            }
        }
    }
}
