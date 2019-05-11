use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::half_player_phase::*;
use instrument::piece_etc::*;
use instrument::position::*;
use live::best_move_picker::*;
use media::cassette_tape::*;
use media::two_heads_vec::*;
use regex::Regex;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sheet_music_format::kifu_usi::fen::*;
use sheet_music_format::kifu_usi::usi_converter::*;
use sheet_music_format::kifu_usi::usi_position::*;
use sound::shogi_note::*;
use sound::shogi_note_operation::*;
use studio::address::*;
use studio::application::*;
use studio::board_size::*;
use studio::common::caret::*;

pub struct LibSub {}
impl LibSub {
    // #####
    // # B #
    // #####

    pub fn back_walk_a_note_and_touch(
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        if app.is_debug() {
            app.comm.println("[#back_walk_a_note_and_touch]")
        }

        let (taken_overflow, awareness, rnote_opt) =
            deck.slots[Slot::Learning as usize].back_walk_a_note(&app);

        if let Some(rnote) = rnote_opt {
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#back_walk_a_note_and_touch: taken_overflow:{}, awareness:{:?}, Rnote:{}]",
                    taken_overflow,
                    awareness,
                    rnote.to_human_presentable(position.get_board_size(), &app)
                ));
            }

            if !position.try_beautiful_touch(deck, &rnote, &app) {
                app.comm.println("Touch fail.");
            }
        } else if app.is_debug() {
            app.comm.println(&format!(
                "[#back_walk_a_note_and_touch: taken_overflow:{}, awareness:{:?}, Rnote:None]",
                taken_overflow, awareness
            ));
        }

        HumanInterface::bo(deck, &position, &app);
    }

    pub fn back_1_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.turn_caret_towards_negative_infinity(Slot::Learning, &app);
        deck.seek_a_move(Slot::Learning, position, &app);
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn back_10_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.turn_caret_towards_negative_infinity(Slot::Learning, &app);
        for _i in 0..10 {
            deck.seek_a_move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn back_400_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.turn_caret_towards_negative_infinity(Slot::Learning, &app);
        for _i in 0..400 {
            deck.seek_a_move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    // #####
    // # F #
    // #####

    pub fn forward_1_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        // 非合法タッチは自動で戻します。
        deck.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        deck.seek_a_move(Slot::Learning, position, &app);
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn forward_10_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        for _i in 0..10 {
            deck.seek_a_move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    pub fn forward_400_move(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        for _i in 0..400 {
            deck.seek_a_move(Slot::Learning, position, &app);
        }
        HumanInterface::bo(deck, &position, &app);
    }

    // #####
    // # G #
    // #####

    pub fn go(
        best_move_picker: &mut BestMovePicker,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        let board_size = position.get_board_size();

        deck.slots[Slot::Learning as usize].turn_caret_towards_positive_infinity(&app);

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

    // #####
    // # H #
    // #####

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

    // #####
    // # L #
    // #####

    pub fn look_back(deck: &mut CassetteDeck, slot: Slot, app: &Application) {
        deck.look_back_caret(slot, &app)
    }

    // #####
    // # P #
    // #####

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

    // #####
    // # S #
    // #####

    pub fn seek_a_note(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        deck.turn_caret_towards_positive_infinity(Slot::Learning, &app);
        if let (_taken_overflow, _awareness, Some(rnote)) = deck.seek_a_note(Slot::Learning, &app) {
            if !position.try_beautiful_touch(&deck, &rnote, &app) {
                app.comm.println("Touch fail.");
            }
        }
        HumanInterface::bo(deck, &position, &app);
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
        position.seek_a_player(deck, &app);

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
        position.back_walk_player_phase(deck, &app);
    }

    // #####
    // # T #
    // #####

    pub fn test_2heads_vec(board_size: BoardSize, app: &Application) {
        let tvec = TwoHeadsVec::new();

        let mut caret = Caret::new_facing_right_caret();
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 最初:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app),
            ))
        }

        // １つ目
        let tvec = tvec.insert(
            &mut caret,
            ShogiNote::from_id_ope(None, ShogiNoteOpe::change_phase(0)),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: １つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }

        // ２つ目
        let tvec = tvec.insert(
            &mut caret,
            ShogiNote::from_id_ope(
                Some(PieceIdentify::K00),
                ShogiNoteOpe::from_address(Address::from_cell(
                    Cell::from_file_rank(5, 1),
                    board_size,
                )),
            ),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 2つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }

        // ３つ目
        let tvec = tvec.insert(
            &mut caret,
            ShogiNote::from_id_ope(
                Some(PieceIdentify::K00),
                ShogiNoteOpe::from_address(Address::from_cell(
                    Cell::from_file_rank(6, 2),
                    board_size,
                )),
            ),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 3つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }

        // ４つ目
        let tvec = tvec.insert(
            &mut caret,
            ShogiNote::from_id_ope(None, ShogiNoteOpe::change_phase(0)),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 4つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }
    }

    // #####
    // # U #
    // #####

    pub fn usi_new_game(deck: &mut CassetteDeck, app: &Application) {
        // 今対局分のラーニング・テープを１つ追加するぜ☆（＾～＾）

        // ラーニング・テープ作成。
        let mut tape = CassetteTape::new_facing_right(&app);
        tape.set_file_full_name_without_extension(
            &RpmTapeBox::create_file_full_name_without_extension(&app.kw29_conf, &app),
        );
        deck.add_tape_to_tape_box(Slot::Learning, tape, &app);
        deck.seek_of_next_tape(Slot::Learning, &app);
    }
}
