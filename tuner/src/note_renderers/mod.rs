pub mod cmd_line;

use pitch_detector::{core::error::PitchError, note::NoteDetectionResult};

pub trait NoteRenderer {
    fn render_note(&mut self, note: NoteDetectionResult) -> anyhow::Result<()>;
    fn render_no_note(&mut self, error: PitchError) -> anyhow::Result<()>;
    fn input_start(&mut self) -> anyhow::Result<()>;
    fn input_end(&mut self) -> anyhow::Result<()>;
}
