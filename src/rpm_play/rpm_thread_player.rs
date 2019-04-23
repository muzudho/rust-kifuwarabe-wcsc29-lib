use communication::*;
use position::*;
use rpm_conv::rpm_cassette_tape_recorder::*;
use rpm_play::rpm_move_player::*;

pub struct RpmThreadPlayer {}
impl RpmThreadPlayer {
    pub fn get_n_move_and_go(
        repeats: i16,
        recorder: &mut RpmCassetteTapeRecorder,
        position: &mut Position,
        comm: &Communication,
    ) {
        for _i in 0..repeats {
            RpmMovePlayer::get_1move_and_go(
                &mut recorder.cassette_tape,
                position,
                recorder.ply,
                false,
                &comm,
            );
        }
    }
}
