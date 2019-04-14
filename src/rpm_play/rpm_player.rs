use communication::*;
use position::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_conv::rpm_record::*;
use std::*;
//use communication::*;
//use position::*;
//use rpm_conv::thread::rpm_note_operation::*;
//use rpm_conv::rpm_record::*;

pub struct RpmPlayer {
}
impl RpmPlayer {
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_beautiful_world(comm:&Communication, rpm_record:&mut RpmRecord, rpm_note:&RpmNoteOpe, position:&mut Position) {
        let piece_id_number = if let (_is_legal_touch, Some(piece_identify)) = position.touch_world(comm, &rpm_note) {
            piece_identify.get_id().get_number()
        } else {
            -1
        };
        rpm_record.add_note(&rpm_note, piece_id_number);
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_current_1mark(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        let mut cursor_clone = rpm_record.body.cursor; // .clone();
        if let Some(rpm_note) = rpm_record.body.operation_track.pop_current(&mut rpm_record.body.cursor, &mut rpm_record.body.ply) {
            rpm_record.body.identify_track.pop_current(&mut cursor_clone);

            let (_is_legal_touch, _piece_identify_opt) = position.touch_world(comm, &rpm_note);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手削除する。
    pub fn pop_current_1ply(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = RpmPlayer::pop_current_1mark(comm, rpm_record, position) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。
    /// 動かせなかったなら、Noneを返す。
    pub fn forward_1note(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        if rpm_record.forward() {
            if let Some(rpm_note) = rpm_record.body.operation_track.get_current(rpm_record.body.cursor) {
                let (is_legal_touch, _piece_identify_opt) = position.touch_world(comm, &rpm_note);

                // 非合法タッチなら戻す。
                if is_legal_touch {
                    Some(rpm_note)
                } else {
                    // もう１回タッチすれば戻る。（トグル式なんで）
                    position.touch_world(comm, &rpm_note);
                    None
                }
            } else {
                panic!("Unexpected forward 1 note.")
            }
        } else {
            None
        }
    }

    /// 1手進める。
    pub fn forward_1ply(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) {
        let mut is_first = true;
        // 最後尾に達していたのなら動かさず終了。
        while let Some(rpm_note) = RpmPlayer::forward_1note(comm, rpm_record, position) {
            if !is_first && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            is_first = false;
        }
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。
    pub fn back_1note(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        if let Some(rpm_note) = rpm_record.body.operation_track.get_current(rpm_record.body.cursor) {
            let (_is_legal_touch, _piece_identify_opt) = position.touch_world(comm, &rpm_note);
            rpm_record.back();
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手戻す。
    pub fn back_1ply(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = RpmPlayer::back_1note(comm, rpm_record, position) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }
}
