use communication::*;
use position::*;
use physical_move::*;
use physical_record::*;

pub struct CommonOperation {
}
impl CommonOperation {
    pub fn go(comm:&Communication, physical_record:&mut PhysicalRecord, physical_move:&PhysicalMove, position:&mut Position) {
        physical_record.add(&physical_move);
        position.touch(comm, &physical_move);
    }

    pub fn bo(comm:&Communication, physical_record:&PhysicalRecord, position:&Position) {
        comm.println(&position.to_text(comm, position.get_phase()));
        // comm.print(&format!("Physical record({}): ", physical_record.len()));
        comm.println(&physical_record.to_sign(position.get_board_size()));
    }

    pub fn touch(comm:&Communication, physical_record:&mut PhysicalRecord, physical_move:&PhysicalMove, position:&mut Position) {
        CommonOperation::go(comm, physical_record, physical_move, position);
        CommonOperation::bo(comm, &physical_record, &position);
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop(comm:&Communication, physical_record:&mut PhysicalRecord, position:&mut Position) {
        if let Some(physical_move) = physical_record.pop() {
            position.touch(comm, &physical_move);
        }
        CommonOperation::bo(comm, &physical_record, &position);
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。
    pub fn back(comm:&Communication, physical_record:&mut PhysicalRecord, position:&mut Position) {
        if let Some(physical_move) = physical_record.get_current() {
            position.touch(comm, &physical_move);
            physical_record.back();
        }
        CommonOperation::bo(comm, &physical_record, &position);
    }

    /// 棋譜のカーソルを１つ進め、カーソルが指している要素をタッチする。
    pub fn forward(comm:&Communication, physical_record:&mut PhysicalRecord, position:&mut Position) {
        if physical_record.forward() {
            if let Some(physical_move) = physical_record.get_current() {
                position.touch(comm, &physical_move);
            }
        }
        CommonOperation::bo(comm, &physical_record, &position);
    }
}