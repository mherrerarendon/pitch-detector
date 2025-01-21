pub mod cmd_line;
pub mod simple_command_line;

use pitch_detector::{core::error::PitchError, note::NoteDetection};

pub trait NoteRenderer {
    /// Renders the note detected from the pitch detector
    fn render_note(&self, note: NoteDetection) -> anyhow::Result<()>;

    /// Renders "no note detected"
    fn render_no_note(&self, error: PitchError) -> anyhow::Result<()>;

    /// Initializes the renderer
    fn initialize(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Tears down the renderer
    fn tear_down(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
