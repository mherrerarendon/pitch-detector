use crate::{
    core::{
        constants::{A4_FREQ, MAX_CENTS_OFFSET, MIN_FREQ, NOTES},
        fft_space::FftSpace,
    },
    frequency::FrequencyDetector,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pitch {
    pub freq: f64,
    pub note_name: String,
    pub octave: i32,
    pub cents_offset: f64,
    pub previous_note_name: String,
    pub next_note_name: String,
    pub in_tune: bool,
}

impl TryFrom<f64> for Pitch {
    type Error = anyhow::Error;
    fn try_from(freq: f64) -> Result<Self, Self::Error> {
        if freq < MIN_FREQ {
            return Err(anyhow::anyhow!("Invalid frequency: {}", freq));
        }
        let steps_from_a4 = (freq / A4_FREQ).log2() * 12.0;
        let steps_from_c5 = steps_from_a4 - 2.0;
        let cents_offset = (steps_from_a4 - steps_from_a4.round()) * 100.0;
        Ok(Self {
            freq,
            note_name: NOTES[(steps_from_a4.round() as usize) % NOTES.len()].into(),
            octave: (5. + (steps_from_c5 / 12.0).floor()) as i32,
            cents_offset,
            previous_note_name: NOTES
                [(steps_from_a4.round() as isize - 1).rem_euclid(NOTES.len() as isize) as usize]
                .into(),
            next_note_name: NOTES[(steps_from_a4.round() as usize + 1) % NOTES.len()].into(),
            in_tune: cents_offset.abs() < MAX_CENTS_OFFSET,
        })
    }
}

pub fn detect_frequency<I, D>(signal: I, freq_detector: &mut D, sample_rate: f64) -> Option<Pitch>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    D: FrequencyDetector,
{
    freq_detector
        .detect_frequency(signal, sample_rate)
        .and_then(|f| f.try_into().ok())
}

pub fn detect_frequency_with_fft_space<I, D>(
    signal: I,
    freq_detector: &mut D,
    sample_rate: f64,
    fft_space: &mut FftSpace,
) -> Option<Pitch>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
    D: FrequencyDetector,
{
    freq_detector
        .detect_frequency_with_fft_space(signal, sample_rate, fft_space)
        .and_then(|f| f.try_into().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use float_cmp::ApproxEq;
    fn test_pitch_from_f64(
        freq: f64,
        note_name: &str,
        octave: i32,
        cents_offset: f64,
        previous_note_name: &str,
        next_note_name: &str,
        in_tune: bool,
    ) -> Result<()> {
        let pitch = Pitch::try_from(freq)?;
        assert_eq!(pitch.note_name, note_name);
        assert_eq!(pitch.octave, octave);
        assert!(
            pitch.cents_offset.approx_eq(cents_offset, (0.02, 2)),
            "Expected cents_offset: {}, actual cents_offset: {}",
            cents_offset,
            pitch.cents_offset
        );
        assert_eq!(pitch.previous_note_name, previous_note_name);
        assert_eq!(pitch.next_note_name, next_note_name);
        assert_eq!(pitch.in_tune, in_tune);
        Ok(())
    }

    #[test]
    fn pitch_from_f64_works() -> Result<()> {
        test_pitch_from_f64(440., "A", 4, 0., "G#", "A#", true)?;
        test_pitch_from_f64(493.88, "B", 4, 0., "A#", "C", true)?;
        test_pitch_from_f64(523.25, "C", 5, 0., "B", "C#", true)?;
        test_pitch_from_f64(880., "A", 5, 0., "G#", "A#", true)?;
        test_pitch_from_f64(220., "A", 3, 0., "G#", "A#", true)?;
        assert!(test_pitch_from_f64(0., "A", 0, 0., "G#", "A#", true).is_err());
        Ok(())
    }
}
