use application::*;
use board_size::*;
use human::human_interface::*;
use instrument::game_player::*;
use instrument::position::*;
use musician::best_move_picker::*;
use piece_etc::*;
use sheet_music_format::kifu_usi::fen::*;
use sheet_music_format::kifu_usi::usi_converter::*;
use sheet_music_format::kifu_usi::usi_position::*;
use video_recorder::cassette_deck::*;

pub struct LibSub {}
impl LibSub {
    pub fn back_1_note(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let ply = deck.get_ply(Slot::Learning);

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.turn_caret_to_negative();
            if let Some(rnote) = tape_box.go_1note_forcely(&app) {
                GamePlayer::try_touch_1note(&rnote, position, ply, &app);
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
                GamePlayer::try_touch_1note(&rnote, position, ply, &app);
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

        let best_umove = best_move_picker.get_mut_best_move(
            position,
            deck.slots[Slot::Learning as usize].ply,
            deck,
            &app,
        );
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
        let (line0, line1, line2, line3) = position.to_hand_4lines(Some(Phase::First));
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }
    pub fn hand2(position: &Position, app: &Application) {
        // TODO 後手の持ち駒を表示。
        let (line0, line1, line2, line3) = position.to_hand_4lines(Some(Phase::Second));
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }
    pub fn hand3(position: &Position, app: &Application) {
        // TODO 使っていない駒を表示。
        let (line0, line1, line2, line3) = position.to_hand_4lines(None);
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }

    pub fn position(
        line: String,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        // 相手が指したあとの局面まで進める。
        let mut urecord_opt = None;
        let mut start = 0;

        // 大橋流を指せるところまで、局面を戻す。
        GamePlayer::clear_to_honshogi_origin(position, deck, &app);
        if Fen::parse_initial_position(&line, &mut start, position, deck, &app) {
            urecord_opt = UsiPosition::parse_usi_line_moves(
                &app.comm,
                &line,
                &mut start,
                position.get_board_size(),
            );
        }
        //comm.println("#Position parse end1.");
        //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);

        // USI -> RPM 変換を作れていないので、ポジションをもう１回初期局面に戻してから、プレイアウトします。
        // TODO できれば USI -> RPM 変換したい。
        // comm.println("#Lib: TODO 'position' command(2).");
        {
            //comm.println("#Lib: 'position' command(2).");
            let mut start = 0;

            // 大橋流を指せるところまで、局面を戻す。
            GamePlayer::clear_to_honshogi_origin(position, deck, &app);
            if Fen::parse_initial_position(&line, &mut start, position, deck, &app) {
                //comm.println("#Position parsed.");
            }

            if let Some(urecord) = urecord_opt {
                // 差し替え。
                deck.change(None, position.get_board_size(), &app);
                UsiConverter::play_out_usi_tape(position, &urecord, deck, &app);
            }
            //comm.println("#Record converted1.");
            //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);
            //comm.println("#Record converted2.");
        }
    }
}
