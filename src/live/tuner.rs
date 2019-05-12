use audio_compo::audio_rack::*;
use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::position::*;
use live::base_performer::*;
use studio::application::*;
use studio::common::caret::*;

pub struct Tuner {}
impl Tuner {
    // #####
    // # B #
    // #####

    pub fn back_1_note(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        if app.is_debug() {
            app.comm.println("[#back_1_note]")
        }

        // ルックバックする。
        rack.look_back_caret(Slot::Learning, &app);

        // 棋譜上で１つ進む。
        let (taken_overflow, awareness, rnote_opt) = rack.seek_a_note(Slot::Learning, &app);

        if let Some(rnote) = rnote_opt {
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#back_1_note: {}{}, Note:{}]",
                    if taken_overflow {
                        "Overflow, ".to_string()
                    } else {
                        "".to_string()
                    },
                    awareness.to_human_presentable(),
                    rnote.to_human_presentable(position.get_board_size(), &app)
                ));
            }

            // 局面上でそのノートをタッチする。ログも出力する。
            if let (false, _) = position.touch_ope(
                rack.is_facing_left_of_current_tape(Slot::Learning, &app),
                &rnote.get_ope(),
                &app,
            ) {
                // タッチできないのはおかしい。
                app.comm.println("Touch fail.");
            }
        } else if app.is_debug() {
            // 戻れなかったというのはおかしい。
            app.comm.println(&format!(
                "[#back_1_note fail: {}{}, Note:None]",
                if taken_overflow {
                    "Overflow, ".to_string()
                } else {
                    "".to_string()
                },
                awareness.to_human_presentable()
            ));
        }

        // ルックバックする。
        rack.look_back_caret(Slot::Learning, &app);
    }

    pub fn back_1_move(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        rack.turn_caret_towards_negative_infinity(Slot::Learning, &app);
        BasePerformer::replay_a_move(rack, Slot::Learning, position, &app);
        HumanInterface::bo(rack, &position, &app);
    }

    pub fn back_10_move(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        rack.turn_caret_towards_negative_infinity(Slot::Learning, &app);
        for _i in 0..10 {
            let (sought_move_result, _rmove) =
                BasePerformer::replay_a_move(rack, Slot::Learning, position, &app);
            match sought_move_result {
                SoughtMoveResult::Aware => {}
                _ => {
                    break;
                }
            }
        }
        HumanInterface::bo(rack, &position, &app);
    }

    pub fn back_400_move(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        rack.turn_caret_towards_negative_infinity(Slot::Learning, &app);
        for _i in 0..400 {
            let (sought_move_result, _rmove) =
                BasePerformer::replay_a_move(rack, Slot::Learning, position, &app);
            match sought_move_result {
                SoughtMoveResult::Aware => {}
                _ => {
                    break;
                }
            }
        }
        HumanInterface::bo(rack, &position, &app);
    }

    // #####
    // # F #
    // #####

    pub fn forward_1_move(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        // 非合法タッチは自動で戻します。
        rack.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        BasePerformer::replay_a_move(rack, Slot::Learning, position, &app);
        HumanInterface::bo(rack, &position, &app);
    }

    pub fn forward_10_move(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        rack.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        for _i in 0..10 {
            BasePerformer::replay_a_move(rack, Slot::Learning, position, &app);
        }
        HumanInterface::bo(rack, &position, &app);
    }

    pub fn forward_400_move(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        rack.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        for _i in 0..400 {
            BasePerformer::replay_a_move(rack, Slot::Learning, position, &app);
        }
        HumanInterface::bo(rack, &position, &app);
    }

    // #####
    // # R #
    // #####

    pub fn replay_a_note(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        rack.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        if let (_taken_overflow, _awareness, Some(rnote)) = rack.seek_a_note(Slot::Learning, &app) {
            if let (false, _) = position.touch_ope(
                rack.is_facing_left_of_current_tape(Slot::Learning, &app),
                &rnote.get_ope(),
                &app,
            ) {
                app.comm.println("Touch fail.");
            }
        }
        HumanInterface::bo(rack, &position, &app);
    }

    pub fn rollback_note(rack: &mut AudioRack, position: &mut Position, app: &Application) {
        rack.look_back_caret(Slot::Training, &app);
        rack.look_back_caret(Slot::Learning, &app);
        BasePerformer::rollback_note(rack, Slot::Training, position, &app);
        rack.look_back_caret(Slot::Learning, &app);
        rack.look_back_caret(Slot::Training, &app);
    }

    /*
    pub fn rollback_move(
        rack: &mut AudioRack,
        slot_0: Slot,
        position: &mut Position,
        app: &Application,
    ) {
        deck.slots[Slot::Training as usize].look_back_caret(&app);
        deck.slots[Slot::Learning as usize].look_back_caret(&app);
        BasePerformer::rollback_move(deck, Slot::Training, position, &app);
        deck.slots[Slot::Learning as usize].look_back_caret(&app);
        deck.slots[Slot::Training as usize].look_back_caret(&app);
    }
    */
}
