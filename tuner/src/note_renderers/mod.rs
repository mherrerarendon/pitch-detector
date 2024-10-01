mod cmd_line;

use pitch_detector::note::NoteDetectionResult;

pub trait NoteRenderer {
    fn render_note(&mut self, note: NoteDetectionResult) -> anyhow::Result<()>;
}
