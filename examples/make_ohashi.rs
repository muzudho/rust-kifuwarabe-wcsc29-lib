// 大橋流の棋譜を作る。
extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;

use getopts::Options;
use kifuwarabe_wcsc29_lib::audio_compo::cassette_deck::*;
use kifuwarabe_wcsc29_lib::instrument::position::*;
use kifuwarabe_wcsc29_lib::live::ohashi_performer::*;
use kifuwarabe_wcsc29_lib::media::cassette_tape::*;
use kifuwarabe_wcsc29_lib::studio::application::*;
use std::env;

#[derive(Debug)]
pub struct Arguments {
    pub output_file: Option<String>,
    pub debug: bool,
}
impl Arguments {
    pub fn parse(app: &Application) -> Arguments {
        let args: Vec<String> = env::args().collect();

        let mut opts = Options::new();
        opts.optopt("o", "output", "set output record file name.", "NAME");
        opts.optflag("d", "debug", "Debug.");

        let matches = opts
            .parse(&args[1..])
            .unwrap_or_else(|f| panic!(app.comm.panic(&f.to_string())));

        Arguments {
            output_file: matches.opt_str("output"),
            debug: matches.opt_present("debug"),
        }
    }
}

fn main() {
    // The application contains all immutable content.
    let mut app = Application::new();

    // Command line arguments.
    let args = Arguments::parse(&app);

    // 保存先のテープ・フラグメント名☆（＾～＾）　ラーニング・テープと想定☆（＾～＾）
    let tape_file_name_without_extension = args
        .output_file
        .unwrap_or_else(|| panic!(app.comm.panic("Fail. args.output_file.")));

    if args.debug {
        app.kifuwarabe_flag = true;
        app.comm.println("Debug on!");
    }

    // Deck.
    let mut deck = CassetteDeck::new_empty(&app);
    deck.set_file_name_without_extension_of_tape_box(
        Slot::Learning,
        &tape_file_name_without_extension,
    );
    let mut tape = CassetteTape::new_facing_right(&app);
    tape.set_file_full_name_without_extension(&tape_file_name_without_extension);
    deck.add_tape_to_tape_box(Slot::Learning, tape, &app);
    deck.seek_of_next_tape(Slot::Learning, &app);

    // Position.
    let mut position = Position::new_honshogi_origin(&app);

    // Play out.
    OhashiPerformer::improvise_ohashi_starting(&mut deck, &mut position, &app);

    // Write.
    deck.write_tape_box(position.get_board_size(), &app);
}
