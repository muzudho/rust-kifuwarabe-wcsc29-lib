extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
extern crate rand;
use getopts::Options;
use kifuwarabe_wcsc29_lib::application::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_converter::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_tape::*;
use kifuwarabe_wcsc29_lib::object_rpm::cassette_deck::*;
use kifuwarabe_wcsc29_lib::shogi_ban::position::*;
use std::env;

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
    let mut deck = CassetteDeck::new_empty(&app);

    // Model.
    let mut position = Position::default();
    let crecord = CsaTape::load(&path, &app.comm);

    // Play out.
    CsaConverter::play_out_csa_tape(&crecord, &mut position, &mut deck, &app);
    HumanInterface::bo(&mut deck, Slot::Learning, &position, &app);

    // Save.
    deck.write_tape_box(position.get_board_size(), &app);

    app.comm.println("Finished.");
}
