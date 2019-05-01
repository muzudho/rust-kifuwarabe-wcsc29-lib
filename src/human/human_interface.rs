use audio_compo::cassette_deck::CassetteDeck;
use audio_compo::cassette_deck::Slot;
use instrument::position::*;
use std::*;
use studio::application::Application;

pub struct HumanInterface {}
impl HumanInterface {
    /// 盤面と持ち駒だけ表示。
    pub fn show_position(position: &Position, app: &Application) {
        if app.is_debug() {
            // 盤面。
            app.comm.println(&position.to_text(
                &app.comm,
                position.get_phase(),
                position.get_board_size(),
            ));
        }
    }

    /// 指定スロットの局面の表示。
    pub fn bo(deck: &mut CassetteDeck, position: &Position, app: &Application) {
        if app.is_debug() {
            // 何手目か。
            app.comm.println(&format!(
                "[Ply: T{},L{}]",
                deck.get_ply(Slot::Training),
                deck.get_ply(Slot::Learning),
            ));
            // 局面。
            HumanInterface::show_position(position, &app);
        }
    }

    /// 棋譜の表示。
    pub fn kifu(deck: &mut CassetteDeck, slot: Slot, position: &Position, app: &Application) {
        if app.is_debug() {
            if let Some(ref tape_box) = deck.slots[slot as usize].tape_box {
                let (_numbers, operations) =
                    &tape_box.get_sign_of_current_tape(position.get_board_size());

                use audio_compo::cassette_deck::Slot::*;
                app.comm.println(&format!(
                    "{}-Score: {}",
                    match slot {
                        Training => "T",
                        Learning => "L",
                    }
                    .to_string(),
                    operations
                ));
            } else {
                panic!("tape_box none.")
            }
        }
    }

    /// 局面と、テープ中の棋譜　の表示。
    /// トレーニング用。
    pub fn bo_with_tape(deck: &CassetteDeck, slot: Slot, position: &Position, app: &Application) {
        if app.is_debug() {
            // 局面。
            HumanInterface::show_position(position, &app);

            // 棋譜。
            let (_numbers, operations) =
                deck.get_sign_of_current_tape(slot, position.get_board_size());
            app.comm.println(&format!("TAPE: {}", operations));
        }
    }
}
