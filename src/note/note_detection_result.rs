use crate::core::{
    constants::{A4_FREQ, MAX_CENTS_OFFSET, MIN_FREQ, NOTES},
    NoteName,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NoteDetectionResult {
    pub freq: f64,
    pub note_name: NoteName,
    pub octave: i32,
    pub cents_offset: f64,
    pub previous_note_name: NoteName,
    pub next_note_name: NoteName,
    pub in_tune: bool,
}

impl TryFrom<f64> for NoteDetectionResult {
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
            note_name: NOTES
                [(steps_from_a4.round() as isize).rem_euclid(NOTES.len() as isize) as usize]
                .into(),
            octave: (5. + (steps_from_c5 / 12.0).floor()) as i32,
            cents_offset,
            previous_note_name: NOTES
                [(steps_from_a4.round() as isize - 1).rem_euclid(NOTES.len() as isize) as usize]
                .into(),
            next_note_name: NOTES
                [(steps_from_a4.round() as isize + 1).rem_euclid(NOTES.len() as isize) as usize]
                .into(),
            in_tune: cents_offset.abs() < MAX_CENTS_OFFSET,
        })
    }
}

impl TryFrom<usize> for NoteDetectionResult {
    type Error = anyhow::Error;
    fn try_from(bin: usize) -> Result<Self, Self::Error> {
        (bin as f64).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use float_cmp::ApproxEq;
    fn test_pitch_from_f64(
        freq: f64,
        note_name: NoteName,
        octave: i32,
        cents_offset: f64,
        previous_note_name: NoteName,
        next_note_name: NoteName,
        in_tune: bool,
    ) -> Result<()> {
        let pitch = NoteDetectionResult::try_from(freq)?;
        assert_eq!(
            pitch.note_name, note_name,
            "Expected note name {}, got {}",
            note_name, pitch.note_name
        );
        assert_eq!(pitch.octave, octave);
        assert!(
            pitch.cents_offset.approx_eq(cents_offset, (0.1, 1)),
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
        test_pitch_from_f64(
            311.13,
            NoteName::DSharp,
            4,
            0.,
            NoteName::D,
            NoteName::E,
            true,
        )?;
        test_pitch_from_f64(
            329.63,
            NoteName::E,
            4,
            0.,
            NoteName::DSharp,
            NoteName::F,
            true,
        )?;
        test_pitch_from_f64(
            349.23,
            NoteName::F,
            4,
            0.,
            NoteName::E,
            NoteName::FSharp,
            true,
        )?;
        test_pitch_from_f64(
            369.99,
            NoteName::FSharp,
            4,
            0.,
            NoteName::F,
            NoteName::G,
            true,
        )?;
        test_pitch_from_f64(
            392.,
            NoteName::G,
            4,
            0.,
            NoteName::FSharp,
            NoteName::GSharp,
            true,
        )?;
        test_pitch_from_f64(
            440.,
            NoteName::A,
            4,
            0.,
            NoteName::GSharp,
            NoteName::ASharp,
            true,
        )?;
        test_pitch_from_f64(
            493.88,
            NoteName::B,
            4,
            0.,
            NoteName::ASharp,
            NoteName::C,
            true,
        )?;
        test_pitch_from_f64(
            523.25,
            NoteName::C,
            5,
            0.,
            NoteName::B,
            NoteName::CSharp,
            true,
        )?;
        test_pitch_from_f64(
            880.,
            NoteName::A,
            5,
            0.,
            NoteName::GSharp,
            NoteName::ASharp,
            true,
        )?;
        test_pitch_from_f64(
            220.,
            NoteName::A,
            3,
            0.,
            NoteName::GSharp,
            NoteName::ASharp,
            true,
        )?;
        assert!(test_pitch_from_f64(
            0.,
            NoteName::A,
            0,
            0.,
            NoteName::GSharp,
            NoteName::ASharp,
            true
        )
        .is_err());
        Ok(())
    }
}
