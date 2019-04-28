use application::Application;
use common::caret::*;
use human::human_interface::*;
use object_rpm::cassette_deck::*;
use object_rpm::cassette_tape_recorder::*;
use object_rpm::shogi_note_operation::*;
use position::*;
use std::*;

pub struct CassetteTapeConverter {}
impl CassetteTapeConverter {
    /// Operation トラック文字列読取。
    pub fn read_ope_track(
        line: &str,
        position: &mut Position,
        tape_box_conveyor: &mut CassetteDeck,
        app: &Application,
    ) {
        let mut caret = Caret::new_facing_right_caret();

        loop {
            if caret.is_greater_than_or_equal_to(line.len() as i16) {
                return;
            }

            let tuple =
                ShogiNoteOpe::parse_1ope(&line, &mut caret, position.get_board_size(), &app.comm);

            if let (_last_used_caret, Some(rnote_ope)) = tuple {
                app.comm
                    .println("rpm_cassette_tape_editor.rs:read_ope_track: touch_1note_ope");
                CassetteTapeRecorder::touch_1note_ope(
                    &rnote_ope,
                    position,
                    tape_box_conveyor,
                    &app,
                );

                let ply = if let Some(ply) = rnote_ope.get_phase_change() {
                    ply
                } else {
                    -1
                };

                HumanInterface::bo(tape_box_conveyor, &position, &app);
            }
        }
    }
}
