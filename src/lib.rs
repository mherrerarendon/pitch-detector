mod constants;
mod detectors;
mod fft_space;
mod peak_iter;
mod utils;

pub trait FrequencyDetector {
    fn detect_frequency<I: IntoIterator>(&mut self, signal: I) -> Option<f64>
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
        fn spectrum(&self) -> Vec<(usize, f64)>;

        fn name(&self) -> &'static str;
    }

    #[test]
    fn test_name() -> Result<()> {
        Ok(())
    }
}
