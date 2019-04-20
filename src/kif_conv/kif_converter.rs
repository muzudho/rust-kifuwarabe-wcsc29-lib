use communication::*;
use kif_conv::kif_player::*;
use kif_conv::kif_record::*;
use position::*;
use rpm_conv::rpm_object_sheet::*;
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
        let mut recorder = KifPlayer::play_out_and_record(&comm, &mut position, &krecord);
        // HumanInterface::bo(&comm, &rrecord.body.operation_track, &position);

        // Save. (Append)
        let rpm_object_sheet = RpmObjectSheet::default(output_path);
        rpm_object_sheet.append_record(&comm, position.get_board_size(), &recorder);

        // comm.println("Finished.");
    }
}
