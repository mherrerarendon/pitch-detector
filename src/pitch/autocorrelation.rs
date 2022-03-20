use crate::{
    core::constants::{MAX_FREQ, MIN_FREQ},
    core::{fft_space::FftSpace, utils::interpolated_peak_at},
};
use rustfft::FftPlanner;

use super::{FftPoint, PitchDetector};

pub struct AutocorrelationDetector;

impl AutocorrelationDetector {
    fn relevant_fft_range(sample_rate: f64) -> (usize, usize) {
        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (sample_rate / MAX_FREQ).round() as usize;
        let upper_limit = (sample_rate / MIN_FREQ).round() as usize;
        (lower_limit, upper_limit)
    }
    fn unscaled_spectrum(
        fft_space: &FftSpace,
        fft_range: (usize, usize),
    ) -> Box<dyn Iterator<Item = f64> + '_> {
        let (lower_limit, upper_limit) = fft_range;
        Box::new(
            fft_space
                .space()
                .iter()
                .skip(lower_limit)
                .take(upper_limit - lower_limit)
                .map(|f| f.re / fft_space.space()[0].re),
        )
    }

    fn process_fft(fft_space: &mut FftSpace) {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(fft_space.padded_len());

        let (space, scratch) = fft_space.workspace();
        forward_fft.process_with_scratch(space, scratch);

        fft_space.map(|f| f * f.conj());
        let (space, scratch) = fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(space.len());
        inverse_fft.process_with_scratch(space, scratch);
    }

    fn detect_unscaled_freq(
        fft_range: (usize, usize),
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint> {
        Self::process_fft(fft_space);
        let unscaled_spectrum: Vec<f64> = Self::unscaled_spectrum(fft_space, fft_range).collect();
        let fft_point = unscaled_spectrum
            .iter()
            .enumerate()
            .reduce(|accum, quefrency| {
                if quefrency.1 > accum.1 {
                    quefrency
                } else {
                    accum
                }
            })?;
        interpolated_peak_at(&unscaled_spectrum, fft_point.0)
    }
}

impl PitchDetector for AutocorrelationDetector {
    fn detect_with_fft_space(&mut self, sample_rate: f64, fft_space: &mut FftSpace) -> Option<f64> {
        let (lower_limit, upper_limit) = Self::relevant_fft_range(sample_rate);
        Self::detect_unscaled_freq((lower_limit, upper_limit), fft_space)
            .map(|point| sample_rate / (lower_limit as f64 + point.x))
    }
}

#[cfg(feature = "test_utils")]
mod test_utils {
    use crate::{
        core::{constants::test_utils::AUTOCORRELATION_ALGORITHM, fft_space::FftSpace},
        pitch::{FftPoint, FrequencyDetectorTest},
    };

    use super::AutocorrelationDetector;

    impl FrequencyDetectorTest for AutocorrelationDetector {
        fn unscaled_spectrum(&self, signal: &[f64], fft_range: (usize, usize)) -> Vec<f64> {
            let mut fft_space = FftSpace::new(signal.len());
            fft_space.init_with_signal(signal);
            Self::process_fft(&mut fft_space);
            Self::unscaled_spectrum(&fft_space, fft_range).collect()
        }

        fn relevant_fft_range(&self, _fft_space_len: usize, sample_rate: f64) -> (usize, usize) {
            Self::relevant_fft_range(sample_rate)
        }

        fn detect_unscaled_freq_with_space(
            &mut self,
            fft_range: (usize, usize),
            fft_space: &mut FftSpace,
        ) -> Option<FftPoint> {
            Self::detect_unscaled_freq(fft_range, fft_space)
        }

        fn name(&self) -> &'static str {
            AUTOCORRELATION_ALGORITHM
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{test_fundamental_freq, test_sine_wave};

    #[test]
    fn test_autocorrelation() -> anyhow::Result<()> {
        let mut detector = AutocorrelationDetector;

        test_fundamental_freq(&mut detector, "tuner_c5.json", 529.841)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.634)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.717)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.985)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.535)?;
        Ok(())
    }

    #[test]
    fn test_autocorrelation_sine() -> anyhow::Result<()> {
        let mut detector = AutocorrelationDetector;
        test_sine_wave(&mut detector, 440.)?;
        Ok(())
    }
}
