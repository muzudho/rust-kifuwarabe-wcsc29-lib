use communication::*;
use csa_conv::csa_player::*;
use csa_conv::csa_record::*;
use rpm_conv::rpm_record::*;
use rpm_conv::rpm_object_sheet::*;
use position::*;
use std::fs;
use std::path::Path;

pub struct CsaConverter {
}
impl CsaConverter {
    pub fn convert_csa(input_path:&str, output_path:&str)
    {
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
            Ok(_x) => {},
            Err(err) => panic!("Directory create fail: {}", err),
        }

        // Model.
        let mut rrecord = RpmRecord::default();
        let mut position = Position::default();
        let crecord = CsaRecord::load(&input_path);

        // Play.
        CsaPlayer::play_out_record(&comm, &mut position, &crecord, &mut rrecord);
        // HumanInterface::bo(&comm, &rrecord.body.operation_track, &position);

        // Save. (Append)
        let rpm_object_sheet = RpmObjectSheet::default(output_path);
        rpm_object_sheet.append(&comm, position.get_board_size(), &rrecord);

        // comm.println("Finished.");
    }
}
