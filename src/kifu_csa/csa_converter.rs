use communication::*;
use kifu_csa::csa_player::*;
use kifu_csa::csa_record::*;
use kifu_rpm::rpm_object_sheet::*;
use position::*;
use std::fs;
use std::path::Path;

pub struct CsaConverter {}
impl CsaConverter {
    pub fn convert_csa(input_path: &str, output_path: &str) {
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
        let crecord = CsaRecord::load(&input_path);

        // Play.
        let recorder = CsaPlayer::play_out_and_record(&comm, &mut position, &crecord);
        // HumanInterface::bo(&comm, &rrecord.body.operation_track, &position);

        // Save. (Append)
        let rpm_object_sheet = RpmObjectSheet::default(output_path);
        rpm_object_sheet.append_record(&comm, position.get_board_size(), &recorder);

        // comm.println("Finished.");
    }
}
