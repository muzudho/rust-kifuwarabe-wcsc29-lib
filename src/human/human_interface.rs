use communication::*;
use position::*;
use rpm_conv::rpm_record::*;
use std::*;

pub struct HumanInterface {}
impl HumanInterface {
    /// 局面だけ表示。
    pub fn show_position(comm: &Communication, ply: i16, position: &Position) {
        // 何手目か。
        comm.println(&format!("[{}]", ply));
        // 盤面。
        comm.println(&position.to_text(comm, position.get_phase()));
    }

    /// 局面と棋譜の表示。
    pub fn bo(comm: &Communication, rrecord: &RpmRecord, position: &Position) {
        // 局面。
        HumanInterface::show_position(comm, rrecord.body.ply, position);

        // 棋譜。
        let mut unused_ply = 0;
        let (_numbers, operations) = &rrecord
            .body
            .cassette_tape
            .to_sign(position.get_board_size(), &mut unused_ply);
        comm.println(operations);
    }
}
