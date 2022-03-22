use itertools::Itertools;
use num_traits::signum;

fn zero_crossing_count<I>(signal: I) -> usize
where
    I: IntoIterator<Item = f64>,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
{
    signal
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| (signum(a) - signum(b)).abs() as usize)
        .sum::<usize>()
        / 2
}
pub fn zero_crossing_rate<I>(signal: I, sample_rate: f64) -> f64
where
    I: IntoIterator<Item = f64>,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
{
    zero_crossing_count(signal) as f64 / sample_rate
}
#[cfg(test)]
mod tests {
    use std::fs;

    use crate::core::{test_utils::SampleData, utils::audio_buffer_to_signal};

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_name() -> Result<()> {
        Ok(())
    }

    fn test_zero_rate_crossing(filename: &str, expected_rate: usize) -> anyhow::Result<()> {
        let file_path = format!("{}/test_data/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let mut sample_data: SampleData = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
        let buffer = sample_data.data.take().unwrap();
        let signal = audio_buffer_to_signal(&buffer);
        let zero_crossings = zero_crossing_count(signal);
        assert_eq!(
            zero_crossings, expected_rate,
            "{} failed. Expected {} but got {}",
            filename, expected_rate, zero_crossings
        );
        Ok(())
    }

    #[test]
    fn zero_crossings() -> anyhow::Result<()> {
        test_zero_rate_crossing("tuner_c5.json", 370)?;
        test_zero_rate_crossing("cello_open_a.json", 258)?;
        test_zero_rate_crossing("cello_open_d.json", 262)?;
        test_zero_rate_crossing("cello_open_g.json", 162)?;
        test_zero_rate_crossing("cello_open_c.json", 158)?;
        test_zero_rate_crossing("noise.json", 103)?;

        Ok(())
    }
}
