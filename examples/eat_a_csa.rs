extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
extern crate rand;
use getopts::Options;
use kifuwarabe_wcsc29_lib::audio_compo::cassette_deck::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::instrument::position::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_converter::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_tape::*;
use kifuwarabe_wcsc29_lib::studio::application::*;
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

    let path = args
        .path
        .unwrap_or_else(|| panic!(app.comm.panic("Fail. Arg path.")));
    app.comm.println(&format!("args.path = '{}'.", path));

    // Position.
    let mut position = Position::new_honshogi_origin(&app);

    // Deck.
    let mut deck = CassetteDeck::new_empty(&app);

    let crecord = CsaTape::from_file(&path, &app);

    // Play out.
    CsaConverter::play_out_csa_tape(&crecord, &mut position, &mut deck, &app);
    HumanInterface::bo(&mut deck, &position, &app);

    // Save.
    deck.write_tape_box(position.get_board_size(), &app);

    app.comm.println("Finished.");
}
