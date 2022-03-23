use std::borrow::Borrow;

use itertools::Itertools;
use num_traits::signum;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FftPoint {
    pub x: f64,
    pub y: f64,
}

impl Default for FftPoint {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

fn zero_crossing_count<I>(signal: I) -> usize
where
    I: IntoIterator,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
{
    signal
        .into_iter()
        .map(|x| *x.borrow())
        .tuple_windows()
        .map(|(a, b)| (signum(a) - signum(b)).abs() as usize)
        .sum::<usize>()
        / 2
}
pub fn zero_crossing_rate<I>(signal: I, sample_rate: f64) -> f64
where
    I: IntoIterator,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
{
    let signal_iter = signal.into_iter();
    let signal_len = signal_iter.size_hint().1.expect("signal length not found");
    sample_rate / signal_len as f64 * zero_crossing_count(signal_iter) as f64
}
#[cfg(test)]
mod tests {
    use std::fs;

    use float_cmp::ApproxEq;

    use crate::core::{
        test_utils::SampleData,
        utils::{audio_buffer_to_signal, sine_wave_signal},
    };

    use super::*;

    fn test_zero_rate_count(filename: &str, expected_rate: usize) -> anyhow::Result<()> {
        let file_path = format!("{}/test_data/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let mut sample_data: SampleData = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
        let buffer = sample_data.data.take().unwrap();
        let signal = audio_buffer_to_signal(&buffer);
        let zero_crossings = zero_crossing_count(signal.iter());
        assert_eq!(
            zero_crossings, expected_rate,
            "{} failed. Expected {} but got {}",
            filename, expected_rate, zero_crossings
        );
        Ok(())
    }

    fn test_file_to_signal(filename: &str) -> anyhow::Result<Vec<f64>> {
        let file_path = format!("{}/test_data/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let mut sample_data: SampleData = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
        let buffer = sample_data.data.take().unwrap();
        Ok(audio_buffer_to_signal(&buffer))
    }

    fn test_zero_rate_crossing(
        signal: &[f64],
        sample_rate: f64,
        expected_rate: f64,
    ) -> anyhow::Result<()> {
        let zero_crossing_rate = zero_crossing_rate(signal.iter(), sample_rate);
        assert!(
            zero_crossing_rate.approx_eq(expected_rate, (0.2, 2)),
            "Expected {} but got {}",
            expected_rate,
            zero_crossing_rate
        );
        Ok(())
    }

    #[test]
    fn zero_crossings() -> anyhow::Result<()> {
        test_zero_rate_count("tuner_c5.json", 370)?;
        test_zero_rate_count("cello_open_a.json", 258)?;
        test_zero_rate_count("cello_open_d.json", 262)?;
        test_zero_rate_count("cello_open_g.json", 162)?;
        test_zero_rate_count("cello_open_c.json", 158)?;
        test_zero_rate_count("noise.json", 103)?;

        Ok(())
    }

    #[test]
    fn zero_crossing_rates() -> anyhow::Result<()> {
        const TEST_SAMPLE_RATE: f64 = 44000.0;
        test_zero_rate_crossing(
            &test_file_to_signal("tuner_c5.json")?,
            TEST_SAMPLE_RATE,
            925.,
        )?;
        test_zero_rate_crossing(
            &test_file_to_signal("cello_open_a.json")?,
            TEST_SAMPLE_RATE,
            645.,
        )?;
        test_zero_rate_crossing(
            &test_file_to_signal("cello_open_d.json")?,
            TEST_SAMPLE_RATE,
            655.,
        )?;
        test_zero_rate_crossing(
            &test_file_to_signal("cello_open_g.json")?,
            TEST_SAMPLE_RATE,
            405.,
        )?;
        test_zero_rate_crossing(
            &test_file_to_signal("cello_open_c.json")?,
            TEST_SAMPLE_RATE,
            395.,
        )?;
        test_zero_rate_crossing(
            &test_file_to_signal("noise.json")?,
            TEST_SAMPLE_RATE,
            257.719,
        )?;

        Ok(())
    }

    #[test]
    fn zero_rate_crossing_sine() -> anyhow::Result<()> {
        const NUM_SAMPLES: usize = 16384;
        test_zero_rate_crossing(
            &sine_wave_signal(NUM_SAMPLES, 440., 44100.),
            44100.,
            877.478,
        )?;
        test_zero_rate_crossing(&sine_wave_signal(10000, 440., 44100.), 44100., 877.478)?;

        Ok(())
    }
}
