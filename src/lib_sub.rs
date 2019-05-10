use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::half_player_phase::*;
use instrument::piece_etc::*;
use instrument::position::*;
use live::best_move_picker::*;
use media::cassette_tape::*;
use regex::Regex;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sheet_music_format::kifu_usi::fen::*;
use sheet_music_format::kifu_usi::usi_converter::*;
use sheet_music_format::kifu_usi::usi_position::*;
use studio::application::*;
use studio::board_size::*;

pub struct LibSub {}
impl LibSub {
    pub fn back_1_note(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        let slot = Slot::Learning;
        let rnote_opt = {
            let tape_box = &mut deck.slots[slot as usize];
            tape_box.look_back_caret_to_negative(&app);
            if let (_taken_overflow, _note_move, Some(rnote)) = tape_box.seek_to_next_note(&app) {
                Some(rnote)
            } else {
                None
            }
        };

        if let Some(rnote) = rnote_opt {
            if !position.try_beautiful_touch(deck, &rnote, &app) {
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
        deck.look_back_caret_to_positive(Slot::Learning, &app);
        if let (_taken_overflow, _note_move, Some(rnote)) =
            deck.seek_to_next_note(Slot::Learning, &app)
        {
            if !position.try_beautiful_touch(&deck, &rnote, &app) {
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

        deck.slots[Slot::Learning as usize].look_back_caret_to_positive(&app);

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

        // USI -> RPM 変換を作れていないので、ポジションをもう１回初期局面に戻してから、プレイアウトします。
        // TODO できれば USI -> RPM 変換したい。
        if let Some(urecord) = urecord_opt {
            // 差し替え。
            deck.clear_of_tapes(Slot::Training, &app);
            UsiConverter::play_out_usi_tape(position, &urecord, deck, &app);
        }
    }

    pub fn scan_pid(
        line: &str,
        deck: &mut CassetteDeck,
        position: &mut Position,
        app: &Application,
    ) {
        let re = Regex::new(r"scan-pid\s+(\d+)")
            .unwrap_or_else(|f| panic!(app.comm.panic(&f.to_string())));
        let matched = re
            .captures(line)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. parse.")));
        let pnum_str = matched.get(1).map_or("", |m| m.as_str());
        let pnum: i8 = pnum_str.parse().unwrap();
        let pid = if let Some(pid) = PieceIdentify::from_number(pnum) {
            pid
        } else {
            app.comm
                .println(&format!("[#Scan pid fail: Pnum: {}]", pnum));
            return;
        };

        // 記録係フェーズなんで、もう１つ先に進めるぜ☆（＾～＾）
        position.go_next_phase(deck);

        if let Some((idp, addr)) = position.scan_pid(position.get_phase().get_state(), pid) {
            app.comm.println(&format!(
                "[#Scan pid: Found pnum:{}, Idp:{}, Addr:{}]",
                pnum,
                idp.to_human_presentable(),
                addr.to_human_presentable(position.get_board_size())
            ));
        } else {
            app.comm
                .println(&format!("[#Scan pid: Not found pnum: {}]", pnum));
        }

        // 進めた分を戻すぜ☆（＾～＾）
        position.back_phase(deck, &app);
    }

    pub fn usi_new_game(deck: &mut CassetteDeck, app: &Application) {
        // 今対局分のラーニング・テープを１つ追加するぜ☆（＾～＾）
        let learning_file_name_without_extension =
            &RpmTapeBox::create_file_full_name_without_extension(&app.kw29_conf, &app);
        let mut tape = CassetteTape::new_facing_right(&app);
        tape.set_file_full_name_without_extension(&learning_file_name_without_extension);
        deck.add_tape_to_tape_box(Slot::Learning, tape, &app);
        deck.seek_of_next_tape(Slot::Learning, &app);
    }
}
