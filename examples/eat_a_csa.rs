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
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_player::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_record::*;
use kifuwarabe_wcsc29_lib::kifu_rpm::rpm_cassette_tape_box::*;
use kifuwarabe_wcsc29_lib::position::*;

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
        let rand1: u64 = rng.gen();
        let rand2: u64 = rng.gen();
        let rand3: u64 = rng.gen();
        let rand4: u64 = rng.gen();
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

    // Play out.
    let recorder = CsaPlayer::play_out_and_record(&comm, &mut position, &crecord);
    HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);

    // Save.
    let rpm_sheet = RpmCassetteTapeBox::default(&rpm_object_sheet_path);
    rpm_sheet.append_cassette_tape(&comm, position.get_board_size(), &recorder.cassette_tape);

    comm.println("Finished.");
}
