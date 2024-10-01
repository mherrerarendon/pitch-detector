use super::NoteRenderer;
use pitch_detector::{
    core::constants::{MAX_CENTS_OFFSET, NUM_CENTS_BETWEEN_NOTES},
    note::NoteDetectionResult,
};
use std::fmt::Write as _;
use std::io::Write as _;

struct TunerLayout {
    /// Position at which pitch is considered in tune but is technically still flat
    left_tick_pos: usize,

    /// Position at which pitch is considered in tune but is technically still sharp
    right_tick_pos: usize,

    note_name_pos: usize,
    cursor_pos: usize,
    previous_note_pos: usize,
    next_note_pos: usize,
    width: usize,
}

impl TunerLayout {
    fn new(note: &NoteDetectionResult, terminal_width: usize) -> Self {
        let note_name_pos = terminal_width / 2;
        let ticks_mark_percent =
            (NUM_CENTS_BETWEEN_NOTES - MAX_CENTS_OFFSET) / NUM_CENTS_BETWEEN_NOTES;
        let left_tick_pos = (note_name_pos as f64 * ticks_mark_percent).round() as usize;
        let right_tick_pos = note_name_pos + (note_name_pos - left_tick_pos);
        let cursor_pos_percent =
            (NUM_CENTS_BETWEEN_NOTES - note.cents_offset) / NUM_CENTS_BETWEEN_NOTES;
        let cursor_pos = (note_name_pos as f64 * cursor_pos_percent).round() as usize;
        TunerLayout {
            left_tick_pos,
            right_tick_pos,
            note_name_pos,
            cursor_pos,
            previous_note_pos: 0,
            next_note_pos: terminal_width - 1,
            width: terminal_width,
        }
    }

    fn build(&self, note: &NoteDetectionResult) -> String {
        let mut s = String::with_capacity(self.width);
        macro_rules! repeating_str {
            ($the_str:expr, $count:expr) => {{
                &std::iter::repeat($the_str).take($count).collect::<String>()
            }};
        }
        s.push_str(&note.previous_note_name.to_string());
        s.push_str(repeating_str!(" ",));
        s.push_str(std::iter::repeat(".").take(self.note_name_pos - self.left_tick_pos));
        s.push_str(&note.note_name.to_string());
        s.push_str(std::iter::repeat(".").take(self.right_tick_pos - self.note_name_pos));
        s.push_str(std::iter::repeat(" ").take(self.next_note_pos - self.right_tick_pos));
        s.push_str(&note.next_note_name.to_string());
        s
    }
}

pub struct CmdLineNoteRenderer {
    width: usize,
}

impl NoteRenderer for CmdLineNoteRenderer {
    fn render_note(&mut self, note: NoteDetectionResult) -> anyhow::Result<()> {
        let mut tuner_str_format = std::iter::repeat(" ").take(self.width).collect::<String>();
        let tuner_layout = TunerLayout::new(&note, self.width);
        tuner_str_format.replace_range(
            tuner_layout.previous_note_pos..tuner_layout.previous_note_pos,
            &note.previous_note_name.to_string(),
        );
        tuner_str_format.replace_range(
            tuner_layout.left_tick_pos..tuner_layout.right_tick_pos,
            &note.previous_note_name.to_string(),
        );
        tuner_str_format.replace_range(
            tuner_layout.previous_note_pos..tuner_layout.previous_note_pos,
            &note.previous_note_name.to_string(),
        );
        tuner_str_format.replace_range(
            tuner_layout.previous_note_pos..tuner_layout.previous_note_pos,
            &note.previous_note_name.to_string(),
        );
        tuner_str_format.replace_range(
            tuner_layout.previous_note_pos..tuner_layout.previous_note_pos,
            &note.previous_note_name.to_string(),
        );
        tuner_str_format.replace_range(
            tuner_layout.previous_note_pos..tuner_layout.previous_note_pos,
            &note.previous_note_name.to_string(),
        );
        let mut stdout = std::io::stdout().lock();
        write!(stdout, "{}[2J", 27 as char)?;
        write!(stdout, "")?;
        Ok(())
    }
}
