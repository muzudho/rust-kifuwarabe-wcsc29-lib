extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;

use getopts::Options;
use kifuwarabe_wcsc29_lib::application::*;
use kifuwarabe_wcsc29_lib::kifu_kif::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::object_rpm::cassette_deck::*;
use kifuwarabe_wcsc29_lib::object_rpm::cassette_tape_box::*;
use kifuwarabe_wcsc29_lib::shogi_ban::position::*;
use kifuwarabe_wcsc29_lib::*;
use std::env;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug)]
pub struct Arguments {
    pub input_file: Option<String>,
    pub output_file: Option<String>,
}
impl Arguments {
    pub fn parse() -> Arguments {
        let args: Vec<String> = env::args().collect();

        let mut opts = Options::new();
        opts.optopt("i", "input", "set input record file name.", "NAME");
        opts.optopt("o", "output", "set output record file name.", "NAME");

        let matches = opts
            .parse(&args[1..])
            .unwrap_or_else(|f| panic!(f.to_string()));

        Arguments {
            input_file: matches.opt_str("input"),
            output_file: matches.opt_str("output"),
        }
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn main() {
    // Command line arguments.
    let args = Arguments::parse();
    let in_file = args.input_file.unwrap();
    let tape_box_file_for_write = args.output_file.unwrap();

    // The application contains all immutable content.
    let app = Application::new();

    // Position.
    let position = Position::new_honshogi_origin();

    // Deck.
    let mut deck = CassetteDeck::new_change(
        Some(CassetteTapeBox::from_file(
            tape_box_file_for_write.to_string(),
            position.get_board_size(),
            &app,
        )),
        position.get_board_size(),
        &app,
    );

    if !in_file.is_empty() {
        // 棋譜解析。
        let ext = get_extension_from_filename(&in_file)
            .unwrap()
            .to_uppercase();

        match ext.as_str() {
            "KIF" => {
                KifConverter::convert_kif_tape_fragment(&in_file, &mut deck, &app);
            }
            "CSA" => {}
            _ => print!("Pass extension: {}", ext),
        }
    } else {
        main_loop();
    }
}
