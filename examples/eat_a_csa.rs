extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
extern crate rand;
use getopts::Options;
use std::env;

use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_config::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_lib_config::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_player::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_record::*;
use kifuwarabe_wcsc29_lib::kifu_rpm::object::rpm_cassette_tape_box_conveyor::*;
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

    // Model.
    let mut position = Position::default();
    let crecord = CsaRecord::load(&path);

    // Play out.
    let recorder = CsaPlayer::play_out_and_record(&comm, &mut position, &crecord);
    HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);

    // Save.
    let mut tape_box_conveyor = RpmCassetteTapeBoxConveyor::new_empty();
    tape_box_conveyor.write_cassette_tape(
        &kw29_config,
        position.get_board_size(),
        &recorder.cassette_tape,
        &comm,
    );

    comm.println("Finished.");
}
