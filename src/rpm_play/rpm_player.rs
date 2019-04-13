use communication::*;
// use parser::*;
use position::*;
use rpm_conv::thread::rpm_operation_note::*;
// use rpm_conv::rpm_operation_track::*;
use rpm_conv::rpm_record::*;
use std::*;
// use usi_conv::usi_record::*;

pub struct RpmPlayer {
}
impl RpmPlayer {
    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。
    /// 動かせなかったなら、Noneを返す。
    pub fn forward_1note(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmOpeNote> {
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
}
