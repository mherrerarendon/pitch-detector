pub mod cmd_line;

use pitch_detector::{core::error::PitchError, note::NoteDetection};

pub trait NoteRenderer {
    /// Renders the note detected from the pitch detector
    fn render_note(&mut self, note: NoteDetection) -> anyhow::Result<()>;

    /// Renders "no note detected"
    fn render_no_note(&mut self, error: PitchError) -> anyhow::Result<()>;

    /// Initializes the renderer
    fn initialize(&mut self) -> anyhow::Result<()>;

    /// Tears down the renderer
    fn tear_down(&mut self) -> anyhow::Result<()>;
}
