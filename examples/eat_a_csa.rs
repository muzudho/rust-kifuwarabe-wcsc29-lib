extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
extern crate rand;
use getopts::Options;
use std::env;

use kifuwarabe_wcsc29_lib::application::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_converter::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_tape::*;
use kifuwarabe_wcsc29_lib::kifu_rpm::cassette_deck::rpm_cassette_tape_editor::*;
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

    // The application contains all immutable content.
    let app = Application::new();

    let path = args.path.unwrap();
    app.comm.println(&format!("args.path = '{}'.", path));

    // Record.
    let mut tape_box_conveyor = RpmCassetteTapeBoxConveyor::new_empty();
    let mut recorder = RpmCassetteTapeEditor::new_cassette_tape_editor();

    // Model.
    let mut position = Position::default();
    let crecord = CsaTape::load(&path);

    // Play out.
    CsaConverter::play_out_usi_tape(
        &mut position,
        &crecord,
        &mut tape_box_conveyor,
        &mut recorder,
        &app.comm,
    );
    HumanInterface::bo(
        &app.comm,
        &tape_box_conveyor.recording_cassette_tape,
        recorder.ply,
        &position,
    );

    // Save.
    tape_box_conveyor.write_cassette_tape_box(position.get_board_size(), &app);

    app.comm.println("Finished.");
}
