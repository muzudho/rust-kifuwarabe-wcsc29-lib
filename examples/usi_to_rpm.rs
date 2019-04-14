extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;
use std::env;
use getopts::Options;

use kifuwarabe_wcsc29_lib::common_operation::*;
use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::usi_conv::fen::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_player::*;
//use kifuwarabe_wcsc29_lib::usi_conv::usi_move::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_record::*;
use kifuwarabe_wcsc29_lib::rpm_operation_track::*;
use kifuwarabe_wcsc29_lib::position::*;

//use std::fs::File;
//use std::io::{BufRead, BufReader};

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

    let mut rpm_o_track = RpmOTrack::default();
    let mut position = Position::default();

    let line = UsiRecord::read_first_line(&comm, &path);

    comm.println(&format!("Parse line: `{}`.", line));
    let mut urecord = UsiRecord::new();
        
    let mut start = 0;
    if Fen::parse_position(&comm, &line, &mut start, &mut position) {
        comm.println("Position parsed.");

        if let Some(parsed_urecord) = CommonOperation::parse_usi_1record(&comm, &line, &mut start, position.get_board_size()) {
            comm.println("Moves parsed.");
            urecord = parsed_urecord;
        };
    }
    comm.println("Created urecord.");

    // ポジションをもう１回初期局面に戻す。
    let mut start = 0;
    if Fen::parse_position(&comm, &line, &mut start, &mut position) {
        comm.println("Position parsed.");
    }

    UsiConverter::convert_record(&comm, &mut position, &urecord, &mut rpm_o_track);
    CommonOperation::bo(&comm, &rpm_o_track, &position);
}
