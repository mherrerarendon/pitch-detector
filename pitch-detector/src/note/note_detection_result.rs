use crate::core::{
    constants::{A4_FREQ, MAX_CENTS_OFFSET, MIN_FREQ, NOTES},
    error::PitchError,
    NoteName,
};

/// The resut of a pitch detection expressed as a note.
/// You will rarely need to instantiate this struct directly. Most commonly this will be returned from
/// [`detect_note`](crate::note::detect_note).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NoteDetection {
    /// The predominant frequency detected from a signal.
    pub actual_freq: f64,

    /// The note name of the detected note.
    pub note_name: NoteName,

    /// The expected frequency of the detected note.
    pub note_freq: f64,

    /// The octave of the detected note.
    pub octave: i32,

    /// The degree to which the detected not is in tune, expressed in cents. The absolute maximum `cents_offset` is
    /// 50, since anything larger than 50 would be considered the next or previous note.
    pub cents_offset: f64,

    /// The note name of the note that comes before the detected note. Not commonly used.
    pub previous_note_name: NoteName,

    /// The note name of the note that comes after the detected note. Not commonly used.
    pub next_note_name: NoteName,

    /// A `NoteDetectionResult` will be marked as `in_tune` if the `cents_offset` is less than
    /// [`MAX_CENTS_OFFSET`](crate::core::constants::MAX_CENTS_OFFSET).
    pub in_tune: bool,
}

impl TryFrom<f64> for NoteDetection {
    type Error = PitchError;
    fn try_from(freq: f64) -> Result<Self, Self::Error> {
        if freq < MIN_FREQ {
            return Err(PitchError::IncorrectParameters(format!(
                "Invalid frequency: {}",
                freq
            )));
        }
        let steps_from_a4 = (freq / A4_FREQ).log2() * 12.0;
        let note_freq = A4_FREQ * 2f64.powf(steps_from_a4.round() / 12.0);
        let steps_from_c5 = steps_from_a4 - 2.0;
        let cents_offset = (steps_from_a4 - steps_from_a4.round()) * 100.0;
        Ok(Self {
            actual_freq: freq,
            note_name: NOTES
                [(steps_from_a4.round() as isize).rem_euclid(NOTES.len() as isize) as usize]
                .into(),
            note_freq,
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use float_cmp::ApproxEq;
    fn test_pitch_from_f64(
        actual_freq: f64,
        note_name: NoteName,
        note_freq: f64,
        octave: i32,
        cents_offset: f64,
        previous_note_name: NoteName,
        next_note_name: NoteName,
        in_tune: bool,
    ) -> Result<()> {
        let pitch = NoteDetection::try_from(actual_freq)?;
        assert_eq!(
            pitch.note_name, note_name,
            "Expected note name {}, got {}",
            note_name, pitch.note_name
        );
        assert!(
            pitch.note_freq.approx_eq(note_freq, (0.1, 1)),
            "Expected note_freq: {}, actual note_freq: {}",
            note_freq,
            pitch.note_freq
        );
        assert_eq!(
            pitch.octave, octave,
            "Expected octave {}, got {}",
            octave, pitch.octave
        );
        assert!(
            pitch.cents_offset.approx_eq(cents_offset, (0.1, 1)),
            "Expected cents_offset: {}, actual cents_offset: {}",
            cents_offset,
            pitch.cents_offset
        );
        assert_eq!(
            pitch.previous_note_name, previous_note_name,
            "Expected previous note name {}, got {}",
            previous_note_name, pitch.previous_note_name
        );
        assert_eq!(
            pitch.next_note_name, next_note_name,
            "Expected next note name {}, got {}",
            next_note_name, pitch.next_note_name
        );
        assert_eq!(
            pitch.in_tune, in_tune,
            "Expected in tune {}, got {}",
            in_tune, pitch.in_tune
        );
        Ok(())
    }

    #[test]
    fn pitch_from_f64_works() -> Result<()> {
        test_pitch_from_f64(
            311.13,
            NoteName::DSharp,
            311.13,
            4,
            0.,
            NoteName::D,
            NoteName::E,
            true,
        )?;
        test_pitch_from_f64(
            329.63,
            NoteName::E,
            329.63,
            4,
            0.,
            NoteName::DSharp,
            NoteName::F,
            true,
        )?;
        test_pitch_from_f64(
            349.23,
            NoteName::F,
            349.23,
            4,
            0.,
            NoteName::E,
            NoteName::FSharp,
            true,
        )?;
        test_pitch_from_f64(
            369.99,
            NoteName::FSharp,
            369.99,
            4,
            0.,
            NoteName::F,
            NoteName::G,
            true,
        )?;
        test_pitch_from_f64(
            392.,
            NoteName::G,
            392.,
            4,
            0.,
            NoteName::FSharp,
            NoteName::GSharp,
            true,
        )?;
        test_pitch_from_f64(
            440.,
            NoteName::A,
            440.,
            4,
            0.,
            NoteName::GSharp,
            NoteName::ASharp,
            true,
        )?;
        test_pitch_from_f64(
            493.88,
            NoteName::B,
            493.88,
            4,
            0.,
            NoteName::ASharp,
            NoteName::C,
            true,
        )?;
        test_pitch_from_f64(
            523.25,
            NoteName::C,
            523.25,
            5,
            0.,
            NoteName::B,
            NoteName::CSharp,
            true,
        )?;
        test_pitch_from_f64(
            880.,
            NoteName::A,
            880.,
            5,
            0.,
            NoteName::GSharp,
            NoteName::ASharp,
            true,
        )?;
        test_pitch_from_f64(
            220.,
            NoteName::A,
            220.,
            3,
            0.,
            NoteName::GSharp,
            NoteName::ASharp,
            true,
        )?;
        test_pitch_from_f64(
            448., // Slighly sharp A
            NoteName::A,
            440.,
            4,
            31.194,
            NoteName::GSharp,
            NoteName::ASharp,
            false,
        )?;
        assert!(test_pitch_from_f64(
            0.,
            NoteName::A,
            0.,
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
