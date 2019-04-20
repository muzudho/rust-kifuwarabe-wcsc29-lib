use communication::*;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_record::*;
use rpm_conv::rpm_tape::*;
use rpm_conv::thread::rpm_note::*;
use rpm_conv::thread::rpm_note_operation::*;
use std::*;

pub struct RpmNotePlayer {

}
impl RpmNotePlayer {
    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_brandnew_note(rtape:&mut RpmTape, rpm_note_ope:&RpmNoteOpe, position:&mut Position, comm:&Communication) {
        let board_size = position.get_board_size();
        let pid_opt = if let (_is_legal_touch, Some(piece_identify)) = position.touch_beautiful_1note(&rpm_note_ope, comm, board_size) {
            PieceIdentify::from_number(piece_identify.get_id().get_number())
        } else {
            None
        };

        rtape.record_next_note(RpmNote::from_id_ope(pid_opt, *rpm_note_ope));
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_current_1note_on_record(rpm_record:&mut RpmRecord, position:&mut Position, comm:&Communication) -> Option<RpmNote> {
        if let Some(rpm_note) = rpm_record.body.rpm_tape.delete_next(&mut rpm_record.body.ply) {
            let board_size = position.get_board_size();
            let (_is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(&rpm_note.get_ope(), comm, board_size);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 既存の棋譜を前方に移動するだけ。
    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。
    /// 動かせなかったなら、Noneを返す。
    /// 
    /// # Return
    /// 
    /// 合法タッチか否か。
    pub fn forward_1note(rnote:&RpmNote, position:&mut Position, ply:&mut i16, comm:&Communication) -> bool {
        let board_size = position.get_board_size();
        let (is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(&rnote.get_ope(), comm, board_size);

        if is_legal_touch {
            print!("[F{}]", ply);
            *ply += 1;
            true
        } else {
            print!("<IL-FN{}>", ply);
            // 非合法タッチなら戻す。
            // もう１回タッチすれば戻る。（トグル式なんで）
            position.touch_beautiful_1note(&rnote.get_ope(), comm, board_size);
            false
        }
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。
    /// 
    /// # Return
    /// 
    /// 合法タッチか否か。
    pub fn back_1note(rnote:&RpmNote, position:&mut Position, ply:&mut i16, comm:&Communication) -> bool {
        let board_size = position.get_board_size();
        let (is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(&rnote.get_ope(), comm, board_size);

        if is_legal_touch {
            print!("[B{}]", ply);
            *ply -= 1;
            true
        } else {
            // 非合法タッチなら戻す。
            // もう１回タッチすれば戻る。（トグル式なんで）
            position.touch_beautiful_1note(&rnote.get_ope(), comm, board_size);
            false
        }
    }
}
