extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
extern crate rand;
use getopts::Options;
use rand::Rng;
use std::env;
use std::path::Path;

use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_config::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_lib_config::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_player::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_record::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::position::*;
use kifuwarabe_wcsc29_lib::rpm_conv::rpm_object_sheet::*;

#[derive(Debug)]
struct Args {
    path: Option<String>,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "path", "set input csa file name.", "NAME");

    let matches = opts
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    Args {
        path: matches.opt_str("path"),
    }
}

pub fn main() {
    // Command line arguments.
    let args = parse_args();

    // Logging.
    let comm = Communication::new();
    let path = args.path.unwrap();
    comm.println(&format!("args.path = '{}'.", path));

    // Config.
    let my_config = KifuwarabeWcsc29LibConfig::load();
    let kw29_config = KifuwarabeWcsc29Config::load(&my_config);

    // ファイル名をランダムに作成する。
    let rpm_object_sheet_path;
    {
        let mut rng = rand::thread_rng();
        let rand1: i64 = rng.gen();
        let rand2: i64 = rng.gen();
        let rand3: i64 = rng.gen();
        let rand4: i64 = rng.gen();
        rpm_object_sheet_path = Path::new(&kw29_config.learning)
            .join(format!(
                "{}-{}-{}-{}-learning.rpmove",
                rand1, rand2, rand3, rand4
            ))
            .to_str()
            .unwrap()
            .to_string();
    }

    // Model.
    let mut position = Position::default();
    let crecord = CsaRecord::load(&path);

    // Play.
    let mut recorder = CsaPlayer::play_out_and_record(&comm, &mut position, &crecord);
    HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);

    // Save.
    let rpm_sheet = RpmObjectSheet::default(&rpm_object_sheet_path);
    rpm_sheet.append_record(&comm, position.get_board_size(), &recorder);

    comm.println("Finished.");
}
