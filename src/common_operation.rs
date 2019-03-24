use communication::*;
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

    pub fn bo(comm:&Communication, physical_record:&PhysicalRecord, position:&Position) {
        comm.println(&position.to_text(position.get_phase()));
        // comm.print(&format!("Physical record({}): ", physical_record.len()));
        comm.println(&format!("{}", physical_record.to_sign(position.get_board_size())));
    }

    pub fn touch(comm:&Communication, physical_record:&mut PhysicalRecord, physical_move:&PhysicalMove, position:&mut Position) {
        CommonOperation::go(physical_record, physical_move, position);
        CommonOperation::bo(comm, &physical_record, &position);
    }

    pub fn detouch(comm:&Communication, physical_record:&mut PhysicalRecord, position:&mut Position) {
        if let Some(physical_move) = physical_record.pop() {
            position.touch(&physical_move);
        }
        CommonOperation::bo(comm, &physical_record, &position);
    }
}