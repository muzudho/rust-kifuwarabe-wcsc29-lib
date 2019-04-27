extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
extern crate regex;

use getopts::Options;
use std::env;

use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_config::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_lib_config::*;
use kifuwarabe_wcsc29_lib::kifu_kif::kif_converter::*;
use kifuwarabe_wcsc29_lib::kifu_rpm::object::rpm_cassette_tape_box_conveyor::*;

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
    let my_conf = KifuwarabeWcsc29LibConfig::load();
    let kw29_conf = KifuwarabeWcsc29Config::load(&my_conf);

    // Record.
    let mut tape_box_conveyer = RpmCassetteTapeBoxConveyor::new_empty();
    tape_box_conveyer.choice_box_manually("sheet.txt");

    KifConverter::convert_kif(&kw29_conf, &path, &mut tape_box_conveyer, &comm);
}
