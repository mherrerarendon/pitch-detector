use crate::core::{
    constants::{MAX_FREQ, MIN_FREQ},
    fft_space::FftSpace,
    peak_iter::FftPeaks,
};
use rustfft::FftPlanner;
use std::borrow::Borrow;

use super::{FftPoint, FrequencyDetector};

pub struct RawFftDetector;

impl RawFftDetector {
    fn spectrum(
        fft_space: &FftSpace,
        sample_rate: f64,
    ) -> Box<dyn Iterator<Item = (usize, f64)> + '_> {
        let lower_limit = (MIN_FREQ * fft_space.len() as f64 / sample_rate).round() as usize;
        let upper_limit = (MAX_FREQ * fft_space.len() as f64 / sample_rate).round() as usize;
        Box::new(
            fft_space
                .freq_domain(true)
                .enumerate()
                .skip(lower_limit)
                .take(upper_limit - lower_limit)
                .map(|(i, (amplitude, _))| (i, amplitude)),
        )
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

    fn detect_unscaled_freq<I: IntoIterator>(
        signal: I,
        sample_rate: f64,
        fft_space: &mut FftSpace,
    ) -> Option<FftPoint>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        Self::process_fft(signal, fft_space);
        Self::spectrum(fft_space, sample_rate)
            .into_iter()
            .fft_peaks(40, 10.)
            .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
            .map(|item| FftPoint {
                x: item.0,
                y: item.1,
            })
    }
}

impl FrequencyDetector for RawFftDetector {
    fn detect_frequency_with_fft_space<I: IntoIterator>(
        &mut self,
        signal: I,
        sample_rate: f64,
        fft_space: &mut FftSpace,
    ) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        Self::detect_unscaled_freq(signal, sample_rate, fft_space)
            .map(|point| point.x * sample_rate / fft_space.len() as f64)
    }
}

#[cfg(feature = "test_utils")]
mod test_utils {
    use crate::{
        core::{constants::test_utils::RAW_FFT_ALGORITHM, fft_space::FftSpace},
        frequency::{FftPoint, FrequencyDetectorTest},
    };

    use super::RawFftDetector;

    impl FrequencyDetectorTest for RawFftDetector {
        fn spectrum<'a, I>(&self, signal: I, sample_rate: f64) -> Vec<(usize, f64)>
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
            Self::spectrum(&fft_space, sample_rate).collect()
        }

        fn detect_unscaled_freq_with_space<I: IntoIterator>(
            &mut self,
            signal: I,
            sample_rate: f64,
            fft_space: &mut FftSpace,
        ) -> Option<FftPoint>
        where
            <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
        {
            Self::detect_unscaled_freq(signal, sample_rate, fft_space)
        }

        fn name(&self) -> &'static str {
            RAW_FFT_ALGORITHM
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{test_fundamental_freq, test_sine_wave};

    #[test]
    fn test_raw_fft() -> anyhow::Result<()> {
        let mut detector = RawFftDetector;

        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.383)?;

        // Fails to detect open d, which should be at around 146 Hz
        test_fundamental_freq(&mut detector, "cello_open_d.json", 293.390)?;

        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.209)?;

        // Fails to detect open C, which should be around 64 Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 129.046)?;
        Ok(())
    }

    #[test]
    fn test_raw_fft_sine() -> anyhow::Result<()> {
        let mut detector = RawFftDetector;
        test_sine_wave(&mut detector, 440.)?;
        Ok(())
    }
}
