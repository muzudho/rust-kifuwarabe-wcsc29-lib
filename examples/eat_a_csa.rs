extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
extern crate rand;
use getopts::Options;
use std::env;

use kifuwarabe_wcsc29_lib::application::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_player::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_record::*;
use kifuwarabe_wcsc29_lib::kifu_rpm::object::rpm_cassette_tape_box_conveyor::*;
use kifuwarabe_wcsc29_lib::kifu_rpm::recorder::rpm_cassette_tape_recorder::*;
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

    // The application contains all immutable content.
    let app = Application::new();

    let path = args.path.unwrap();
    app.comm.println(&format!("args.path = '{}'.", path));

    // Record.
    let mut tape_box_conveyor = RpmCassetteTapeBoxConveyor::new_empty();
    let mut recorder = RpmCassetteTapeRecorder::new_cassette_tape_recorder();

    // Model.
    let mut position = Position::default();
    let crecord = CsaRecord::load(&path);

    // Play out.
    CsaPlayer::play_out_and_record(&mut position, &crecord, &mut recorder, &app.comm);
    HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);

    // Save.
    tape_box_conveyor.write_cassette_tape_box(
        &app.kw29_conf,
        position.get_board_size(),
        &recorder.cassette_tape,
        &app.comm,
    );

    app.comm.println("Finished.");
}
