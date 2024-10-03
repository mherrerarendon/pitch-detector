use super::NoteRenderer;
use crossterm::{
    cursor, execute, queue, style,
    terminal::{self, size, ClearType, ScrollUp, SetSize},
};
use pitch_detector::{
    core::constants::{MAX_CENTS_OFFSET, NUM_CENTS_BETWEEN_NOTES},
    note::NoteDetectionResult,
};
use std::io::Write as _;

struct TunerLayout {
    /// Position at which pitch is considered in tune but is technically still flat
    left_tick_pos: u16,

    /// Position at which pitch is considered in tune but is technically still sharp
    right_tick_pos: u16,

    note_name_pos: u16,
    cursor_pos: u16,
    previous_note_pos: u16,
    next_note_pos: u16,
    width: u16,
}

impl TunerLayout {
    fn new(note: &NoteDetectionResult, terminal_width: u16) -> Self {
        let note_name_pos = terminal_width / 2;
        let ticks_mark_percent =
            (NUM_CENTS_BETWEEN_NOTES - MAX_CENTS_OFFSET) / NUM_CENTS_BETWEEN_NOTES;
        let left_tick_pos = (note_name_pos as f64 * ticks_mark_percent).round() as u16;
        let right_tick_pos = note_name_pos + (note_name_pos - left_tick_pos);
        let cursor_pos_percent =
            (NUM_CENTS_BETWEEN_NOTES - note.cents_offset) / NUM_CENTS_BETWEEN_NOTES;
        let cursor_pos = (note_name_pos as f64 * cursor_pos_percent).round() as u16;
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

    fn build(&self, previous_note_name: &str, note_name: &str, next_note_name: &str) -> String {
        let mut s = String::with_capacity(self.width as usize);
        macro_rules! repeating_str {
            ($the_str:expr, $count:expr) => {{
                &std::iter::repeat($the_str)
                    .take($count as usize)
                    .collect::<String>()
            }};
        }
        s.push_str(previous_note_name);
        s.push_str(repeating_str!(
            " ",
            self.left_tick_pos - self.previous_note_pos
        ));
        s.push_str(repeating_str!(".", self.note_name_pos - self.left_tick_pos));
        s.push_str(note_name);
        s.push_str(repeating_str!(
            ".",
            self.right_tick_pos - self.note_name_pos
        ));
        s.push_str(repeating_str!(
            " ",
            self.next_note_pos - self.right_tick_pos
        ));
        s.push_str(next_note_name);
        s.replace_range(self.cursor_pos as usize..self.cursor_pos as usize + 1, "|");
        s
    }
}

pub struct CmdLineNoteRenderer {
    cols: u16,
    rows: u16,
    original_cols: Option<u16>,
    original_rows: Option<u16>,
}

impl CmdLineNoteRenderer {
    pub fn new_with_rows_and_columns(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            original_cols: None,
            original_rows: None,
        }
    }
}

impl NoteRenderer for CmdLineNoteRenderer {
    fn render_note(&mut self, note: NoteDetectionResult) -> anyhow::Result<()> {
        let tuner_layout = TunerLayout::new(&note, self.cols);
        let tuner_string = tuner_layout.build(
            &note.previous_note_name.to_string(),
            &note.note_name.to_string(),
            &note.next_note_name.to_string(),
        );
        let mut stdout = std::io::stdout().lock();
        queue!(
            stdout,
            terminal::Clear(ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            style::Print(tuner_string)
        )?;
        stdout.flush()?;
        Ok(())
    }

    fn render_no_note(&mut self) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout().lock();
        queue!(
            stdout,
            terminal::Clear(ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            style::Print("No note detected")
        )?;
        stdout.flush()?;
        Ok(())
    }

    fn input_start(&mut self) -> anyhow::Result<()> {
        let (cols, rows) = size()?;
        self.original_cols = Some(cols);
        self.original_rows = Some(rows);
        let mut stdout = std::io::stdout().lock();
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            SetSize(self.cols, self.rows)
        )?;
        Ok(())
    }

    fn input_end(&mut self) -> anyhow::Result<()> {
        let original_cols = self
            .original_cols
            .expect("input_start should be called before input_end");
        let original_rows = self
            .original_rows
            .expect("input_start should be called before input_end");
        execute!(std::io::stdout(), SetSize(original_cols, original_rows))?;
        Ok(())
    }
}
