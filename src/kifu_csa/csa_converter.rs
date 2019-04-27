use communication::*;
use conf::kifuwarabe_wcsc29_config::KifuwarabeWcsc29Config;
use kifu_csa::csa_player::*;
use kifu_csa::csa_record::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::*;
use position::*;

pub struct CsaConverter {}
impl CsaConverter {
    pub fn convert_csa(
        kw29_conf: &KifuwarabeWcsc29Config,
        input_path: &str,
        cassette_tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        comm: &Communication,
    ) {
        // comm.println(&format!("input_path: {}", input_path));

        // Model.
        let mut position = Position::default();
        let crecord = CsaRecord::load(&input_path);

        // Play.
        let recorder = CsaPlayer::play_out_and_record(&comm, &mut position, &crecord);
        // HumanInterface::bo(&comm, &rrecord.body.operation_track, &position);

        // Save. (Append)
        cassette_tape_box_conveyor.write_cassette_tape(
            &kw29_conf,
            position.get_board_size(),
            &recorder.cassette_tape,
            &comm,
        );

        // comm.println("Finished.");
    }
}
