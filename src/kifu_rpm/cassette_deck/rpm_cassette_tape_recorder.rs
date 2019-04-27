use address::*;
use board_size::*;
use communication::*;
use human::human_interface::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_editor::*;
use kifu_rpm::object::rpm_cassette_tape::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::RpmCassetteTapeBoxConveyor;
use kifu_rpm::thread::rpm_note::*;
use kifu_rpm::thread::rpm_note_operation::*;
use piece_etc::*;
use position::*;
use std::*;

pub struct RpmCassetteTapeRecorder {}
impl RpmCassetteTapeRecorder {
    /// 初期化に使う。
    fn init_note(
        ply: i16,
        ph: Phase,
        file: i8,
        rank: i8,
        pid: PieceIdentify,
        bs: BoardSize,
    ) -> (RpmNoteOpe, RpmNoteOpe, RpmNoteOpe) {
        (
            RpmNoteOpe::from_address(Address::from_hand_pi(Piece::from_ph_pid(Some(ph), pid))),
            RpmNoteOpe::from_address(Address::from_cell(Cell::from_file_rank(file, rank), bs)),
            RpmNoteOpe::change_phase(ply),
        )
    }

    /// オリジン・ポジションから、平手初期局面に進めます。
    pub fn play_ohashi_starting(
        pos: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        comm: &Communication,
    ) {
        use piece_etc::Phase::*;
        use piece_etc::PieceIdentify::*;

        // 大橋流の順序にしてください。
        let bs = pos.get_board_size();
        let array: [(RpmNoteOpe, RpmNoteOpe, RpmNoteOpe); 40] = [
            RpmCassetteTapeRecorder::init_note(-39, Second, 5, 1, K00, bs),
            RpmCassetteTapeRecorder::init_note(-38, First, 5, 9, K01, bs),
            RpmCassetteTapeRecorder::init_note(-37, Second, 4, 1, G02, bs),
            RpmCassetteTapeRecorder::init_note(-36, First, 6, 9, G03, bs),
            RpmCassetteTapeRecorder::init_note(-35, Second, 6, 1, G04, bs),
            RpmCassetteTapeRecorder::init_note(-34, First, 4, 9, G05, bs),
            RpmCassetteTapeRecorder::init_note(-33, Second, 3, 1, S06, bs),
            RpmCassetteTapeRecorder::init_note(-32, First, 7, 9, S07, bs),
            RpmCassetteTapeRecorder::init_note(-31, Second, 7, 1, S08, bs),
            RpmCassetteTapeRecorder::init_note(-30, First, 3, 9, S09, bs),
            RpmCassetteTapeRecorder::init_note(-29, Second, 2, 1, N10, bs),
            RpmCassetteTapeRecorder::init_note(-28, First, 8, 9, N11, bs),
            RpmCassetteTapeRecorder::init_note(-27, Second, 8, 1, N12, bs),
            RpmCassetteTapeRecorder::init_note(-26, First, 2, 9, N13, bs),
            RpmCassetteTapeRecorder::init_note(-25, Second, 1, 1, L14, bs),
            RpmCassetteTapeRecorder::init_note(-24, First, 9, 9, L15, bs),
            RpmCassetteTapeRecorder::init_note(-23, Second, 9, 1, L16, bs),
            RpmCassetteTapeRecorder::init_note(-22, First, 1, 9, L17, bs),
            RpmCassetteTapeRecorder::init_note(-21, Second, 2, 2, B18, bs),
            RpmCassetteTapeRecorder::init_note(-20, First, 8, 8, B19, bs),
            RpmCassetteTapeRecorder::init_note(-19, Second, 8, 2, R20, bs),
            RpmCassetteTapeRecorder::init_note(-18, First, 2, 8, R21, bs),
            RpmCassetteTapeRecorder::init_note(-17, Second, 5, 3, P22, bs),
            RpmCassetteTapeRecorder::init_note(-16, First, 5, 7, P23, bs),
            RpmCassetteTapeRecorder::init_note(-15, Second, 4, 3, P24, bs),
            RpmCassetteTapeRecorder::init_note(-14, First, 6, 7, P25, bs),
            RpmCassetteTapeRecorder::init_note(-13, Second, 6, 3, P26, bs),
            RpmCassetteTapeRecorder::init_note(-12, First, 4, 7, P27, bs),
            RpmCassetteTapeRecorder::init_note(-11, Second, 3, 3, P28, bs),
            RpmCassetteTapeRecorder::init_note(-10, First, 7, 7, P29, bs),
            RpmCassetteTapeRecorder::init_note(-9, Second, 7, 3, P30, bs),
            RpmCassetteTapeRecorder::init_note(-8, First, 3, 7, P31, bs),
            RpmCassetteTapeRecorder::init_note(-7, Second, 2, 3, P32, bs),
            RpmCassetteTapeRecorder::init_note(-6, First, 8, 7, P33, bs),
            RpmCassetteTapeRecorder::init_note(-5, Second, 8, 3, P34, bs),
            RpmCassetteTapeRecorder::init_note(-4, First, 2, 7, P35, bs),
            RpmCassetteTapeRecorder::init_note(-3, Second, 1, 3, P36, bs),
            RpmCassetteTapeRecorder::init_note(-2, First, 9, 7, P37, bs),
            RpmCassetteTapeRecorder::init_note(-1, Second, 9, 3, P38, bs),
            RpmCassetteTapeRecorder::init_note(0, First, 1, 7, P39, bs),
        ];

        for element in array.iter() {
            comm.println("rpm_move_player.rs:play_ohashi_starting: touch_1note_ope");
            RpmCassetteTapeRecorder::touch_1note_ope(
                &element.0,
                pos,
                tape_box_conveyor,
                recorder,
                comm,
            );
            RpmCassetteTapeRecorder::touch_1note_ope(
                &element.1,
                pos,
                tape_box_conveyor,
                recorder,
                comm,
            );
            RpmCassetteTapeRecorder::touch_1note_ope(
                &element.2,
                pos,
                tape_box_conveyor,
                recorder,
                comm,
            );
        }
    }

    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_1note_ope(
        // ノートの中に Ply もある☆（＾～＾）
        rnote_ope: &RpmNoteOpe,
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        comm: &Communication,
    ) {
        let board_size = position.get_board_size();
        let pid_opt = if let (_is_legal_touch, Some(piece_identify)) =
            position.touch_beautiful_1note(&rnote_ope, comm, board_size)
        {
            PieceIdentify::from_number(piece_identify.get_id().get_number())
        } else {
            None
        };

        HumanInterface::show_position(comm, recorder.ply, position);
        let rnote = RpmNote::from_id_ope(pid_opt, *rnote_ope);
        /*
        comm.println(&format!(
            "End     :touch_1note_ope. Rnote: {}.",
            rnote.to_human_presentable(board_size)
        ));
         */
        recorder.put_1note(rnote, tape_box_conveyor, comm);
    }

    /// 指定のノートを実行（タッチ）するだけ。（非合法タッチでも行います）
    /// Next も Back も違いはない。キャレットは使わない。
    /// 動かせなかったなら、Noneを返す。
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn try_1note_on_1note(
        rnote: &RpmNote,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) -> bool {
        let board_size = position.get_board_size();

        comm.println(&format!(
            "<NXn:{}>",
            rnote.to_human_presentable(board_size) //rnote.get_ope().to_human_presentable(board_size)
        ));
        let (is_legal_touch, _piece_identify_opt) =
            position.touch_beautiful_1note(&rnote.get_ope(), comm, board_size);
        HumanInterface::show_position(comm, ply, position);

        is_legal_touch
    }

    /// 棋譜の上を進めます。
    pub fn try_n_moves_on_tape(
        repeats: i16,
        ply: i16,
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        comm: &Communication,
    ) {
        for _i in 0..repeats {
            RpmCassetteTapeRecorder::try_1move_on_tape(cassette_tape, position, ply, false, &comm);
        }
    }

    /// 1手進める。（非合法タッチは自動で戻します）
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn try_1move_on_tape(
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        is_auto_reverse: bool,
        comm: &Communication,
    ) -> bool {
        let mut is_legal_touch = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら終了。
        while let Some(rnote) = cassette_tape.go_1note_forcely(comm) {
            comm.println(&format!(
                "<NXm1:{}>",
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch =
                RpmCassetteTapeRecorder::try_1note_on_1note(&rnote, position, ply, comm);
            forwarding_count += 1;

            if is_auto_reverse && !is_legal_touch {
                break;
            }

            if forwarding_count != 1 && rnote.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                print!("<NXm1End{} {}>", forwarding_count, rnote);
                break;
            }
        }

        if is_auto_reverse && !is_legal_touch {
            // 非合法タッチを自動で戻す。
            comm.println("Illegal, go opponent forcely!");
            cassette_tape.caret.turn_to_opponent();
            RpmCassetteTapeRecorder::read_n_notes_on_tape_forcely(
                forwarding_count,
                cassette_tape,
                position,
                ply,
                comm,
            );
            cassette_tape.caret.turn_to_opponent();

            return false;
        }

        // 1つ以上読んでいれば合法。
        forwarding_count > 0
    }

    pub fn go_n_move_on_tape_forcely(
        repeats: i16,
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) {
        for _i in 0..repeats {
            RpmCassetteTapeRecorder::go_1move_on_tape_forcely(cassette_tape, position, ply, comm);
        }
    }

    /// 必ず1手進める。（非合法タッチがあれば強制終了）
    pub fn go_1move_on_tape_forcely(
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) {
        let mut is_legal_touch = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら終了。
        while let Some(rnote) = cassette_tape.go_1note_forcely(comm) {
            comm.println(&format!(
                "<NXm1:{}>",
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch =
                RpmCassetteTapeRecorder::try_1note_on_1note(&rnote, position, ply, comm);
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
    pub fn read_n_notes_on_tape_forcely(
        repeat: u8,
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) {
        for i in 0..repeat {
            if let Some(rnote) = cassette_tape.go_1note_forcely(comm) {
                comm.println(&format!("<Go-force:{}/{} {}>", i, repeat, rnote));
                RpmCassetteTapeRecorder::try_1note_on_1note(&rnote, position, ply, comm);
            } else {
                panic!("<Go forcely fail:{}/{} None>", i, repeat);
            }
        }
    }
}
