use crate::core::{
    constants::{MAX_FREQ, MIN_FREQ, MIN_ZERO_CROSSING_RATE},
    fft_space::FftSpace,
    utils::interpolated_peak_at,
};
use rustfft::FftPlanner;

use super::{core::zero_crossing_rate, core::FftPoint, PitchDetector};

pub struct RawFftDetector;

impl RawFftDetector {
    fn unscaled_spectrum(
        fft_space: &FftSpace,
        fft_range: (usize, usize),
    ) -> Box<dyn Iterator<Item = f64> + '_> {
        let (lower_limit, upper_limit) = fft_range;
        Box::new(
            fft_space
                .freq_domain(true)
                .skip(lower_limit)
                .take(upper_limit - lower_limit)
                .map(|(amplitude, _)| amplitude),
        )
    }

    fn relevant_fft_range(fft_space_len: usize, sample_rate: f64) -> (usize, usize) {
        let lower_limit = (MIN_FREQ * fft_space_len as f64 / sample_rate).round() as usize;
        let upper_limit = (MAX_FREQ * fft_space_len as f64 / sample_rate).round() as usize;
        (lower_limit, upper_limit)
    }

    fn process_fft(fft_space: &mut FftSpace) {
        let mut planner = FftPlanner::new();
        let fft_len = fft_space.padded_len();
        let signal_len = fft_space.signal_len();
        let fft = planner.plan_fft_forward(fft_len);

        let (space, scratch) = fft_space.workspace();
        let hanning = apodize::hanning_iter(signal_len);
        space.iter_mut().zip(hanning).for_each(|(s, h)| s.re *= h);
        fft.process_with_scratch(space, scratch);
    }

    fn detect_unscaled_freq(
        fft_range: (usize, usize),
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint> {
        Self::process_fft(fft_space);
        let unscaled_spectrum: Vec<f64> = Self::unscaled_spectrum(fft_space, fft_range).collect();
        let fft_point = unscaled_spectrum.iter().enumerate().reduce(|accum, item| {
            if item.1 > accum.1 {
                item
            } else {
                accum
            }
        })?;
        interpolated_peak_at(&unscaled_spectrum, fft_point.0)
    }
}

impl PitchDetector for RawFftDetector {
    fn detect_with_fft_space(&mut self, sample_rate: f64, fft_space: &mut FftSpace) -> Option<f64> {
        let rate = zero_crossing_rate(fft_space.signal(), sample_rate);
        if rate < MIN_ZERO_CROSSING_RATE || rate > MAX_FREQ * 2. {
            return None;
        }
        let (lower_limit, upper_limit) =
            Self::relevant_fft_range(fft_space.padded_len(), sample_rate);
        Self::detect_unscaled_freq((lower_limit, upper_limit), fft_space).map(|point| {
            (lower_limit as f64 + point.x) * sample_rate / fft_space.padded_len() as f64
        })
    }
}

#[cfg(feature = "test_utils")]
mod test_utils {
    use crate::{
        core::{constants::test_utils::RAW_FFT_ALGORITHM, fft_space::FftSpace},
        pitch::{core::FftPoint, FrequencyDetectorTest},
    };

    use super::RawFftDetector;

    impl FrequencyDetectorTest for RawFftDetector {
        fn unscaled_spectrum(&self, signal: &[f64], fft_range: (usize, usize)) -> Vec<f64> {
            let mut fft_space = FftSpace::new(signal.len());
            fft_space.init_with_signal(signal);
            Self::process_fft(&mut fft_space);
            Self::unscaled_spectrum(&fft_space, fft_range).collect()
        }

        fn relevant_fft_range(&self, fft_space_len: usize, sample_rate: f64) -> (usize, usize) {
            Self::relevant_fft_range(fft_space_len, sample_rate)
        }

        fn detect_unscaled_freq_with_space(
            &mut self,
            fft_range: (usize, usize),
            fft_space: &mut FftSpace,
        ) -> Option<FftPoint> {
            Self::detect_unscaled_freq(fft_range, fft_space)
        }

        fn name(&self) -> &'static str {
            RAW_FFT_ALGORITHM
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{test_fundamental_freq, test_signal, test_sine_wave};

    #[test]
    fn test_raw_fft() -> anyhow::Result<()> {
        let mut detector = RawFftDetector;

        test_fundamental_freq(&mut detector, "tuner_c5.json", 523.242)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.383)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.732)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.209)?;

        // Fails to detect open C, which should be around 64 Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 129.046)?;
        Ok(())
    }

    #[test]
    fn test_noise() -> anyhow::Result<()> {
        pub const TEST_SAMPLE_RATE: f64 = 44000.0;
        let signal = test_signal("noise.json")?;

        let mut detector = RawFftDetector;
        assert!(detector.detect(&signal, TEST_SAMPLE_RATE).is_none());

        Ok(())
    }

    #[test]
    fn test_raw_fft_sine() -> anyhow::Result<()> {
        let mut detector = RawFftDetector;
        test_sine_wave(&mut detector, 440.)?;
        Ok(())
    }
}
