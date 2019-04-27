extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
use getopts::Options;
use kifuwarabe_wcsc29_lib::application::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::kifu_rpm::recorder::rpm_cassette_tape_recorder::*;
use kifuwarabe_wcsc29_lib::position::*;
use kifuwarabe_wcsc29_lib::usi_conv::fen::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_player::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_position::*;
use kifuwarabe_wcsc29_lib::usi_conv::usi_record::*;
use std::env;

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

    // The application contains all immutable content.
    let app = Application::new();

    let path = args.path.unwrap();
    app.comm.println(&format!("args.path = '{}'.", path));

    let mut position = Position::default();

    let line = UsiRecord::read_first_line(&app.comm, &path);

    app.comm.println(&format!("Parse line: `{}`.", line));
    let mut urecord = UsiRecord::default();

    // Record.
    let mut recorder = RpmCassetteTapeRecorder::new_cassette_tape_recorder();

    let mut start = 0;
    if Fen::parse_initial_position(&app.comm, &line, &mut start, &mut recorder, &mut position) {
        app.comm.println("Position parsed.");

        if let Some(parsed_urecord) = UsiPosition::parse_usi_line_moves(
            &app.comm,
            &line,
            &mut start,
            position.get_board_size(),
        ) {
            app.comm.println("Moves parsed.");
            urecord = parsed_urecord;
        };
    }
    app.comm.println("Created urecord.");

    // ポジションをもう１回初期局面に戻す。
    let mut start = 0;
    if Fen::parse_initial_position(&app.comm, &line, &mut start, &mut recorder, &mut position) {
        app.comm.println("Position parsed.");
    }

    recorder.clear_recorder();
    UsiPlayer::play_out_and_record(&mut position, &urecord, &mut recorder, &app.comm);
    HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
}
