use communication::*;
use conf::kifuwarabe_wcsc29_config::KifuwarabeWcsc29Config;
use kifu_kif::kif_player::*;
use kifu_kif::kif_record::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::*;
use position::*;

pub struct KifConverter {}
impl KifConverter {
    pub fn convert_kif(
        kw29_conf: &KifuwarabeWcsc29Config,
        input_path: &str,
        cassette_tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        comm: &Communication,
    ) {
        // comm.println(&format!("input_path: {}", input_path));

        // Model.
        let mut position = Position::default();
        let krecord = KifRecord::load(&input_path);

        // Play.
        let recorder = KifPlayer::play_out_and_record(&comm, &mut position, &krecord);
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
