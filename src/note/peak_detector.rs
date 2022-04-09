use smoothed_z_score::{PeaksDetector, PeaksFilter};

use crate::core::FftBin;

pub(crate) trait PeakDetector {
    fn detect_peaks(&self, spectrum: &[f64]) -> Vec<FftBin>;
}

pub struct ZScoreDetector {
    lag: usize,
    threshold: f64,
    influence: f64,
}

impl ZScoreDetector {
    pub fn new(lag: usize, threshold: f64, influence: f64) -> Self {
        Self {
            lag,
            threshold,
            influence,
        }
    }
}

impl PeakDetector for ZScoreDetector {
    fn detect_peaks(&self, spectrum: &[f64]) -> Vec<FftBin> {
        spectrum
            .into_iter()
            .enumerate()
            .peaks(
                PeaksDetector::new(self.lag, self.threshold, self.influence),
                |e| *e.1,
            )
            .map(|((bin, mag), _)| FftBin {
                bin,
                magnitude: *mag,
            })
            .collect()
    }
}
