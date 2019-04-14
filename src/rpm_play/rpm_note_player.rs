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
    pub fn touch_brandnew_note(comm:&Communication, rtape:&mut RpmTape, rpm_note_ope:&RpmNoteOpe, position:&mut Position) {
        let pid_opt = if let (_is_legal_touch, Some(piece_identify)) = position.touch_beautiful_1note(comm, &rpm_note_ope) {
            PieceIdentify::from_number(piece_identify.get_id().get_number())
        } else {
            None
        };

        let mut unused_ply = 0;
        rtape.add_note(RpmNote::from_id_ope(pid_opt, *rpm_note_ope), &mut unused_ply);
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_current_1note_on_record(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNote> {
        if let Some(rpm_note) = rpm_record.body.rpm_tape.pop_current(&mut rpm_record.body.ply) {
            let (_is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(comm, &rpm_note.get_ope());
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
    pub fn forward_1note(comm:&Communication, rnote:&RpmNote, position:&mut Position, ply:&mut i16) -> bool {
        // if rpm_record.forward() {
            //if let Some(rnote) = rpm_record.body.rpm_tape.get_current_note() {
                let (is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(comm, &rnote.get_ope());

                if is_legal_touch {
                    *ply += 1;
                    true
                } else {
                    // 非合法タッチなら戻す。
                    // もう１回タッチすれば戻る。（トグル式なんで）
                    position.touch_beautiful_1note(comm, &rnote.get_ope());
                    false
                }
            /*
            } else {
                panic!("Unexpected forward 1 note.")
            }
            */
        /*
        } else {
            (false, None)
        }
        */
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。
    /// 
    /// # Return
    /// 
    /// 合法タッチか否か。
    pub fn back_1note(comm:&Communication, rnote:&RpmNote, position:&mut Position, ply:&mut i16) -> bool {
        //if let Some(rnote) = rpm_record.body.rpm_tape.get_current_note() {
            let (is_legal_touch, _piece_identify_opt) = position.touch_beautiful_1note(comm, &rnote.get_ope());

            if is_legal_touch {
                *ply -= 1;
                // rpm_record.back();
                // Some(rnote)
                true
            } else {
                // 非合法タッチなら戻す。
                // もう１回タッチすれば戻る。（トグル式なんで）
                position.touch_beautiful_1note(comm, &rnote.get_ope());
                false
            }


        /*
        } else {
            None
        }
        */
    }
}
