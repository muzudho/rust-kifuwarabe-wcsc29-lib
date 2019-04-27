use communication::*;
use kifu_rpm::play::rpm_move_player::*;
// use kifu_rpm::recorder::rpm_cassette_tape_recorder::*;
use kifu_rpm::object::rpm_cassette_tape::*;
use position::*;

pub struct RpmThreadPlayer {}
impl RpmThreadPlayer {
    /// 棋譜の上を進めます。
    pub fn go_next_n_repeats(
        repeats: i16,
        ply: i16,
        cassette_tape: &mut RpmCassetteTape,
        // recorder: &mut RpmCassetteTapeRecorder,
        position: &mut Position,
        comm: &Communication,
    ) {
        for _i in 0..repeats {
            RpmMovePlayer::go_next_1_move(cassette_tape, position, ply, false, &comm);
        }
    }
}
