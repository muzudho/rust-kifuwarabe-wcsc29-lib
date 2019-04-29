use human::human_interface::*;
use instrument::piece_etc::*;
use instrument::position::*;
use live::ohashi_player::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::*;
use sound::shogi_note_operation::*;
use std::*;
use studio::application::Application;
use studio::common::caret::*;
use video_recorder::cassette_deck::CassetteDeck;
use video_recorder::cassette_deck::*;
use video_recorder::cassette_tape_box::CassetteTapeBox;

pub struct GamePlayer {}
impl GamePlayer {
    // 大橋流を指せるように、クリアーするぜ☆（＾～＾）
    pub fn clear_to_honshogi_origin(
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        // オリジン局面に戻す☆（＾～＾）
        deck.change(None, position.get_board_size(), &app);
        position.reset_origin_position();
    }

    // 大橋流で初期局面まで指す☆（＾～＾）
    pub fn play_startpos(position: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        // 大橋流で初期局面まで指す☆（＾～＾）
        OhashiPlayer::play_ohashi_starting(position, deck, &app);
    }

    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_1note_ope(
        // ノートの中に Ply もある☆（＾～＾）
        rnote_ope: &ShogiNoteOpe,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        app.comm.println(&format!(
            "[Touch 1note ope:{}]",
            rnote_ope.to_human_presentable(position.get_board_size())
        ));

        GamePlayer::touch_1note_ope_no_log(&rnote_ope, position, deck, &app);

        /*
        comm.println(&format!(
            "End     :touch_1note_ope. Rnote: {}.",
            rnote.to_human_presentable(board_size)
        ));
         */
        HumanInterface::show_position(
            Slot::Learning,
            &app.comm,
            deck.get_ply(Slot::Learning),
            position,
        );
    }

    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_1note_ope_no_log(
        // ノートの中に Ply もある☆（＾～＾）
        rnote_ope: &ShogiNoteOpe,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        deck.put_1note(
            Slot::Learning,
            ShogiNote::from_id_ope(
                if let (_is_legal_touch, Some(piece_identify)) =
                    position.try_beautiful_touch_no_log(&rnote_ope, &app.comm)
                {
                    PieceIdentify::from_number(piece_identify.get_id().get_number())
                } else {
                    None
                },
                *rnote_ope,
            ),
            app,
        );
    }

    /// 1手分進める。（非合法タッチは自動で戻します）
    ///
    /// 結果は、未着手か、１手　のどちらかです。
    ///
    /// # Return
    ///
    /// (指した１手分)
    pub fn try_read_tape_for_1move(
        tape_box: &mut CassetteTapeBox,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) -> Option<ShogiMove> {
        // とりあえず、キャレットを進めてみる☆（*＾～＾*）
        if let (caret_number, Some(_note)) = tape_box.go_to_next(&app) {
            let mut rmove = ShogiMove::new_facing_right_move();
            rmove
                .caret_closed_interval
                .intersect_caret_number(caret_number);

            app.comm.println(&format!(
                "[try_read_tape_for_1move:{}]",
                tape_box.to_human_presentable(),
            ));
            let mut is_legal_touch = true;

            let mut is_first = true;
            // フェーズ切り替えするまで、強制的に１ノート進め続けるぜ☆（＾～＾）。
            while let (_caret_number, Some(rnote)) = tape_box.go_to_next(&app) {
                app.comm.println(&format!(
                    "[LOOP try_read_tape_for_1move:{}:{}]",
                    tape_box.to_human_presentable(),
                    rnote.to_human_presentable(position.get_board_size())
                ));

                is_legal_touch = position.try_beautiful_touch(&rnote, ply, &app);

                if !is_first && !is_legal_touch {
                    break;
                }

                if !is_first && rnote.is_phase_change() {
                    // フェーズ切り替えしたら終了。（ただし、初回除く）
                    print!("<NXm1End {}>", rnote);
                    break;
                }

                is_first = false;
            }

            if !is_legal_touch {
                // 非合法タッチを自動で戻す。
                app.comm.println("Illegal, go opponent forcely!");
                tape_box.turn_caret_to_opponent();
                GamePlayer::read_tape_for_n_notes_forcely(
                    tape_box,
                    rmove.len() as u16,
                    position,
                    ply,
                    &app,
                );
                tape_box.turn_caret_to_opponent();

                return None;
            }

            // 1つ以上読んでいれば合法。
            /*
            if rmove.len() > 0 {
                rmove
            } else {
                None
            }
            */
            Some(rmove)
        } else {
            None
        }
    }

    pub fn read_tape_for_n_moves_forcely(
        tape_box: &mut CassetteTapeBox,
        repeats: i16,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) {
        for _i in 0..repeats {
            GamePlayer::read_tape_for_1move_forcely(tape_box, position, ply, &app);
        }
    }

    /// 必ず1手進める。（非合法タッチがあれば強制終了）
    pub fn read_tape_for_1move_forcely(
        tape_box: &mut CassetteTapeBox,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) {
        app.comm.println(&format!(
            "[read_tape_for_1move_forcely:{}]",
            tape_box.to_human_presentable(),
        ));

        let mut is_legal_touch = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら終了。
        while let (_caret_number, Some(rnote)) = tape_box.go_to_next(&app) {
            app.comm.println(&format!(
                "<LOOP read_tape_for_1move_forcely:{}:{}>",
                tape_box.to_human_presentable(),
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch = position.try_beautiful_touch(&rnote, ply, &app);
            forwarding_count += 1;

            if !is_legal_touch {
                break;
            }

            if forwarding_count != 1 && rnote.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                print!("<NXm1End{} {}>", forwarding_count, rnote);
                break;
            }
        }

        if !is_legal_touch {
            // 非合法タッチは強制終了。
            panic!("Illegal, go opponent forcely!");
        }

        // 1つも読まなかったら強制終了。
        if forwarding_count < 1 {
            panic!("Illegal, zero foward!");
        }
    }

    /// 非合法手はない前提で、強制的にテープを進めます。
    pub fn read_tape_for_n_notes_forcely(
        tape_box: &mut CassetteTapeBox,
        repeat: u16,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) {
        for i in 0..repeat {
            if let (_caret_number, Some(rnote)) = tape_box.go_to_next(&app) {
                app.comm
                    .println(&format!("<Go-force:{}/{} {}>", i, repeat, rnote));
                if !position.try_beautiful_touch(&rnote, ply, &app) {
                    panic!("Touch fail forcely.");
                }
            } else {
                panic!("<Go forcely fail:{}/{} None>", i, repeat);
            }
        }
    }

    /// Operation トラック文字列読取。
    pub fn read_ope_track(
        line: &str,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        let mut caret = Caret::new_facing_right_caret();

        loop {
            if caret.is_greater_than_or_equal_to(line.len() as i16) {
                return;
            }

            let tuple =
                ShogiNoteOpe::parse_1ope(&line, &mut caret, position.get_board_size(), &app.comm);

            if let (_last_used_caret, Some(rnote_ope)) = tuple {
                app.comm
                    .println("rpm_cassette_tape_editor.rs:read_ope_track: touch_1note_ope");
                GamePlayer::touch_1note_ope(&rnote_ope, position, deck, &app);

                HumanInterface::bo(deck, Slot::Learning, &position, &app);
            }
        }
    }
}
