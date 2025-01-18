use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

use crate::core::error::PitchError;

use super::PitchDetector;

/// Perform FFT on the signal
fn fft(signal: &[f64]) -> Vec<Complex<f64>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(signal.len());

    // Convert signal to complex format
    let mut input: Vec<Complex<f64>> = signal.iter().map(|&x| Complex::new(x, 0.0)).collect();

    fft.process(&mut input);
    input
}

/// Perform IFFT to get back to the time domain
fn ifft(spectrum: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut planner = FftPlanner::new();
    let ifft = planner.plan_fft_inverse(spectrum.len());

    let mut input = spectrum.to_vec();

    ifft.process(&mut input);
    input
}

/// Compute the power spectrum (magnitude of FFT values squared)
fn power_spectrum(spectrum: &[Complex<f64>]) -> Vec<f64> {
    spectrum.iter().map(|x| x.norm_sqr()).collect()
}

/// Compute the logarithm of the power spectrum
fn log_spectrum(power_spec: &[f64]) -> Vec<f64> {
    power_spec.iter().map(|&x| (x + 1e-10).ln()).collect() // Add a small value to avoid log(0)
}

/// Cepstrum computation for pitch detection
pub fn cepstrum_pitch(
    signal: &[f64],
    sample_rate: f64,
    freq_range: std::ops::Range<f64>,
) -> Result<f64, PitchError> {
    // Step 1: Compute FFT of the signal
    let spectrum = fft(signal);

    // Step 2: Compute power spectrum
    let power_spec = power_spectrum(&spectrum);

    // Step 3: Take the logarithm of the power spectrum
    let log_spec = log_spectrum(&power_spec);

    // Step 4: Compute IFFT of the log-spectrum to get the cepstrum
    let log_spec_complex: Vec<Complex<f64>> =
        log_spec.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let cepstrum = ifft(&log_spec_complex);

    // Step 5: Find the peak in the cepstrum corresponding to the pitch period
    // let start_index = sample_rate / 500; // Skip low quefrency values (corresponds to very high frequencies)
    // let end_index = sample_rate / 60; // Skip high quefrency values (corresponds to very low frequencies)
    let start_index = (sample_rate / freq_range.end).round() as usize; // Skip low quefrency values (corresponds to very high frequencies)
    let end_index = (sample_rate / freq_range.start).round() as usize; // Skip high quefrency values (corresponds to very low frequencies)

    let mut max_quefrency_index = start_index;
    let mut max_value = cepstrum[start_index].norm_sqr(); // Using squared magnitude

    for i in start_index..end_index {
        let value = cepstrum[i].norm_sqr();
        if value > max_value {
            max_value = value;
            max_quefrency_index = i;
        }
    }

    // Step 6: Calculate pitch from the quefrency peak
    let quefrency = max_quefrency_index as f64 / sample_rate as f64;
    if quefrency > 0.0 {
        Ok(1.0 / quefrency)
    } else {
        Err(PitchError::NoPitchDetected("No pitch detected".to_string()))
    }
}

// fn main() {
//     // Example usage
//     let sample_rate = 44100;
//     let signal = vec![/* Your audio signal here as a Vec<f64> */];

//     // Detect pitch using the Cepstrum method
//     if let Some(pitch) = cepstrum_pitch(&signal, sample_rate) {
//         println!("Detected pitch: {:.2} Hz", pitch);
//     } else {
//         println!("No pitch detected.");
//     }
// }

pub struct Cepstrum2;

impl PitchDetector for Cepstrum2 {
    fn detect_pitch_in_range(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range: std::ops::Range<f64>,
    ) -> Result<f64, PitchError> {
        cepstrum_pitch(&signal, sample_rate, freq_range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::test_fundamental_freq;

    #[test]
    fn test_power() -> anyhow::Result<()> {
        let mut detector = Cepstrum2;

        test_fundamental_freq(&mut detector, "cello_open_a.wav", 218.905)?;
        test_fundamental_freq(&mut detector, "cello_open_d.wav", 146.666)?;
        test_fundamental_freq(&mut detector, "cello_open_g.wav", 97.130)?;
        test_fundamental_freq(&mut detector, "cello_open_c.wav", 64.421)?;
        Ok(())
    }

    // Power cepstrum doesn't work with sine waves since it looks for a harmonic sequence.
    // #[test]
    // fn test_raw_fft_sine() -> anyhow::Result<()> {
    //     let mut detector = PowerCepstrum;
    //     test_sine_wave(&mut detector, 440.)?;
    //     Ok(())
    // }
}
