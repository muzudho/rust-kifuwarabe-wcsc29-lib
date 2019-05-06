extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;
use getopts::Options;
use kifuwarabe_wcsc29_lib::audio_compo::cassette_deck::*;
use kifuwarabe_wcsc29_lib::human::human_interface::*;
use kifuwarabe_wcsc29_lib::instrument::position::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_usi::fen::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_usi::usi_converter::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_usi::usi_position::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_usi::usi_tape::*;
use kifuwarabe_wcsc29_lib::studio::application::*;
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

    let path = args
        .path
        .unwrap_or_else(|| panic!(app.comm.panic("Fail. Arg path.")));
    app.comm.println(&format!("args.path = '{}'.", path));

    // Position.
    let mut position = Position::new_honshogi_origin();

    let line = UsiTape::read_first_line(&path, &app);

    app.comm.println(&format!("Parse line: `{}`.", line));
    let mut utape = UsiTape::default();

    // Deck.
    let mut deck = CassetteDeck::new_empty(&app);

    let mut start = 0;
    if Fen::parse_initial_position(&line, &mut start, &mut position, &mut deck, &app) {
        app.comm.println("Position parsed.");

        if let Some(parsed_utape) =
            UsiPosition::parse_usi_line_moves(&line, &mut start, position.get_board_size(), &app)
        {
            app.comm.println("Moves parsed.");
            utape = parsed_utape;
        };
    }
    app.comm.println("Created utape.");

    // ポジションをもう１回初期局面に戻す。
    let mut start = 0;
    if Fen::parse_initial_position(&line, &mut start, &mut position, &mut deck, &app) {
        app.comm.println("Position parsed.");
    }

    deck.change_training_tape(None, position.get_board_size(), &app);
    UsiConverter::play_out_usi_tape(&mut position, &utape, &mut deck, &app);
    HumanInterface::bo(&mut deck, &position, &app);
}
