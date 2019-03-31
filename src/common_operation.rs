use communication::*;
use position::*;
use physical_move::*;
use physical_record::*;

pub struct CommonOperation {
}
impl CommonOperation {
    pub fn go(comm:&Communication, precord:&mut PhysicalRecord, pmove:&PhysicalMove, position:&mut Position) {
        precord.add(&pmove);
        position.touch(comm, &pmove);
    }

    /// 局面表示。
    pub fn bo(comm:&Communication, precord:&PhysicalRecord, position:&Position) {
        // 何手目か。
        comm.println(&format!("[{}]", precord.get_ply()));
        // 盤面。
        comm.println(&position.to_text(comm, position.get_phase()));
        // 棋譜。
        comm.println(&precord.to_sign(position.get_board_size()));
    }

    pub fn touch(comm:&Communication, precord:&mut PhysicalRecord, pmove:&PhysicalMove, position:&mut Position) {
        CommonOperation::go(comm, precord, pmove, position);
        CommonOperation::bo(comm, &precord, &position);
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_current_1mark(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) -> Option<PhysicalMove> {
        if let Some(pmove) = precord.pop_current() {
            position.touch(comm, &pmove);
            Some(pmove)
        } else {
            None
        }
    }

    /// 1手削除する。
    pub fn pop_current_1ply(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(pmove) = CommonOperation::pop_current_1mark(comm, precord, position) {
            if count != 0 && pmove.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。
    pub fn back_1mark(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) -> Option<PhysicalMove> {
        if let Some(pmove) = precord.get_current() {
            position.touch(comm, &pmove);
            precord.back();
            Some(pmove)
        } else {
            None
        }
    }

    /// 1手戻す。
    pub fn back_1ply(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(pmove) = CommonOperation::back_1mark(comm, precord, position) {
            if count != 0 && pmove.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。
    pub fn forward_1mark(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) -> Option<PhysicalMove> {
        if precord.forward() {
            if let Some(pmove) = precord.get_current() {
                position.touch(comm, &pmove);
                return Some(pmove)
            } else {
                panic!("Unexpected forward 1mark.")
            }
        } else {
            None
        }
    }

    /// 1手進める。
    pub fn forward_1ply(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) {
        let mut count = 0;
        // 最後尾に達していたのなら終了。
        while let Some(pmove) = CommonOperation::forward_1mark(comm, precord, position) {
            if count != 0 && pmove.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }
}