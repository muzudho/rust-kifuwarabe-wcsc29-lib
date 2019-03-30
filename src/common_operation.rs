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
    pub fn pop(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) {
        if let Some(pmove) = precord.pop() {
            position.touch(comm, &pmove);
        }
        CommonOperation::bo(comm, &precord, &position);
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
        loop {
            if let Some(pmove) = CommonOperation::back_1mark(comm, precord, position) {
                if pmove.is_phase_change() {
                    // フェーズ切り替えしたら終了。
                    break;
                }

                // それ以外は繰り返す。
            } else {
                // 開始前に達したら終了。
                break
            }
        }
    }

    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。
    pub fn forward(comm:&Communication, precord:&mut PhysicalRecord, position:&mut Position) {
        if precord.forward() {
            if let Some(pmove) = precord.get_current() {
                position.touch(comm, &pmove);
            }
        }
        CommonOperation::bo(comm, &precord, &position);
    }
}