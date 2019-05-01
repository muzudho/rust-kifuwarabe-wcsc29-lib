extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;

use getopts::Options;
use kifuwarabe_wcsc29_lib::audio_compo::cassette_deck::*;
use kifuwarabe_wcsc29_lib::instrument::position::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_converter::CsaConverter;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_tape::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_kif::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_kif::kif_tape::*;
use kifuwarabe_wcsc29_lib::studio::application::*;
use kifuwarabe_wcsc29_lib::video_tape_model::cassette_tape_box::*;
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

    // トレーニング・テープと想定☆（＾～＾）
    let tape_box_file_for_write = args.output_file.unwrap();

    // The application contains all immutable content.
    let app = Application::new();

    // Position.
    let mut position = Position::new_honshogi_origin();

    // Deck.
    let mut deck = CassetteDeck::new_change(
        Some(CassetteTapeBox::from_training_file(
            &tape_box_file_for_write,
            position.get_board_size(),
            &app,
        )),
        position.get_board_size(),
        &app,
    );

    if !in_file.is_empty() {
        // 棋譜解析。
        let extension = get_extension_from_filename(&in_file)
            .unwrap()
            .to_uppercase();

        match extension.as_str() {
            "KIF" => {
                // Training data.
                let ktape = KifTape::from_file(&in_file);

                // Play out.
                KifConverter::play_out_kifu_tape(&ktape, &mut position, &mut deck, &app);

                // Write.
                deck.write_tape_fragment(position.get_board_size(), &app);
            }
            "CSA" => {
                // Training data.
                let ctape = CsaTape::from_file(&in_file, &app.comm);

                // Play out.
                CsaConverter::play_out_csa_tape(&ctape, &mut position, &mut deck, &app);

                // Write.
                deck.write_tape_fragment(position.get_board_size(), &app);
            }
            _ => print!("Pass extension: {}", extension),
        }
    } else {
        main_loop();
    }
}
