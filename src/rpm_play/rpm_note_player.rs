use communication::*;
use human::human_interface::*;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_cassette_tape::*;
use rpm_conv::rpm_cassette_tape_recorder::*;
use rpm_conv::thread::rpm_note::*;
use rpm_conv::thread::rpm_note_operation::*;
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

        comm.println("End#touch_brandnew_note");
        HumanInterface::show_position(comm, recorder.ply, position);
        recorder.record_next_note(RpmNote::from_id_ope(pid_opt, *rnote_ope));
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_current_1note_on_record(
        recorder: &mut RpmCassetteTapeRecorder,
        position: &mut Position,
        comm: &Communication,
    ) -> Option<RpmNote> {
        comm.println("pop_current_1note_on_record");
        HumanInterface::show_position(comm, -1, position);

        if let Some(rpm_note) = recorder.delete_next() {
            let board_size = position.get_board_size();
            let (_is_legal_touch, _piece_identify_opt) =
                position.touch_beautiful_1note(&rpm_note.get_ope(), comm, board_size);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 既存の棋譜を前方に移動するだけ。
    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。（非合法タッチでも行います）
    /// 動かせなかったなら、Noneを返す。
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn next_1note(
        rnote: &RpmNote,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) -> bool {
        let board_size = position.get_board_size();

        comm.println(&format!(
            "<NXn:{}>",
            rnote.get_ope().to_human_presentable(board_size)
        ));
        let (is_legal_touch, _piece_identify_opt) =
            position.touch_beautiful_1note(&rnote.get_ope(), comm, board_size);
        HumanInterface::show_position(comm, ply, position);

        is_legal_touch
    }

    /// 非合法手はない前提で、強制的に巻き戻します。
    pub fn back_n_note_forcely(
        repeat: u8,
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) {
        for i in 0..repeat {
            if let Some(rnote) = cassette_tape.back_note() {
                comm.println(&format!("<Back-force:{}/{} {}>", i, repeat, rnote));
                RpmNotePlayer::back_1note(&rnote, position, ply, comm);
            } else {
                panic!("<Back forcely fail:{}/{} None>", i, repeat);
            }
        }
    }

    /// 非合法手はない前提で、強制的に巻き進めます。
    pub fn next_n_note_forcely(
        repeat: u8,
        cassette_tape: &mut RpmCassetteTape,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) {
        for i in 0..repeat {
            if let Some(rnote) = cassette_tape.next_note() {
                comm.println(&format!("<Next-force:{}/{} {}>", i, repeat, rnote));
                RpmNotePlayer::back_1note(&rnote, position, ply, comm);
            } else {
                panic!("<Next forcely fail:{}/{} None>", i, repeat);
            }
        }
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。（非合法タッチでも行います）
    ///
    /// # Return
    ///
    /// 合法タッチか否か。
    pub fn back_1note(
        rnote: &RpmNote,
        position: &mut Position,
        ply: i16,
        comm: &Communication,
    ) -> bool {
        let board_size = position.get_board_size();

        comm.println(&format!(
            "<BKn:{}>",
            rnote.get_ope().to_human_presentable(board_size)
        ));
        let (is_legal_touch, _piece_identify_opt) =
            position.touch_beautiful_1note(&rnote.get_ope(), comm, board_size);
        HumanInterface::show_position(comm, ply, position);

        is_legal_touch
    }
}
