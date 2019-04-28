use address::*;
use application::Application;
use board_size::*;
use common::caret::*;
use communication::*;
use human::human_interface::*;
use object_rpm::cassette_deck::CassetteDeck;
use object_rpm::cassette_deck::*;
use object_rpm::cassette_tape::*;
use object_rpm::cassette_tape_box::CassetteTapeBox;
use object_rpm::shogi_note::*;
use object_rpm::shogi_note_operation::*;
use piece_etc::*;
use shogi_ban::position::*;
use std::*;

pub struct GamePlayer {}
impl GamePlayer {
    /// 初期化に使う。
    fn init_note(
        ply: i16,
        ph: Phase,
        file: i8,
        rank: i8,
        pid: PieceIdentify,
        bs: BoardSize,
    ) -> (ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe) {
        (
            ShogiNoteOpe::from_address(Address::from_hand_pi(Piece::from_ph_pid(Some(ph), pid))),
            ShogiNoteOpe::from_address(Address::from_cell(Cell::from_file_rank(file, rank), bs)),
            ShogiNoteOpe::change_phase(ply),
        )
    }

    /// オリジン・ポジションから、平手初期局面に進めます。
    pub fn play_ohashi_starting(pos: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        use piece_etc::Phase::*;
        use piece_etc::PieceIdentify::*;

        // 大橋流の順序にしてください。
        let bs = pos.get_board_size();
        let array: [(ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe); 40] = [
            GamePlayer::init_note(-39, Second, 5, 1, K00, bs),
            GamePlayer::init_note(-38, First, 5, 9, K01, bs),
            GamePlayer::init_note(-37, Second, 4, 1, G02, bs),
            GamePlayer::init_note(-36, First, 6, 9, G03, bs),
            GamePlayer::init_note(-35, Second, 6, 1, G04, bs),
            GamePlayer::init_note(-34, First, 4, 9, G05, bs),
            GamePlayer::init_note(-33, Second, 3, 1, S06, bs),
            GamePlayer::init_note(-32, First, 7, 9, S07, bs),
            GamePlayer::init_note(-31, Second, 7, 1, S08, bs),
            GamePlayer::init_note(-30, First, 3, 9, S09, bs),
            GamePlayer::init_note(-29, Second, 2, 1, N10, bs),
            GamePlayer::init_note(-28, First, 8, 9, N11, bs),
            GamePlayer::init_note(-27, Second, 8, 1, N12, bs),
            GamePlayer::init_note(-26, First, 2, 9, N13, bs),
            GamePlayer::init_note(-25, Second, 1, 1, L14, bs),
            GamePlayer::init_note(-24, First, 9, 9, L15, bs),
            GamePlayer::init_note(-23, Second, 9, 1, L16, bs),
            GamePlayer::init_note(-22, First, 1, 9, L17, bs),
            GamePlayer::init_note(-21, Second, 2, 2, B18, bs),
            GamePlayer::init_note(-20, First, 8, 8, B19, bs),
            GamePlayer::init_note(-19, Second, 8, 2, R20, bs),
            GamePlayer::init_note(-18, First, 2, 8, R21, bs),
            GamePlayer::init_note(-17, Second, 5, 3, P22, bs),
            GamePlayer::init_note(-16, First, 5, 7, P23, bs),
            GamePlayer::init_note(-15, Second, 4, 3, P24, bs),
            GamePlayer::init_note(-14, First, 6, 7, P25, bs),
            GamePlayer::init_note(-13, Second, 6, 3, P26, bs),
            GamePlayer::init_note(-12, First, 4, 7, P27, bs),
            GamePlayer::init_note(-11, Second, 3, 3, P28, bs),
            GamePlayer::init_note(-10, First, 7, 7, P29, bs),
            GamePlayer::init_note(-9, Second, 7, 3, P30, bs),
            GamePlayer::init_note(-8, First, 3, 7, P31, bs),
            GamePlayer::init_note(-7, Second, 2, 3, P32, bs),
            GamePlayer::init_note(-6, First, 8, 7, P33, bs),
            GamePlayer::init_note(-5, Second, 8, 3, P34, bs),
            GamePlayer::init_note(-4, First, 2, 7, P35, bs),
            GamePlayer::init_note(-3, Second, 1, 3, P36, bs),
            GamePlayer::init_note(-2, First, 9, 7, P37, bs),
            GamePlayer::init_note(-1, Second, 9, 3, P38, bs),
            GamePlayer::init_note(0, First, 1, 7, P39, bs),
        ];

        for element in array.iter() {
            app.comm
                .println("rpm_move_player.rs:play_ohashi_starting: touch_1note_ope");
            GamePlayer::touch_1note_ope(&element.0, pos, deck, &app);
            GamePlayer::touch_1note_ope(&element.1, pos, deck, &app);
            GamePlayer::touch_1note_ope(&element.2, pos, deck, &app);
        }
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
        let board_size = position.get_board_size();
        let pid_opt = if let (_is_legal_touch, Some(piece_identify)) =
            position.touch_beautiful_1note(&rnote_ope, &app.comm, board_size)
        {
            PieceIdentify::from_number(piece_identify.get_id().get_number())
        } else {
            None
        };

        HumanInterface::show_position(&app.comm, deck.get_ply(Slot::Learning), position);
        let rnote = ShogiNote::from_id_ope(pid_opt, *rnote_ope);
        /*
        comm.println(&format!(
            "End     :touch_1note_ope. Rnote: {}.",
            rnote.to_human_presentable(board_size)
        ));
         */
        deck.put_1note(Slot::Learning, rnote, app);
    }

    /// 指定のノートを実行（タッチ）するだけ。（非合法タッチでも行います）
    /// Next も Back も違いはない。キャレットは使わない。
    /// 動かせなかったなら、Noneを返す。
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn try_1note_on_1note(
        rnote: &ShogiNote,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) -> bool {
        let board_size = position.get_board_size();

        app.comm.println(&format!(
            "<NXn:{}>",
            rnote.to_human_presentable(board_size) //rnote.get_ope().to_human_presentable(board_size)
        ));
        let (is_legal_touch, _piece_identify_opt) =
            position.touch_beautiful_1note(&rnote.get_ope(), &app.comm, board_size);
        HumanInterface::show_position(&app.comm, ply, position);

        is_legal_touch
    }

    /// 1手進める。（非合法タッチは自動で戻します）
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn try_read_tape_for_1move(
        cassette_tape: &mut CassetteTape,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) -> bool {
        let mut is_legal_touch = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら終了。
        while let Some(rnote) = cassette_tape.go_1note_forcely(&app.comm) {
            app.comm.println(&format!(
                "<NXm1:{}>",
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch = GamePlayer::try_1note_on_1note(&rnote, position, ply, &app);
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
            // 非合法タッチを自動で戻す。
            app.comm.println("Illegal, go opponent forcely!");
            cassette_tape.caret.turn_to_opponent();
            GamePlayer::read_tape_for_n_notes_forcely(
                cassette_tape,
                forwarding_count,
                position,
                ply,
                &app,
            );
            cassette_tape.caret.turn_to_opponent();

            return false;
        }

        // 1つ以上読んでいれば合法。
        forwarding_count > 0
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
        let mut is_legal_touch = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら終了。
        while let Some(rnote) = tape_box.go_1note_forcely(&app) {
            app.comm.println(&format!(
                "<NXm1:{}>",
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch = GamePlayer::try_1note_on_1note(&rnote, position, ply, &app);
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
        cassette_tape: &mut CassetteTape,
        repeat: u8,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) {
        for i in 0..repeat {
            if let Some(rnote) = cassette_tape.go_1note_forcely(&app.comm) {
                app.comm
                    .println(&format!("<Go-force:{}/{} {}>", i, repeat, rnote));
                GamePlayer::try_1note_on_1note(&rnote, position, ply, &app);
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

                let ply = if let Some(ply) = rnote_ope.get_phase_change() {
                    ply
                } else {
                    -1
                };

                HumanInterface::bo(deck, Slot::Learning, &position, &app);
            }
        }
    }
}
