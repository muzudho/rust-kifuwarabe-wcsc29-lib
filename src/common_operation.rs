use position::*;
use physical_move::*;
use physical_record::*;

pub struct CommonOperation {
}
impl CommonOperation {
    pub fn go(physical_record:&mut PhysicalRecord, physical_move:&PhysicalMove, position:&mut Position) {
        physical_record.add(&physical_move);
        position.touch(&physical_move);
    }

    pub fn bo(physical_record:&PhysicalRecord, position:&Position) {
        position.println(position.get_phase());
        physical_record.println(position.get_board_size());
    }

    pub fn touch(physical_record:&mut PhysicalRecord, physical_move:&PhysicalMove, position:&mut Position) {
        CommonOperation::go(physical_record, physical_move, position);
        CommonOperation::bo(&physical_record, &position);
    }

    pub fn detouch(physical_record:&mut PhysicalRecord, position:&mut Position) {
        if let Some(physical_move) = physical_record.pop() {
            position.touch(&physical_move);
        }
        CommonOperation::bo(&physical_record, &position);
    }
}