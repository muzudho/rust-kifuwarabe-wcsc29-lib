extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;
use std::env;
use getopts::Options;

use kifuwarabe_wcsc29_lib::common_operation::*;
use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_converter::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_move::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_record::*;
use kifuwarabe_wcsc29_lib::physical_record::*;
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
    opts.optopt("p", "path", "set input usi file name.", "NAME");

    let matches = opts.parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    Args {
        path: matches.opt_str("path"),
    }
}

pub fn main() {
    let args = parse_args();

    let comm = Communication::new();
    let path = args.path.unwrap();
    comm.println(&format!("args.path = '{}'.", path));

    let mut precord = PhysicalRecord::default();
    let mut position = Position::default();

    let urecord = UsiRecord::load(&comm, &path); // ex.) "download-kifu/WCSC28_F6_PAL_HFW.usi"
    comm.println("Created urecord.");

    UsiConverter::convert_record(&comm, &mut position, &urecord, &mut precord);
    CommonOperation::bo(&comm, &precord, &position);
}
