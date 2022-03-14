mod constants;
mod detectors;
mod fft_space;
mod peak_iter;
mod utils;

use fft_space::FftSpace;

pub trait FrequencyDetector {
    fn detect_frequency<I: IntoIterator>(&mut self, signal: I) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    {
        let signal_iter = signal.into_iter();
        let mut fft_space = FftSpace::new(
            signal_iter
                .size_hint()
                .1
                .expect("Signal length is not known"),
        );
        self.detect_frequency_with_fft_space(signal_iter, &mut fft_space)
    }

    fn detect_frequency_with_fft_space<I: IntoIterator>(
        &mut self,
        signal: I,
        fft_space: &mut FftSpace,
    ) -> Option<f64>
    where
        <I as IntoIterator>::Item: std::borrow::Borrow<f64>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Partial {
    pub freq: f64,
    pub intensity: f64,
}

impl Default for Partial {
    fn default() -> Self {
        Self {
            freq: 0.0,
            intensity: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    pub(crate) trait FrequencyDetectorTest {
        fn spectrum<'a, I>(&self, signal: I) -> Box<dyn Iterator<Item = (usize, f64)> + 'a>
        where
            <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
            I: IntoIterator + 'a;
        fn name(&self) -> &'static str;
    }

    #[test]
    fn test_name() -> Result<()> {
        Ok(())
    }
}
