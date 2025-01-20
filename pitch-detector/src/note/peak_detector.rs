use crate::core::FrequencyBin;

pub(crate) trait PeakDetector {
    fn detect_peaks(&self, spectrum: &[f64]) -> Vec<FrequencyBin>;
}

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
        peak_finder::peaks::find_peaks_over_stddev(spectrum, self.sigmas)
            .into_iter()
            .map(|graph_peak| FrequencyBin {
                bin: graph_peak.peak.x,
                magnitude: graph_peak.peak.y,
            })
            .collect()
    }
}
