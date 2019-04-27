use address::*;
use board_size::*;
use communication::*;
use human::human_interface::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_recorder::*;
use kifu_rpm::object::rpm_cassette_tape::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::RpmCassetteTapeBoxConveyor;
use kifu_rpm::thread::rpm_note::*;
use kifu_rpm::thread::rpm_note_operation::*;
use piece_etc::*;
use position::*;
use std::*;

pub struct RpmCassetteTapePlayer {}
impl RpmCassetteTapePlayer {
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
        recorder: &mut RpmCassetteTapeRecorder,
        comm: &Communication,
    ) {
        use piece_etc::Phase::*;
        use piece_etc::PieceIdentify::*;

        // 大橋流の順序にしてください。
        let bs = pos.get_board_size();
        let array: [(RpmNoteOpe, RpmNoteOpe, RpmNoteOpe); 40] = [
            RpmCassetteTapePlayer::init_note(-39, Second, 5, 1, K00, bs),
            RpmCassetteTapePlayer::init_note(-38, First, 5, 9, K01, bs),
            RpmCassetteTapePlayer::init_note(-37, Second, 4, 1, G02, bs),
            RpmCassetteTapePlayer::init_note(-36, First, 6, 9, G03, bs),
            RpmCassetteTapePlayer::init_note(-35, Second, 6, 1, G04, bs),
            RpmCassetteTapePlayer::init_note(-34, First, 4, 9, G05, bs),
            RpmCassetteTapePlayer::init_note(-33, Second, 3, 1, S06, bs),
            RpmCassetteTapePlayer::init_note(-32, First, 7, 9, S07, bs),
            RpmCassetteTapePlayer::init_note(-31, Second, 7, 1, S08, bs),
            RpmCassetteTapePlayer::init_note(-30, First, 3, 9, S09, bs),
            RpmCassetteTapePlayer::init_note(-29, Second, 2, 1, N10, bs),
            RpmCassetteTapePlayer::init_note(-28, First, 8, 9, N11, bs),
            RpmCassetteTapePlayer::init_note(-27, Second, 8, 1, N12, bs),
            RpmCassetteTapePlayer::init_note(-26, First, 2, 9, N13, bs),
            RpmCassetteTapePlayer::init_note(-25, Second, 1, 1, L14, bs),
            RpmCassetteTapePlayer::init_note(-24, First, 9, 9, L15, bs),
            RpmCassetteTapePlayer::init_note(-23, Second, 9, 1, L16, bs),
            RpmCassetteTapePlayer::init_note(-22, First, 1, 9, L17, bs),
            RpmCassetteTapePlayer::init_note(-21, Second, 2, 2, B18, bs),
            RpmCassetteTapePlayer::init_note(-20, First, 8, 8, B19, bs),
            RpmCassetteTapePlayer::init_note(-19, Second, 8, 2, R20, bs),
            RpmCassetteTapePlayer::init_note(-18, First, 2, 8, R21, bs),
            RpmCassetteTapePlayer::init_note(-17, Second, 5, 3, P22, bs),
            RpmCassetteTapePlayer::init_note(-16, First, 5, 7, P23, bs),
            RpmCassetteTapePlayer::init_note(-15, Second, 4, 3, P24, bs),
            RpmCassetteTapePlayer::init_note(-14, First, 6, 7, P25, bs),
            RpmCassetteTapePlayer::init_note(-13, Second, 6, 3, P26, bs),
            RpmCassetteTapePlayer::init_note(-12, First, 4, 7, P27, bs),
            RpmCassetteTapePlayer::init_note(-11, Second, 3, 3, P28, bs),
            RpmCassetteTapePlayer::init_note(-10, First, 7, 7, P29, bs),
            RpmCassetteTapePlayer::init_note(-9, Second, 7, 3, P30, bs),
            RpmCassetteTapePlayer::init_note(-8, First, 3, 7, P31, bs),
            RpmCassetteTapePlayer::init_note(-7, Second, 2, 3, P32, bs),
            RpmCassetteTapePlayer::init_note(-6, First, 8, 7, P33, bs),
            RpmCassetteTapePlayer::init_note(-5, Second, 8, 3, P34, bs),
            RpmCassetteTapePlayer::init_note(-4, First, 2, 7, P35, bs),
            RpmCassetteTapePlayer::init_note(-3, Second, 1, 3, P36, bs),
            RpmCassetteTapePlayer::init_note(-2, First, 9, 7, P37, bs),
            RpmCassetteTapePlayer::init_note(-1, Second, 9, 3, P38, bs),
            RpmCassetteTapePlayer::init_note(0, First, 1, 7, P39, bs),
        ];

        for element in array.iter() {
            comm.println("rpm_move_player.rs:play_ohashi_starting: touch_brandnew_note");
            RpmCassetteTapePlayer::touch_brandnew_note(
                &element.0,
                pos,
                tape_box_conveyor,
                recorder,
                comm,
            );
            RpmCassetteTapePlayer::touch_brandnew_note(
                &element.1,
                pos,
                tape_box_conveyor,
                recorder,
                comm,
            );
            RpmCassetteTapePlayer::touch_brandnew_note(
                &element.2,
                pos,
                tape_box_conveyor,
                recorder,
                comm,
            );
        }
    }

    /// 1手削除する。
    pub fn pop_current_1move_on_record(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeRecorder,
        comm: &Communication,
    ) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) =
            RpmCassetteTapePlayer::pop_1note(position, tape_box_conveyor, recorder, comm)
        {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    /// 1手進める。（非合法タッチは自動で戻します）
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn go_next_1_move(
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        is_auto_reverse: bool,
        comm: &Communication,
    ) -> bool {
        let mut is_legal_touch = true;
        let mut forwarding_count = 0;
        comm.println("go_next_1_move.");

        // 最後尾に達していたのなら終了。
        while let Some(rnote) = cassette_tape.get_note_and_go_tape(comm) {
            comm.println(&format!(
                "<NXm1:{}>",
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch = RpmCassetteTapePlayer::go_1note(&rnote, position, ply, comm);
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
            RpmCassetteTapePlayer::get_n_note_and_go_forcely(
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

    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_brandnew_note(
        // ノートの中に Ply もある☆（＾～＾）
        rnote_ope: &RpmNoteOpe,
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeRecorder,
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
            "End     :touch_brandnew_note. Rnote: {}.",
            rnote.to_human_presentable(board_size)
        ));
         */
        recorder.record_note(rnote, tape_box_conveyor, comm);
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_1note(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeRecorder,
        comm: &Communication,
    ) -> Option<RpmNote> {
        comm.println("pop_1note");
        HumanInterface::show_position(comm, -1, position);

        if let Some(rpm_note) = recorder.delete(tape_box_conveyor) {
            let board_size = position.get_board_size();
            let (_is_legal_touch, _piece_identify_opt) =
                position.touch_beautiful_1note(&rpm_note.get_ope(), comm, board_size);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 非合法手はない前提で、強制的に巻き進めます。
    pub fn get_n_note_and_go_forcely(
        repeat: u8,
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) {
        for i in 0..repeat {
            if let Some(rnote) = cassette_tape.get_note_and_go_tape(comm) {
                comm.println(&format!("<Go-force:{}/{} {}>", i, repeat, rnote));
                RpmCassetteTapePlayer::go_1note(&rnote, position, ply, comm);
            } else {
                panic!("<Go forcely fail:{}/{} None>", i, repeat);
            }
        }
    }

    /// 指定のノートを実行（タッチ）するだけ。（非合法タッチでも行います）
    /// Next も Back も違いはない。キャレットは使わない。
    /// 動かせなかったなら、Noneを返す。
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn go_1note(
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
    pub fn go_next_n_repeats(
        repeats: i16,
        ply: i16,
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        comm: &Communication,
    ) {
        for _i in 0..repeats {
            RpmCassetteTapePlayer::go_next_1_move(cassette_tape, position, ply, false, &comm);
        }
    }
}
