use common_operation::*;
use communication::*;
use config_file::*;
use csa_conv::csa_player::*;
use csa_conv::csa_record::*;
use rpm_conv::rpm_record::*;
use rpm_conv::rpm_sheet::*;
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
        comm.println(&format!("input_path: {}", input_path));
        comm.println(&format!("output_path: {}", output_path));

        // Config.
        let config = Config::load();

        // Directory.
        let out_path = Path::new(output_path);
        let out_dir = out_path.parent().unwrap();
        match fs::create_dir_all(out_dir) {
            Ok(x) => {},
            Err(err) => panic!("Directory create fail: {}", err),
        }

        let eating_dir = config.my_record_directory;
        match fs::create_dir_all(&eating_dir) {
            Ok(x) => {},
            Err(err) => panic!("Directory create fail: {}", err),
        }

        // Model.
        let mut rrecord = RpmRecord::default();
        let mut position = Position::default();
        let crecord = CsaRecord::load(&input_path);

        // Play.
        CsaPlayer::play_out_record(&comm, &mut position, &crecord, &mut rrecord);
        CommonOperation::bo(&comm, &rrecord.body.operation_track, &position);

        // Save.
        let rpm_sheet = RpmSheet::default(output_path);
        rpm_sheet.append(&comm, position.get_board_size(), &eating_dir, &mut rrecord);

        comm.println("Finished.");
    }
}
