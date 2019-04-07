use common_operation::*;
use communication::*;
use config_file::*;
use kif_conv::kif_player::*;
use kif_conv::kif_record::*;
use rpm_conv::rpm_record::*;
use rpm_conv::rpm_sheet::*;
use position::*;
use std::fs;

pub struct KifConverter {
}
impl KifConverter {
    pub fn convert_kif(input_path:&str, output_path:&str) -> RpmRecord
    {
        // Logging.
        let comm = Communication::new();

        // Config.
        let config = &Config::load();

        // Model.
        let mut rrecord = RpmRecord::default();
        let mut position = Position::default();
        let krecord = KifRecord::load(&input_path);

        // Play.
        KifPlayer::play_out_record(&comm, &mut position, &krecord, &mut rrecord);
        CommonOperation::bo(&comm, &rrecord.body.operation_track, &position);

        // Save.
        let rpm_sheet = RpmSheet::default(output_path);
        let dir = &config.my_record_directory;
        match fs::create_dir_all(dir) {
            Ok(x) => {},
            Err(err) => panic!("Directory create fail: {}", err),
        }
        rpm_sheet.append(&comm, position.get_board_size(), &dir, &mut rrecord);

        comm.println("Finished.");

        rrecord
    }
}
