//use address::*;
//use board_size::*;
use communication::*;
//use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_conv::rpm_record::*;
use std::*;

pub struct RpmNotePlayer {

}
impl RpmNotePlayer {
    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_brandnew_note(comm:&Communication, rrecord:&mut RpmRecord, rpm_note_ope:&RpmNoteOpe, position:&mut Position) {
        let piece_id_number = if let (_is_legal_touch, Some(piece_identify)) = position.touch_beautiful_1note(comm, &rpm_note_ope) {
            piece_identify.get_id().get_number()
        } else {
            -1
        };
        rrecord.add_note(&rpm_note_ope, piece_id_number);
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_current_1note_on_record(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        let mut cursor_clone = rpm_record.body.cursor; // .clone();
        if let Some(rpm_note) = rpm_record.body.operation_track.pop_current(&mut rpm_record.body.cursor, &mut rpm_record.body.ply) {
            rpm_record.body.identify_track.pop_current(&mut cursor_clone);

            let (_is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(comm, &rpm_note);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 既存の棋譜を前方に移動するだけ。
    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。
    /// 動かせなかったなら、Noneを返す。
    pub fn forward_1note_on_record(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        if rpm_record.forward() {
            if let Some(rpm_note) = rpm_record.body.operation_track.get_current(rpm_record.body.cursor) {
                let (is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(comm, &rpm_note);

                // 非合法タッチなら戻す。
                if is_legal_touch {
                    Some(rpm_note)
                } else {
                    // もう１回タッチすれば戻る。（トグル式なんで）
                    position.touch_beautiful_1note(comm, &rpm_note);
                    None
                }
            } else {
                panic!("Unexpected forward 1 note.")
            }
        } else {
            None
        }
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。
    pub fn back_1note_on_record(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        if let Some(rpm_note) = rpm_record.body.operation_track.get_current(rpm_record.body.cursor) {
            let (_is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(comm, &rpm_note);
            rpm_record.back();
            Some(rpm_note)
        } else {
            None
        }
    }
}
