use communication::*;
use position::*;
use rpm_conv::rpm_record::*;
use std::*;

pub struct HumanInterface {
}
impl HumanInterface {
    /// 局面だけ表示。
    pub fn show_position(comm:&Communication, ply:i16, position:&Position) {
        // 何手目か。
        comm.println(&format!("[{}]", ply));
        // 盤面。
        comm.println(&position.to_text(comm, position.get_phase()));
    }

    /// 局面と棋譜の表示。
    pub fn bo(comm:&Communication, rpm_record:&RpmRecord, position:&Position) {
        // 局面。
        HumanInterface::show_position(comm, rpm_record.body.ply, position);
        // 棋譜。
        let mut unused_ply = 0;
        comm.println(&rpm_record.body.operation_track.to_sign(position.get_board_size(), &mut unused_ply));
    }
}
