use common_operation::*;
use communication::*;
use kif_conv::kif_player::*;
use kif_conv::kif_record::*;
use conf::kifuwarabe_wcsc29_config::*;
use conf::kifuwarabe_wcsc29_lib_config::*;
use rpm_conv::rpm_record::*;
use rpm_conv::rpm_sheet::*;
use position::*;
use std::fs;
use std::path::Path;

pub struct KifConverter {
}
impl KifConverter {
    pub fn convert_kif(input_path:&str, output_path:&str)
    {
        // Logging.
        let comm = Communication::new();
        comm.println(&format!("input_path: {}", input_path));
        comm.println(&format!("output_path: {}", output_path));

        // Config.
        let my_config = KifuwarabeWcsc29LibConfig::load();
        let kw29_config = KifuwarabeWcsc29Config::load(&my_config);

        // Directory.
        let out_path = Path::new(output_path);
        let out_dir = out_path.parent().unwrap();
        match fs::create_dir_all(out_dir) {
            Ok(x) => {},
            Err(err) => panic!("Directory create fail: {}", err),
        }

        let eating_dir = kw29_config.learning;
        match fs::create_dir_all(&eating_dir) {
            Ok(x) => {},
            Err(err) => panic!("Directory create fail: {}", err),
        }

        // Model.
        let mut rrecord = RpmRecord::default();
        let mut position = Position::default();
        let krecord = KifRecord::load(&input_path);

        // Play.
        KifPlayer::play_out_record(&comm, &mut position, &krecord, &mut rrecord);
        CommonOperation::bo(&comm, &rrecord.body.operation_track, &position);

        // Save.
        let rpm_sheet = RpmSheet::default(output_path);
        rpm_sheet.append(&comm, position.get_board_size(), &eating_dir, &rrecord);

        comm.println("Finished.");
    }
}
