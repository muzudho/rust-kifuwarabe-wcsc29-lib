use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::half_player_phase::*;
use instrument::position::*;
use live::best_move_picker::*;
use sheet_music_format::kifu_usi::fen::*;
use sheet_music_format::kifu_usi::usi_converter::*;
use sheet_music_format::kifu_usi::usi_position::*;
use studio::application::*;
use studio::board_size::*;

pub struct LibSub {}
impl LibSub {
    pub fn back_1_note(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let slot = Slot::Learning;
        let rnote_opt = if let Some(ref mut tape_box) = &mut deck.slots[slot as usize].tape_box {
            tape_box.look_back_caret_to_negative(&app);
            if let (_taken_overflow, _note_move, Some(rnote)) = tape_box.seek_to_next(&app) {
                Some(rnote)
            } else {
                None
            }
        } else {
            panic!("Tape box none.");
        };

        if let Some(rnote) = rnote_opt {
            if !position.try_beautiful_touch(&deck, slot, &rnote, &app) {
                app.comm.println("Touch fail.");
            }
        }

        HumanInterface::bo(deck, &position, &app);
    }

    pub fn back_1_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.look_back_caret_to_negative(Slot::Learning, &app);
        deck.try_seek_1move(Slot::Learning, position, &app);
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn back_10_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.look_back_caret_to_negative(Slot::Learning, &app);
        for _i in 0..10 {
            deck.try_seek_1move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn back_400_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.look_back_caret_to_negative(Slot::Learning, &app);
        for _i in 0..400 {
            deck.try_seek_1move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn forward_1_note(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let slot = Slot::Learning;
        deck.look_back_caret_to_positive(slot, &app);
        if let (_taken_overflow, _note_move, Some(rnote)) = deck.seek_to_next(Slot::Learning, &app)
        {
            if !position.try_beautiful_touch(&deck, slot, &rnote, &app) {
                app.comm.println("Touch fail.");
            }
        }
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn forward_1_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        // 非合法タッチは自動で戻します。
        deck.look_back_caret_to_positive(Slot::Learning, &app);
        deck.try_seek_1move(Slot::Learning, position, &app);
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn forward_10_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.look_back_caret_to_positive(Slot::Learning, &app);
        for _i in 0..10 {
            deck.try_seek_1move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn forward_400_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.look_back_caret_to_positive(Slot::Learning, &app);
        for _i in 0..400 {
            deck.try_seek_1move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn go(
        best_move_picker: &mut BestMovePicker,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        let board_size = position.get_board_size();

        if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
            tape_box.look_back_caret_to_positive(&app);
        } else {
            panic!("Tape box none.");
        }

        let best_umove = best_move_picker.get_mut_best_move(position, deck, &app);
        // Examples.
        // println!("bestmove 7g7f");
        // println!("bestmove win");
        // println!("bestmove resign");
        app.comm
            .println(&format!("bestmove {}", best_umove.to_sign(&app)));

        // USI を再翻訳して再生するぜ☆（＾～＾）
        let rnote_opes =
            UsiConverter::convert_move(best_umove, &position, deck.get_ply(Slot::Learning), &app);
        for rnote_ope in rnote_opes {
            // app.comm.println("lib.rs:go: touch_1note_ope");

            // 非合法手はいったん出力し、将棋所の方でエラーにする☆（＾～＾）
            position.touch_1note_ope(deck, &rnote_ope, true, board_size, &app);
        }
    }

    pub fn gameover(board_size: BoardSize, deck: &mut CassetteDeck, app: &Application) {
        // TODO とりあえず、テープが１個入った　テープ・ボックス形式で書きだし☆（＾～＾）
        deck.write_tape_box(board_size, &app);
    }

    pub fn hand1(position: &Position, app: &Application) {
        // TODO 先手の持ち駒を表示。
        let (line0, line1, line2, line3) = position.to_hand_4lines(HalfPlayerPhaseValue::First);
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }
    pub fn hand2(position: &Position, app: &Application) {
        // TODO 後手の持ち駒を表示。
        let (line0, line1, line2, line3) = position.to_hand_4lines(HalfPlayerPhaseValue::Second);
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }
    pub fn hand3(position: &Position, app: &Application) {
        // TODO 使っていない駒を表示。
        let (line0, line1, line2, line3) =
            position.to_hand_4lines(HalfPlayerPhaseValue::ZeroPointFive); // TODO とりあえず 0.5 で。
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

        // 指定局面にリセットするぜ☆（＾～＾）
        if Fen::parse_initial_position(&line, &mut start, position, deck, &app) {
            // USI の moves の文字列を、オブジェクトに直訳するぜ☆（＾～＾）局面は指定局面から動かさないぜ☆（＾～＾）
            urecord_opt = UsiPosition::parse_usi_line_moves(
                &line,
                &mut start,
                position.get_board_size(),
                &app,
            );
        }
        //comm.println("#Position parse end1.");
        //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);

        // USI -> RPM 変換を作れていないので、ポジションをもう１回初期局面に戻してから、プレイアウトします。
        // TODO できれば USI -> RPM 変換したい。
        // comm.println("#Lib: TODO 'position' command(2).");
        {
            //comm.println("#Lib: 'position' command(2).");
            //let mut start = 0;

            /*
            // 大橋流を指せるところまで、局面を戻す。
            GamePlayer::clear_to_honshogi_origin(position, deck, &app);
            if Fen::parse_initial_position(&line, &mut start, position, deck, &app) {
                //comm.println("#Position parsed.");
            }
            */

            if let Some(urecord) = urecord_opt {
                // 差し替え。
                deck.change_training_tape(None, position.get_board_size(), &app);
                UsiConverter::play_out_usi_tape(position, &urecord, deck, &app);
            }
            //comm.println("#Record converted1.");
            //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);
            //comm.println("#Record converted2.");
        }
    }
}
