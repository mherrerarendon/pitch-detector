use crate::{constants::*, fft_space::FftSpace, peak_iter::FftPeaks, FrequencyDetector, Partial};
use rustfft::FftPlanner;
use std::borrow::Borrow;

pub struct RawFftDetector;

impl RawFftDetector {
    fn spectrum(fft_space: &FftSpace) -> Vec<(usize, f64)> {
        let lower_limit = (MIN_FREQ * fft_space.len() as f64 / SAMPLE_RATE).round() as usize;
        let upper_limit = (MAX_FREQ * fft_space.len() as f64 / SAMPLE_RATE).round() as usize;
        fft_space
            .freq_domain(true)
            .enumerate()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .map(|(i, (amplitude, _))| (i, amplitude))
            .collect()
    }

    fn process_fft<I: IntoIterator>(signal: I, fft_space: &mut FftSpace)
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_space.len());
        let signal_iter = signal.into_iter();
        let signal_size = signal_iter
            .size_hint()
            .1
            .expect("Signal length is not known");
        fft_space.init_fft_space(
            signal_iter
                .zip(apodize::hanning_iter(signal_size))
                .map(|(x, y)| x.borrow() * y),
        );

        let (space, scratch) = fft_space.workspace();
        fft.process_with_scratch(space, scratch);
    }
}

impl FrequencyDetector for RawFftDetector {
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
            .fft_peaks(40, 10.)
            .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
            .map(|item| Partial {
                freq: item.0 * SAMPLE_RATE / fft_space.len() as f64,
                intensity: item.1,
            })
            .map(|partial| partial.freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::FrequencyDetectorTest, utils::test_utils::*};

    impl FrequencyDetectorTest for RawFftDetector {
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
            Self::spectrum(&fft_space)
        }

        fn name(&self) -> &'static str {
            RAW_FFT_ALGORITHM
        }
    }

    #[test]
    fn test_raw_fft() -> anyhow::Result<()> {
        let mut detector = RawFftDetector;
        let mut fft_space = FftSpace::new(TEST_FFT_SPACE_SIZE);

        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_a.json", 219.383)?;

        // Fails to detect open d, which should be at around 146 Hz
        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_d.json", 293.390)?;

        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_g.json", 97.209)?;

        // Fails to detect open C, which should be around 64 Hz
        test_fundamental_freq(&mut detector, &mut fft_space, "cello_open_c.json", 129.046)?;
        Ok(())
    }
}
