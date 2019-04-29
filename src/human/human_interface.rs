use instrument::position::*;
use std::*;
use studio::application::Application;
use studio::communication::*;
use video_recorder::cassette_deck::CassetteDeck;
use video_recorder::cassette_deck::Slot;
use video_recorder::cassette_tape_box::CassetteTapeBox;

pub struct HumanInterface {}
impl HumanInterface {
    /// 局面だけ表示。
    pub fn show_position(slot: Slot, comm: &Communication, ply: i16, position: &Position) {
        // 何手目か。
        comm.println(&format!("[{}]", ply));
        // 盤面。
        comm.println(&position.to_text(
            slot,
            comm,
            position.get_phase(),
            position.get_board_size(),
        ));
    }

    /// トレーニング局面と、棋譜　の表示。
    pub fn bo(deck: &mut CassetteDeck, slot: Slot, position: &Position, app: &Application) {
        /*
        use video_recorder::cassette_deck::Slot::*;
        match slot {
            Training => {
                app.comm.println("<TRAINING>");
            }
            Learning => {
                app.comm.println("<LEARN>");
            }
        }
         */

        // 局面。
        HumanInterface::show_position(slot, &app.comm, deck.slots[slot as usize].ply, position);

        // 棋譜。
        if let Some(ref tape_box) = deck.slots[slot as usize].tape_box {
            let (_numbers, operations) =
                &tape_box.get_sign_of_current_tape(position.get_board_size());
            app.comm.println(&format!("Score: {}", operations));
        } else {
            panic!("tape_box none.")
        }
    }

    /// 局面と、テープ中の棋譜　の表示。
    /// トレーニング用。
    pub fn bo_with_tape(
        tape_box: &CassetteTapeBox,
        ply: i16,
        position: &Position,
        app: &Application,
    ) {
        // 局面。
        HumanInterface::show_position(Slot::Training, &app.comm, ply, position);

        // 棋譜。
        let (_numbers, operations) = &tape_box.get_sign_of_current_tape(position.get_board_size());
        app.comm.println(&format!("TAPE: {}", operations));
    }
}
