pub mod cmd_line;

use pitch_detector::note::NoteDetectionResult;

pub trait NoteRenderer {
    fn render_note(&mut self, note: NoteDetectionResult) -> anyhow::Result<()>;
    fn render_no_note(&mut self) -> anyhow::Result<()>;
    fn input_start(&mut self) -> anyhow::Result<()>;
    fn input_end(&mut self) -> anyhow::Result<()>;
}
