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

pub struct AutocorrelationDetector {
    fft_space: FftSpace,
}

impl FrequencyDetector for AutocorrelationDetector {
    fn detect_frequency<I: IntoIterator>(&mut self, signal: I) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);

        self.fft_space.map(|f| f * f.conj());
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);

        self.spectrum()
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

    fn spectrum(&self) -> Vec<(usize, f64)> {
        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;
        self.fft_space
            .space()
            .iter()
            .enumerate()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .map(|(idx, f)| (idx /*+ 1*/, f.re / self.fft_space.space()[0].re))
            .collect()
    }

    #[cfg(test)]
    fn name(&self) -> &'static str {
        AUTOCORRELATION_ALGORITHM
    }
}

impl AutocorrelationDetector {
    pub fn new(fft_space_size: usize) -> Self {
        AutocorrelationDetector {
            fft_space: FftSpace::new(fft_space_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_autocorrelation() -> anyhow::Result<()> {
        let mut detector = AutocorrelationDetector::new(TEST_FFT_SPACE_SIZE);

        test_fundamental_freq(&mut detector, "tuner_c5.json", 529.841)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.634)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.717)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.985)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.535)?;
        Ok(())
    }
}
