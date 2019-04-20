extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
use getopts::Options;
use std::env;

use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::position::*;
use kifuwarabe_wcsc29_lib::rpm_conv::rpm_cassette_tape_recorder::*;
use kifuwarabe_wcsc29_lib::usi_conv::fen::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_player::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_position::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_record::*;

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

    let matches = opts
        .parse(&args[1..])
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

    let mut position = Position::default();

    let line = UsiRecord::read_first_line(&comm, &path);

    comm.println(&format!("Parse line: `{}`.", line));
    let mut urecord = UsiRecord::default();
    let mut recorder = RpmCassetteTapeRecorder::default();

    let mut start = 0;
    if Fen::parse_position(&comm, &line, &mut start, &mut recorder, &mut position) {
        comm.println("Position parsed.");

        if let Some(parsed_urecord) =
            UsiPosition::parse_usi_line_moves(&comm, &line, &mut start, position.get_board_size())
        {
            comm.println("Moves parsed.");
            urecord = parsed_urecord;
        };
    }
    comm.println("Created urecord.");

    // ポジションをもう１回初期局面に戻す。
    let mut start = 0;
    if Fen::parse_position(&comm, &line, &mut start, &mut recorder, &mut position) {
        comm.println("Position parsed.");
    }

    recorder = UsiPlayer::play_out_and_record(&comm, &mut position, &urecord);
    HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
}
