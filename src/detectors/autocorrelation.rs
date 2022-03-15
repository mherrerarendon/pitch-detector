use crate::{
    constants::{AUTOCORRELATION_ALGORITHM, MAX_FREQ, MIN_FREQ, SAMPLE_RATE},
    fft_space::FftSpace,
    FrequencyDetector, Partial,
};
use fitting::gaussian::fit;
use rustfft::FftPlanner;

struct AutocorrelationPeakIter<I: Iterator<Item = (usize, f64)>> {
    signal: I,
}

trait AutocorrelationPeaks<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    fn autocorrelation_peaks(self) -> AutocorrelationPeakIter<I>;
}

impl<I> AutocorrelationPeaks<I> for I
where
    I: Iterator<Item = (usize, f64)>,
{
    fn autocorrelation_peaks(self) -> AutocorrelationPeakIter<I> {
        AutocorrelationPeakIter { signal: self }
    }
}

impl<I> Iterator for AutocorrelationPeakIter<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    type Item = Partial;

    fn next(&mut self) -> Option<Self::Item> {
        let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = self
            .signal
            .by_ref()
            .skip_while(|(_, intensity)| *intensity <= 0.0)
            .take_while(|(_, intensity)| *intensity >= 0.0)
            .map(|(index, intensity)| (index as f64, intensity))
            .unzip();

        if x_vals.is_empty() {
            return None;
        }

        // mu, sigma, a
        if let Ok((mu, _, amplitude)) = fit(x_vals.into(), y_vals.into()) {
            Some(Partial {
                freq: SAMPLE_RATE / mu,
                intensity: amplitude,
            })
        } else {
            None
        }
    }
}

pub struct AutocorrelationDetector;

impl AutocorrelationDetector {
    fn spectrum(fft_space: &FftSpace) -> Box<dyn Iterator<Item = (usize, f64)> + '_> {
        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;
        Box::new(
            fft_space
                .space()
                .iter()
                .enumerate()
                .skip(lower_limit)
                .take(upper_limit - lower_limit)
                .map(|(idx, f)| (idx /*+ 1*/, f.re / fft_space.space()[0].re)),
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

        fft_space.map(|f| f * f.conj());
        let (space, scratch) = fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(space.len());
        inverse_fft.process_with_scratch(space, scratch);
    }
}

impl FrequencyDetector for AutocorrelationDetector {
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
            .skip_while(|(_, intensity)| *intensity > 0.001) // Skip the first slide
            .autocorrelation_peaks()
            .reduce(|accum, partial| {
                if partial.intensity > accum.intensity {
                    partial
                } else {
                    accum
                }
            })
            .map(|partial| partial.freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::RAW_FFT_ALGORITHM, tests::FrequencyDetectorTest, utils::test_utils::*};

    impl FrequencyDetectorTest for AutocorrelationDetector {
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
            AUTOCORRELATION_ALGORITHM
        }
    }
    #[test]
    fn test_autocorrelation() -> anyhow::Result<()> {
        let mut detector = AutocorrelationDetector;
        let mut fft_space = FftSpace::new(TEST_FFT_SPACE_SIZE);

        test_fundamental_freq(&mut detector, &mut fft_space, "tuner_c5.json", 529.841)?;
        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_a.json", 219.634)?;
        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_d.json", 146.717)?;
        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_g.json", 97.985)?;
        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_c.json", 64.535)?;
        Ok(())
    }
}
