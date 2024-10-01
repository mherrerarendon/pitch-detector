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
}

impl TunerLayout {
    fn new(note: &NoteDetectionResult, terminal_width: usize) -> Self {
        let actual_terminal_width = if terminal_width % 2 == 1 {
            terminal_width
        } else {
            terminal_width - 1
        };
        let note_name_pos = actual_terminal_width / 2;
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
            next_note_pos: actual_terminal_width - 1,
        }
    }
}

pub struct CmdLineNoteRenderer {
    width: usize,
}

impl CmdLineNoteRenderer {
    fn clear_terminal() {
        print!("{}[2J", 27 as char);
    }
}

impl NoteRenderer for CmdLineNoteRenderer {
    fn render_note(&mut self, note: NoteDetectionResult) -> anyhow::Result<()> {
        let mut tuner_str_format = String::new();

        // subtracting 2 to account for the previous and next note names
        // fill size will be used twice, once per side of the current note name
        let fill = self.width - 2;
        write!(
            tuner_str_format,
            "{}{:^width$}",
            note.previous_note_name,
            width = fill
        )?;
        let tuner_layout = TunerLayout::new(&note, self.width);
        let mut stdout = std::io::stdout().lock();
        write!(stdout, "")?;
        Ok(())
    }
}
