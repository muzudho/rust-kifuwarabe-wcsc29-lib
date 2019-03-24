use board::*;
use physical_move::*;
use physical_record::*;

pub struct CommonOperation {
}
impl CommonOperation {
    pub fn go(physical_record:&mut PhysicalRecord, physical_move:&PhysicalMove, board:&mut Board) {
        physical_record.add(&physical_move);
        board.touch(&physical_move);
    }

    pub fn bo(physical_record:&PhysicalRecord, board:&Board) {
        board.println(physical_record.get_phase());
        physical_record.println(board.get_board_size());
    }

    pub fn touch(physical_record:&mut PhysicalRecord, physical_move:&PhysicalMove, board:&mut Board) {
        CommonOperation::go(physical_record, physical_move, board);
        CommonOperation::bo(&physical_record, &board);
    }

    pub fn detouch(physical_record:&mut PhysicalRecord, board:&mut Board) {
        if let Some(physical_move) = physical_record.pop() {
            board.touch(&physical_move);
        }
        CommonOperation::bo(&physical_record, &board);
    }
}