use address::*;
use board_size::*;
use communication::*;
//use human::human_interface::*;
use kifu_rpm::object::rpm_cassette_tape::*;
use kifu_rpm::play::rpm_note_player::*;
use kifu_rpm::recorder::rpm_cassette_tape_recorder::*;
use kifu_rpm::thread::rpm_note_operation::*;
use piece_etc::*;
use position::*;
use std::*;

pub struct RpmMovePlayer {}
impl RpmMovePlayer {
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
    pub fn record_ohashi_starting(
        comm: &Communication,
        recorder: &mut RpmCassetteTapeRecorder,
        pos: &mut Position,
    ) {
        //use piece_etc::IdentifiedPiece;
        use piece_etc::Phase::*;
        use piece_etc::PieceIdentify::*;

        // 大橋流の順序にしてください。
        let bs = pos.get_board_size();
        let array: [(RpmNoteOpe, RpmNoteOpe, RpmNoteOpe); 40] = [
            RpmMovePlayer::init_note(-39, Second, 5, 1, K00, bs),
            RpmMovePlayer::init_note(-38, First, 5, 9, K01, bs),
            RpmMovePlayer::init_note(-37, Second, 4, 1, G02, bs),
            RpmMovePlayer::init_note(-36, First, 6, 9, G03, bs),
            RpmMovePlayer::init_note(-35, Second, 6, 1, G04, bs),
            RpmMovePlayer::init_note(-34, First, 4, 9, G05, bs),
            RpmMovePlayer::init_note(-33, Second, 3, 1, S06, bs),
            RpmMovePlayer::init_note(-32, First, 7, 9, S07, bs),
            RpmMovePlayer::init_note(-31, Second, 7, 1, S08, bs),
            RpmMovePlayer::init_note(-30, First, 3, 9, S09, bs),
            RpmMovePlayer::init_note(-29, Second, 2, 1, N10, bs),
            RpmMovePlayer::init_note(-28, First, 8, 9, N11, bs),
            RpmMovePlayer::init_note(-27, Second, 8, 1, N12, bs),
            RpmMovePlayer::init_note(-26, First, 2, 9, N13, bs),
            RpmMovePlayer::init_note(-25, Second, 1, 1, L14, bs),
            RpmMovePlayer::init_note(-24, First, 9, 9, L15, bs),
            RpmMovePlayer::init_note(-23, Second, 9, 1, L16, bs),
            RpmMovePlayer::init_note(-22, First, 1, 9, L17, bs),
            RpmMovePlayer::init_note(-21, Second, 2, 2, B18, bs),
            RpmMovePlayer::init_note(-20, First, 8, 8, B19, bs),
            RpmMovePlayer::init_note(-19, Second, 8, 2, R20, bs),
            RpmMovePlayer::init_note(-18, First, 2, 8, R21, bs),
            RpmMovePlayer::init_note(-17, Second, 5, 3, P22, bs),
            RpmMovePlayer::init_note(-16, First, 5, 7, P23, bs),
            RpmMovePlayer::init_note(-15, Second, 4, 3, P24, bs),
            RpmMovePlayer::init_note(-14, First, 6, 7, P25, bs),
            RpmMovePlayer::init_note(-13, Second, 6, 3, P26, bs),
            RpmMovePlayer::init_note(-12, First, 4, 7, P27, bs),
            RpmMovePlayer::init_note(-11, Second, 3, 3, P28, bs),
            RpmMovePlayer::init_note(-10, First, 7, 7, P29, bs),
            RpmMovePlayer::init_note(-9, Second, 7, 3, P30, bs),
            RpmMovePlayer::init_note(-8, First, 3, 7, P31, bs),
            RpmMovePlayer::init_note(-7, Second, 2, 3, P32, bs),
            RpmMovePlayer::init_note(-6, First, 8, 7, P33, bs),
            RpmMovePlayer::init_note(-5, Second, 8, 3, P34, bs),
            RpmMovePlayer::init_note(-4, First, 2, 7, P35, bs),
            RpmMovePlayer::init_note(-3, Second, 1, 3, P36, bs),
            RpmMovePlayer::init_note(-2, First, 9, 7, P37, bs),
            RpmMovePlayer::init_note(-1, Second, 9, 3, P38, bs),
            RpmMovePlayer::init_note(0, First, 1, 7, P39, bs),
        ];

        for element in array.iter() {
            comm.println("rpm_move_player.rs:record_ohashi_starting: touch_brandnew_note");
            RpmNotePlayer::touch_brandnew_note(recorder, &element.0, pos, comm);
            RpmNotePlayer::touch_brandnew_note(recorder, &element.1, pos, comm);
            RpmNotePlayer::touch_brandnew_note(recorder, &element.2, pos, comm);
        }
    }

    /// 1手削除する。
    pub fn pop_current_1move_on_record(
        recorder: &mut RpmCassetteTapeRecorder,
        position: &mut Position,
        comm: &Communication,
    ) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = RpmNotePlayer::pop_1note(recorder, position, comm) {
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
    pub fn get_1move_and_go(
        // next_1move_on_tape
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        is_auto_reverse: bool,
        comm: &Communication,
    ) -> bool {
        let mut is_legal_touch = true;
        let mut forwarding_count = 0;
        comm.println(&format!(
            "<NXm1:{}>",
            cassette_tape.to_human_presentable(position.get_board_size())
        ));

        // 最後尾に達していたのなら終了。
        while let Some(rnote) = cassette_tape.get_note_and_go_tape(comm) {
            comm.println(&format!(
                "<NXm2:{}:{}>",
                cassette_tape.to_human_presentable(position.get_board_size()),
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch = RpmNotePlayer::go_1note(&rnote, position, ply, comm);
            forwarding_count += 1;

            if is_auto_reverse && !is_legal_touch {
                break;
            }

            if forwarding_count != 1 && rnote.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                print!("<Fpc{} {}>", forwarding_count, rnote);
                break;
            }
        }

        if is_auto_reverse && !is_legal_touch {
            // 非合法タッチを自動で戻す。
            comm.println("Illegal, go back forcely!");
            cassette_tape.caret.turn_to_opponent();
            RpmNotePlayer::get_n_note_and_go_forcely(
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
}
