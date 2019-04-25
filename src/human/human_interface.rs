use communication::*;
use kifu_rpm::object::rpm_cassette_tape::*;
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
    pub fn bo(
        comm: &Communication,
        cassette_tape: &RpmCassetteTape,
        ply: i16,
        position: &Position,
    ) {
        // 局面。
        HumanInterface::show_position(comm, ply, position);

        // 棋譜。
        let (_numbers, operations) = &cassette_tape.to_sign(position.get_board_size());
        comm.println(operations);

        /*
        // デバッグ。
        comm.println(&format!(
            "#Append record: 確認用: {}",
            cassette_tape.to_human_presentable(position.get_board_size())
        ));
        comm.println(&format!(
            "#Append record: セーブ用の内容: {}",
            cassette_tape.to_json_object(position.get_board_size())
        ));
        */
    }
}
