// use smoothed_z_score::{PeaksDetector, PeaksFilter};

// use super::peak_iter::FftPeaks;
use crate::core::FrequencyBin;

pub(crate) trait PeakDetector {
    fn detect_peaks(&self, spectrum: &[f64]) -> Vec<FrequencyBin>;
}

// pub struct ZScoreDetector {
//     lag: usize,
//     threshold: f64,
//     influence: f64,
// }

// impl ZScoreDetector {
//     pub fn new(lag: usize, threshold: f64, influence: f64) -> Self {
//         Self {
//             lag,
//             threshold,
//             influence,
//         }
//     }
// }

// impl PeakDetector for ZScoreDetector {
//     fn detect_peaks(&self, spectrum: &[f64]) -> Vec<FftBin> {
//         spectrum
//             .into_iter()
//             .map(|x| *x)
//             .enumerate()
//             .rev()
//             .fft_peaks(self.lag, self.threshold, self.influence)
//             .collect()
//     }
// }

pub struct PeakFinderDetector {
    sigmas: f64,
}

impl PeakFinderDetector {
    pub fn new(sigmas: f64) -> Self {
        Self { sigmas }
    }
}

impl PeakDetector for PeakFinderDetector {
    fn detect_peaks(&self, spectrum: &[f64]) -> Vec<FrequencyBin> {
        peak_finder::peaks::find_peaks_over_stddev(&spectrum.to_vec(), self.sigmas)
            .into_iter()
            .map(|graph_peak| FrequencyBin {
                bin: graph_peak.peak.x as usize,
                magnitude: graph_peak.peak.y,
            })
            .collect()
    }
}
