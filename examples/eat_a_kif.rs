extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;
use std::env;
use getopts::Options;

use kifuwarabe_wcsc29_lib::common_operation::*;
use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_converter::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_move::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_record::*;
use kifuwarabe_wcsc29_lib::rpm_operation_track::*;
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

pub fn main()
{
    let args = parse_args();

    let comm = Communication::new();
    let path = args.path.unwrap();
    comm.println(&format!("args.path = '{}'.", path));

    let mut rpm_o_track = RpmOTrack::default();
    let mut position = Position::default();

}