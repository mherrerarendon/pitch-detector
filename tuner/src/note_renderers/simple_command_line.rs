use std::io::Write;

use crossterm::{
    cursor, queue, style,
    terminal::{self, ClearType},
};
use pitch_detector::{core::error::PitchError, note::NoteDetection};

use super::NoteRenderer;

pub struct SimpleCommandLineRenderer;

impl NoteRenderer for SimpleCommandLineRenderer {
    fn render_note(&self, note: NoteDetection) -> anyhow::Result<()> {
        let note_line = format!("Note: {}", note.note_name);
        let cents_line = format!("cents_offset: {}", note.cents_offset);
        let mut stdout = std::io::stdout().lock();
        queue!(
            stdout,
            cursor::MoveToPreviousLine(1),
            terminal::Clear(ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            style::Print(note_line),
            cursor::MoveToNextLine(1),
            terminal::Clear(ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            style::Print(cents_line)
        )?;
        stdout.flush()?;
        Ok(())
    }

    fn render_no_note(&self, error: PitchError) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout().lock();
        queue!(
            stdout,
            cursor::MoveToPreviousLine(2),
            terminal::Clear(ClearType::CurrentLine),
            // cursor::MoveToColumn(0),
            // style::Print(""),
            cursor::MoveToNextLine(1),
            terminal::Clear(ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            style::Print(error.to_string())
        )?;
        stdout.flush()?;
        Ok(())
    }
}
