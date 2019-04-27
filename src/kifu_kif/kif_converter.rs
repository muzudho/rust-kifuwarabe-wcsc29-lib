use communication::*;
use kifu_kif::kif_player::*;
use kifu_kif::kif_record::*;
use kifu_rpm::object::rpm_cassette_tape_box::*;
use position::*;
use std::fs;
use std::path::Path;

pub struct KifConverter {}
impl KifConverter {
    pub fn convert_kif(input_path: &str, output_path: &str) {
        // Logging.
        let comm = Communication::new();
        // comm.println(&format!("input_path: {}", input_path));
        // comm.println(&format!("output_path: {}", output_path));

        // Config.
        // let my_config = KifuwarabeWcsc29LibConfig::load();
        // let kw29_config = KifuwarabeWcsc29Config::load(&my_config);

        // Directory.
        let out_path = Path::new(output_path);
        let out_dir = out_path.parent().unwrap();
        match fs::create_dir_all(out_dir) {
            Ok(_x) => {}
            Err(err) => panic!("Directory create fail: {}", err),
        }

        // Model.
        let mut position = Position::default();
        let krecord = KifRecord::load(&input_path);

        // Play.
        let recorder = KifPlayer::play_out_and_record(&comm, &mut position, &krecord);
        // HumanInterface::bo(&comm, &rrecord.body.operation_track, &position);

        // Save. (Append)
        let rpm_object_sheet = RpmCassetteTapeBox::default(output_path);
        rpm_object_sheet.write_cassette_tape(
            position.get_board_size(),
            &recorder.cassette_tape,
            &comm,
        );

        // comm.println("Finished.");
    }
}
