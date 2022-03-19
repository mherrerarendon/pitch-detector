use crate::core::{
    constants::{MAX_FREQ, MIN_FREQ},
    fft_space::FftSpace,
    utils::interpolated_peak_at,
};
use rustfft::{num_complex::Complex, FftPlanner};

use super::{FftPoint, PitchDetector};

pub struct PowerCepstrum;
impl PowerCepstrum {
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
                .freq_domain(false)
                .skip(lower_limit)
                .take(upper_limit - lower_limit)
                .map(|(amplitude, _)| amplitude),
        )
    }

    fn process_fft<I: IntoIterator>(signal: I, fft_space: &mut FftSpace)
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(fft_space.len());
        fft_space.init_fft_space(signal);

        let (space, scratch) = fft_space.workspace();
        forward_fft.process_with_scratch(space, scratch);
        fft_space.map(|f| Complex::new(f.norm_sqr().log(std::f64::consts::E), 0.0));
        let (space, scratch) = fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(space.len());
        inverse_fft.process_with_scratch(space, scratch);
    }

    fn detect_unscaled_freq<I: IntoIterator>(
        signal: I,
        fft_range: (usize, usize),
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        Self::process_fft(signal, fft_space);
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

impl PitchDetector for PowerCepstrum {
    fn detect_with_fft_space<I: IntoIterator>(
        &mut self,
        signal: I,
        sample_rate: f64,
        fft_space: &mut FftSpace,
    ) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let (lower_limit, upper_limit) = Self::relevant_fft_range(sample_rate);
        Self::detect_unscaled_freq(signal, (lower_limit, upper_limit), fft_space)
            .map(|point| sample_rate / (lower_limit as f64 + point.x))
    }
}

#[cfg(feature = "test_utils")]
mod test_utils {
    use crate::{
        core::{constants::test_utils::POWER_CEPSTRUM_ALGORITHM, fft_space::FftSpace},
        pitch::{FftPoint, FrequencyDetectorTest},
    };

    use super::PowerCepstrum;

    impl FrequencyDetectorTest for PowerCepstrum {
        fn unscaled_spectrum<'a, I>(&self, signal: I, fft_range: (usize, usize)) -> Vec<f64>
        where
            <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
            I: IntoIterator + 'a,
        {
            let signal_iter = signal.into_iter();
            let mut fft_space = FftSpace::new(
                signal_iter
                    .size_hint()
                    .1
                    .expect("Signal length is not known"),
            );
            Self::process_fft(signal_iter, &mut fft_space);
            Self::unscaled_spectrum(&fft_space, fft_range).collect()
        }

        fn relevant_fft_range(&self, _fft_space_len: usize, sample_rate: f64) -> (usize, usize) {
            Self::relevant_fft_range(sample_rate)
        }

        fn detect_unscaled_freq_with_space<I: IntoIterator>(
            &mut self,
            signal: I,
            fft_range: (usize, usize),
            fft_space: &mut FftSpace,
        ) -> Option<FftPoint>
        where
            <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
        {
            Self::detect_unscaled_freq(signal, fft_range, fft_space)
        }

        fn name(&self) -> &'static str {
            POWER_CEPSTRUM_ALGORITHM
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::test_fundamental_freq;

    #[test]
    fn test_power() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum;

        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.418)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.730)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.214)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.476)?;
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
