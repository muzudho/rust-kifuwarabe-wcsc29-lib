use communication::*;
use human::human_interface::*;
use kifu_rpm::object::rpm_cassette_tape::*;
use kifu_rpm::recorder::rpm_cassette_tape_recorder::*;
use kifu_rpm::thread::rpm_note::*;
use kifu_rpm::thread::rpm_note_operation::*;
use piece_etc::*;
use position::*;
use std::*;

pub struct RpmNotePlayer {}
impl RpmNotePlayer {
    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_brandnew_note(
        recorder: &mut RpmCassetteTapeRecorder,
        // ノートの中に Ply もある☆（＾～＾）
        rnote_ope: &RpmNoteOpe,
        position: &mut Position,
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
        recorder.record_note(rnote, comm);
        /*
        comm.println(&format!(
            "End     :Recorder: {}.",
            recorder.to_human_presentable(board_size)
        ));
        comm.println(&format!(
            "End     :Recorder json: {}.",
            recorder.cassette_tape.to_json(board_size)
        ));
        */
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_1note(
        recorder: &mut RpmCassetteTapeRecorder,
        position: &mut Position,
        comm: &Communication,
    ) -> Option<RpmNote> {
        comm.println("pop_1note");
        HumanInterface::show_position(comm, -1, position);

        if let Some(rpm_note) = recorder.delete() {
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
                RpmNotePlayer::go_1note(&rnote, position, ply, comm);
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
}
