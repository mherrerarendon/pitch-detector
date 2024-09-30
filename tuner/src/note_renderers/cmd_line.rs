use super::NoteRenderer;
use pitch_detector::note::NoteDetectionResult;

pub struct CmdLineNoteRenderer;
impl CmdLineNoteRenderer {
    fn clear_terminal() {
        print!("{}[2J", 27 as char);
    }
}

impl NoteRenderer for CmdLineNoteRenderer {
    fn render_note(note: NoteDetectionResult) {
        todo!()
    }
}
