// Rust implementation of Yin pitch detection algorithm
use ndarray::Array1;

use crate::core::error::PitchError;

use super::PitchDetector;

/// Function to compute the Yin difference function.
fn difference_function(signal: &[f64], max_lag: usize) -> Array1<f64> {
    let mut diff = Array1::zeros(max_lag);

    for tau in 1..max_lag {
        let mut sum = 0.0;
        for i in 0..(signal.len() - tau) {
            let delta = signal[i] - signal[i + tau];
            sum += delta * delta;
        }
        diff[tau] = sum;
    }

    diff
}

/// Cumulative mean normalized difference function.
fn cumulative_mean_normalized_difference(diff: Array1<f64>) -> Array1<f64> {
    let mut cmnd = Array1::zeros(diff.len());
    cmnd[0] = 1.0; // first value is typically set to 1.0 to avoid division by 0

    let mut running_sum = 0.0;
    for tau in 1..diff.len() {
        running_sum += diff[tau];
        cmnd[tau] = diff[tau] / (running_sum / tau as f64);
    }

    cmnd
}

/// Finds the pitch period (tau) given the cumulative mean normalized difference function.
fn find_pitch_period(cmnd: &Array1<f64>, threshold: f64) -> Result<usize, PitchError> {
    for tau in 1..cmnd.len() {
        if cmnd[tau] < threshold {
            // Check if it's a minimum compared to the neighboring values
            if tau + 1 < cmnd.len() && cmnd[tau] < cmnd[tau + 1] {
                return Ok(tau);
            }
        }
    }
    Err(PitchError::NoPitchDetected(
        "Did not find a pitch within threshol".to_string(),
    ))
}

/// Main Yin pitch detection function
pub fn yin_pitch(
    signal: &[f64],
    sample_rate: f64,
    threshold: f64,
    max_lag: usize,
) -> Result<f64, PitchError> {
    // Step 1: Compute the difference function
    let diff = difference_function(signal, max_lag);

    // Step 2: Compute the cumulative mean normalized difference function
    let cmnd = cumulative_mean_normalized_difference(diff);

    // Step 3: Find the pitch period (tau) that meets the threshold
    if let Ok(tau) = find_pitch_period(&cmnd, threshold) {
        // Convert tau to frequency (pitch)
        let pitch = sample_rate / tau as f64;
        return Ok(pitch);
    }

    Err(PitchError::NoPitchDetected("No pitch detected".to_string()))
}

// fn main() {
//     // Example usage of the Yin algorithm
//     let sample_rate = 44100;
//     let signal = vec![/* Your audio signal here as a Vec<f64> */];

//     // Maximum lag corresponds to the lowest frequency we want to detect (typically around 80Hz)
//     let max_lag = sample_rate / 80;

//     // The threshold for pitch detection; a typical value is around 0.1
//     let threshold = 0.1;

//     // Run the Yin algorithm to get the detected pitch
//     if let Some(pitch) = yin_pitch(&signal, sample_rate, threshold, max_lag) {
//         println!("Detected pitch: {:.2} Hz", pitch);
//     } else {
//         println!("No pitch detected.");
//     }
// }

pub struct Autocorrelation2 {
    /// Typical value is around 0.1
    threshold: f64,
}

impl Autocorrelation2 {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }
}

impl PitchDetector for Autocorrelation2 {
    fn detect_pitch_in_range(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range: std::ops::Range<f64>,
    ) -> Result<f64, crate::core::error::PitchError> {
        // Maximum lag corresponds to the lowest frequency we want to detect (typically around 80Hz)
        let max_lag = (sample_rate / freq_range.start).round() as usize;
        yin_pitch(signal, sample_rate, self.threshold, max_lag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{test_freq, test_sine_wave};

    test_freq! {tuner_c5: {
        detector: Autocorrelation2::new(0.1),
        file: "tuner_c5.wav",
        expected_freq: 47.675 // Autocorrelation gets the tuner (sine wave-like) pitch detection wrong
    }}
    test_freq! {cello_open_a: {
        detector: Autocorrelation2::new(0.1),
        file: "cello_open_a.wav",
        expected_freq: 220.5
    }}
    test_freq! {cello_open_d: {
        detector: Autocorrelation2::new(0.1),
        file: "cello_open_d.wav",
        expected_freq: 147.
    }}
    test_freq! {cello_open_g: {
        detector: Autocorrelation2::new(0.1),
        file: "cello_open_g.wav",
        expected_freq: 97.566
    }}
    test_freq! {cello_open_c: {
        detector: Autocorrelation2::new(0.1),
        file: "cello_open_c.wav",
        expected_freq: 64.662
    }}

    #[test]
    fn it_detects_sine_wave_pitch() -> anyhow::Result<()> {
        let mut detector = Autocorrelation2::new(0.1);
        test_sine_wave(&mut detector, 441.)?;
        Ok(())
    }
}
