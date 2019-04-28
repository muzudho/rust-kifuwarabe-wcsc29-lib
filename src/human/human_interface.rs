use application::Application;
use communication::*;
use object_rpm::cassette_deck::CassetteDeck;
use object_rpm::cassette_tape::CassetteTape;
use position::*;
use std::*;

pub struct HumanInterface {}
impl HumanInterface {
    /// 局面だけ表示。
    pub fn show_position(comm: &Communication, ply: i16, position: &Position) {
        // 何手目か。
        comm.println(&format!("[{}]", ply));
        // 盤面。
        comm.println(&position.to_text(comm, position.get_phase(), position.get_board_size()));
    }

    /// 局面と、記録中の棋譜　の表示。
    pub fn bo(deck: &mut CassetteDeck, position: &Position, app: &Application) {
        // 局面。
        HumanInterface::show_position(&app.comm, deck.recording_tape_ply, position);

        // 棋譜。
        let (_numbers, operations) =
            &deck.to_mut_recording_tape_sign(position.get_board_size(), &app);
        app.comm.println(&format!("REC: {}", operations));
    }

    /// 局面と、テープ中の棋譜　の表示。
    pub fn bo_with_tape(tape: &CassetteTape, ply: i16, position: &Position, app: &Application) {
        // 局面。
        HumanInterface::show_position(&app.comm, ply, position);

        // 棋譜。
        let (_numbers, operations) = &tape.to_sign(position.get_board_size());
        app.comm.println(&format!("TAPE: {}", operations));
    }
}
