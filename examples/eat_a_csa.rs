extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;
use std::env;
use getopts::Options;

use kifuwarabe_wcsc29_lib::common_operation::*;
use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::config_file::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_move::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_player::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_record::*;
use kifuwarabe_wcsc29_lib::rpm_conv::rpm_operation_track::*;
use kifuwarabe_wcsc29_lib::rpm_conv::rpm_record::*;
use kifuwarabe_wcsc29_lib::rpm_conv::rpm_sheet::*;
use kifuwarabe_wcsc29_lib::position::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Args {
    path: Option<String>,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "path", "set input csa file name.", "NAME");

    let matches = opts.parse(&args[1..])
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
    let config = &Config::load();

    // Model.
    let mut rrecord = RpmRecord::default();
    let mut position = Position::default();
    let crecord = CsaRecord::load(&path);

    // Play.
    CsaPlayer::play_out_record(&comm, &mut position, &crecord, &mut rrecord);
    HumanInterface::bo(&comm, &rrecord.body.operation_track, &position);

    // Save.
    let rpm_sheet = RpmSheet::new();
    let dir = &config.my_record_directory;
    rpm_sheet.append(&comm, position.get_board_size(), &dir, &mut rrecord);

    comm.println("Finished.");
}
