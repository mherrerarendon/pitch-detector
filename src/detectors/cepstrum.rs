use crate::{
    core::{
        constants::{MAX_FREQ, MIN_FREQ, SAMPLE_RATE},
        fft_space::FftSpace,
        peak_iter::FftPeaks,
    },
    FrequencyDetector, Partial,
};
use rustfft::{num_complex::Complex, FftPlanner};

pub struct PowerCepstrum;
impl PowerCepstrum {
    fn spectrum(fft_space: &FftSpace) -> Box<dyn Iterator<Item = (usize, f64)> + '_> {
        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;

        Box::new(
            fft_space
                .freq_domain(false)
                .map(|(amplitude, _)| amplitude)
                .enumerate()
                .skip(lower_limit)
                .take(upper_limit - lower_limit),
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
}
impl FrequencyDetector for PowerCepstrum {
    fn detect_frequency_with_fft_space<I: IntoIterator>(
        &mut self,
        signal: I,
        fft_space: &mut FftSpace,
    ) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        Self::process_fft(signal, fft_space);
        Self::spectrum(fft_space)
            .into_iter()
            .fft_peaks(60, 10.)
            .reduce(|accum, quefrency| {
                if quefrency.1 > accum.1 {
                    quefrency
                } else {
                    accum
                }
            })
            .map(|(mu, amplitude)| Partial {
                freq: SAMPLE_RATE / mu,
                intensity: amplitude,
            })
            .map(|partial| partial.freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{constants::tests::POWER_CEPSTRUM_ALGORITHM, test_utils::test_fundamental_freq},
        FrequencyDetectorTest,
    };

    impl FrequencyDetectorTest for PowerCepstrum {
        fn spectrum<'a, I>(&self, signal: I) -> Vec<(usize, f64)>
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
            Self::spectrum(&fft_space).collect()
        }

        fn name(&self) -> &'static str {
            POWER_CEPSTRUM_ALGORITHM
        }
    }

    #[test]
    fn test_power() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum;

        // Power cepstrum fails to detect the C5 note, which should be at around 523Hz
        test_fundamental_freq(&mut detector, "tuner_c5.json", 261.591)?;

        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.418)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.730)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.214)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.454)?;
        Ok(())
    }
}
