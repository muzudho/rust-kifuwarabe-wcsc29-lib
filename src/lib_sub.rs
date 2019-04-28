use application::*;
use board_size::*;
use human::human_interface::*;
use kifu_usi::usi_converter::*;
use object_rpm::cassette_deck::*;
use piece_etc::*;
use shogi_ban::game_player::*;
use shogi_ban::position::*;
use thought::best_move_picker::*;

pub struct LibSub {}
impl LibSub {
    pub fn back_1_note(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_negative();
            if let Some(rnote) = tape_box.go_1note_forcely(&app) {
                GamePlayer::try_1note_on_1note(&rnote, position, ply, &app);
            }
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn back_1_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_negative();
            GamePlayer::try_read_tape_for_1move(tape_box, position, ply, &app);
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn back_10_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_negative();
            for _i in 0..10 {
                GamePlayer::try_read_tape_for_1move(tape_box, position, ply, &app);
            }
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn back_400_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_negative();
            for _i in 0..400 {
                GamePlayer::try_read_tape_for_1move(tape_box, position, ply, &app);
            }
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn forward_1_note(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_positive();
            if let Some(rnote) = tape_box.go_1note_forcely(&app) {
                GamePlayer::try_1note_on_1note(&rnote, position, ply, &app);
            }
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn forward_1_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        // 非合法タッチは自動で戻します。
        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_positive();
            GamePlayer::try_read_tape_for_1move(tape_box, position, ply, &app);
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn forward_10_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_positive();
            for _i in 0..10 {
                GamePlayer::try_read_tape_for_1move(tape_box, position, ply, &app);
            }
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn forward_400_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_positive();
            for _i in 0..400 {
                GamePlayer::try_read_tape_for_1move(tape_box, position, ply, &app);
            }
        } else {
            panic!("Tape box none.");
        }

        HumanInterface::bo(deck, Slot::Learning, &position, &app);
    }

    pub fn go(
        best_move_picker: &mut BestMovePicker,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_positive();
        } else {
            panic!("Tape box none.");
        }

        let best_umove = best_move_picker.get_mut_best_move(position, deck, &app);
        // Examples.
        // println!("bestmove 7g7f");
        // println!("bestmove win");
        // println!("bestmove resign");
        app.comm
            .println(&format!("bestmove {}", best_umove.to_sign()));

        // USI を再翻訳して再生するぜ☆（＾～＾）
        let rnote_opes =
            UsiConverter::convert_move(best_umove, &position, deck.get_ply(Slot::Learning));
        for rnote_ope in rnote_opes {
            app.comm.println("lib.rs:go: touch_1note_ope");
            GamePlayer::touch_1note_ope(&rnote_ope, position, deck, &app);
        }
    }

    pub fn gameover(board_size: BoardSize, deck: &mut CassetteDeck, app: &Application) {
        // TODO とりあえず、テープが１個入った　テープ・ボックス形式で書きだし☆（＾～＾）
        deck.write_tape_box(board_size, &app);
    }

    pub fn hand1(position: &Position, app: &Application) {
        // TODO 先手の持ち駒を表示。
        app.comm.println(&position.to_hand_text(Some(Phase::First)));
    }
    pub fn hand2(position: &Position, app: &Application) {
        // TODO 後手の持ち駒を表示。
        app.comm
            .println(&position.to_hand_text(Some(Phase::Second)));
    }
    pub fn hand3(position: &Position, app: &Application) {
        // TODO 使っていない駒を表示。
        app.comm.println(&position.to_hand_text(None));
    }
}
