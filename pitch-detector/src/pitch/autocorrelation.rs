use super::PitchDetector;

pub struct Autocorrelation;

impl PitchDetector for Autocorrelation {
    fn detect_pitch_in_range(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range: std::ops::Range<f64>,
    ) -> Result<f64, crate::core::error::PitchError> {
        let n = signal.len();
        let mut r = vec![0.0; n];
        let mut d = vec![0.0; n];
        let mut d_normalized = vec![0.0; n];

        // Step 1: Autocorrelation
        for tau in 0..n {
            r[tau] = (0..n - tau).map(|i| signal[i] * signal[i + tau]).sum();
        }

        // Step 2: Difference function
        for tau in 1..n {
            d[tau] = r[0] - r[tau];
        }

        // Step 3: Cumulative Mean Normalized Difference
        let mut cumulative_mean = 0.0;
        for tau in 1..n {
            cumulative_mean += d[tau];
            d_normalized[tau] = d[tau] / (cumulative_mean / tau as f64);
        }

        // Step 4: Find the first minimum in the normalized difference function
        let mut min_tau = 1;
        for tau in 2..n {
            if d_normalized[tau] < d_normalized[min_tau] {
                min_tau = tau;
            }
        }

        // Avoid divisions by zero and ensure it's a plausible pitch
        if min_tau == 0 {
            return Err(crate::core::error::PitchError::UnexpectedError(
                "First minimum was invalid".to_string(),
            ));
        }

        // Step 5: Calculate the fundamental frequency
        let fundamental_freq = sample_rate / min_tau as f64;
        Ok(fundamental_freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{test_fundamental_freq, test_sine_wave};

    #[test]
    fn it_detects_cello_pitches() -> anyhow::Result<()> {
        let mut detector = Autocorrelation;

        test_fundamental_freq(&mut detector, "cello_open_a.json", 220.)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.666)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.345)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.516)?;
        Ok(())
    }

    #[test]
    fn it_detects_sine_wave_pitch() -> anyhow::Result<()> {
        let mut detector = Autocorrelation;
        test_sine_wave(&mut detector, 441.)?;
        Ok(())
    }
}
