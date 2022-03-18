use crate::{
    core::constants::{MAX_FREQ, MIN_FREQ},
    core::fft_space::FftSpace,
};
use fitting::gaussian::fit;
use rustfft::FftPlanner;

use super::{FftPoint, FrequencyDetector};

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
    type Item = FftPoint;

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
            Some(FftPoint {
                x: mu,
                y: amplitude,
            })
        } else {
            None
        }
    }
}

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
    fn spectrum(
        fft_space: &FftSpace,
        fft_range: (usize, usize),
    ) -> Box<dyn Iterator<Item = (usize, f64)> + '_> {
        let (lower_limit, upper_limit) = fft_range;
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

    fn detect_unscaled_freq<I: IntoIterator>(
        signal: I,
        fft_range: (usize, usize),
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        Self::process_fft(signal, fft_space);
        Self::spectrum(fft_space, fft_range)
            .into_iter()
            .skip_while(|(_, intensity)| *intensity > 0.001) // Skip the first slide down
            .autocorrelation_peaks()
            .reduce(|accum, point| if point.y > accum.y { point } else { accum })
    }
}

impl FrequencyDetector for AutocorrelationDetector {
    fn detect_frequency_with_fft_space<I: IntoIterator>(
        &mut self,
        signal: I,
        sample_rate: f64,
        fft_space: &mut FftSpace,
    ) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let fft_range = Self::relevant_fft_range(sample_rate);
        Self::detect_unscaled_freq(signal, fft_range, fft_space)
            .map(|partial| sample_rate / partial.x)
    }
}

#[cfg(feature = "test_utils")]
mod test_utils {
    use crate::{
        core::{constants::test_utils::AUTOCORRELATION_ALGORITHM, fft_space::FftSpace},
        frequency::{FftPoint, FrequencyDetectorTest},
    };

    use super::AutocorrelationDetector;

    impl FrequencyDetectorTest for AutocorrelationDetector {
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
            Self::spectrum(&fft_space, fft_range).map(|f| f.1).collect()
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
