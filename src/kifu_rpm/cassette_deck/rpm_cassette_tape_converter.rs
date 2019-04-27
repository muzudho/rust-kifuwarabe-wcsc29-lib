use common::caret::*;
use communication::*;
use human::human_interface::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_editor::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_recorder::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::RpmCassetteTapeBoxConveyor;
use kifu_rpm::thread::rpm_note_operation::*;
use position::*;
use std::*;

pub struct RpmCassetteTapeConverter {}
impl RpmCassetteTapeConverter {
    /// Operation トラック文字列読取。
    pub fn read_ope_track(
        line: &str,
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        tape_editor: &mut RpmCassetteTapeEditor,
        comm: &Communication,
    ) {
        let mut caret = Caret::new_facing_right_caret();

        loop {
            if caret.is_greater_than_or_equal_to(line.len() as i16) {
                return;
            }

            let tuple = RpmNoteOpe::parse_1ope(&line, &mut caret, position.get_board_size(), &comm);

            if let (_last_used_caret, Some(rnote_ope)) = tuple {
                comm.println("rpm_cassette_tape_editor.rs:read_ope_track: touch_1note_ope");
                RpmCassetteTapeRecorder::touch_1note_ope(
                    &rnote_ope,
                    position,
                    tape_box_conveyor,
                    tape_editor,
                    comm,
                );

                let ply = if let Some(ply) = rnote_ope.get_phase_change() {
                    ply
                } else {
                    -1
                };
                HumanInterface::bo(
                    comm,
                    &tape_box_conveyor.recording_cassette_tape,
                    ply,
                    &position,
                );
            }
        }
    }
}
