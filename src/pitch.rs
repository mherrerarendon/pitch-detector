use crate::core::constants::{A4_FREQ, MAX_CENTS_OFFSET, MIN_FREQ, NOTES};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pitch {
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
