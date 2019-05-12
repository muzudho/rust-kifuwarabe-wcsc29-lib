use audio_compo::audio_rack::*;
use audio_compo::cassette_deck::Slot;
use instrument::position::*;
use std::*;
use studio::application::Application;

pub struct HumanInterface {}
impl HumanInterface {
    /// 局面の表示。
    pub fn bo(rack: &AudioRack, position: &Position, app: &Application) {
        if app.is_debug() {
            // 何手目か。
            app.comm.println(&format!(
                "[Ply: T{},L{}]",
                rack.get_ply(Slot::Training),
                rack.get_ply(Slot::Learning),
            ));
            // 局面。
            app.comm.println(&position.to_text());
        }
    }

    /// 棋譜の表示。
    pub fn kifu(rack: &AudioRack, slot: Slot, position: &Position, app: &Application) {
        if app.is_debug() {
            let (_numbers, operations) =
                &rack.get_sign_of_current_tape(slot, position.get_board_size());

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
        }
    }

    /// 局面と、テープ中の棋譜　の表示。
    /// トレーニング用。
    pub fn bo_with_tape(rack: &AudioRack, slot: Slot, position: &Position, app: &Application) {
        if app.is_debug() {
            // 局面。
            HumanInterface::bo(rack, position, &app);

            // 棋譜。
            let (_numbers, operations) =
                rack.get_sign_of_current_tape(slot, position.get_board_size());
            app.comm.println(&format!("TAPE: {}", operations));
        }
    }
}
