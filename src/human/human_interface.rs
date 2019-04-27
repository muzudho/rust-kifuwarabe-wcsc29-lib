use communication::*;
use object_rpm::cassette_tape::*;
use position::*;
use std::*;

pub struct HumanInterface {}
impl HumanInterface {
    /// 局面だけ表示。
    pub fn show_position(comm: &Communication, ply: i16, position: &Position) {
        // 何手目か。
        comm.println(&format!("[{}]", ply));
        // 盤面。
        comm.println(&position.to_text(comm, position.get_phase(), position.get_board_size()));
    }

    /// 局面と棋譜の表示。
    pub fn bo(comm: &Communication, cassette_tape: &CassetteTape, ply: i16, position: &Position) {
        // 局面。
        HumanInterface::show_position(comm, ply, position);

        // 棋譜。
        let (_numbers, operations) = &cassette_tape.to_sign(position.get_board_size());
        comm.println(operations);
    }
}
