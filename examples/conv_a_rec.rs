extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;

use kifuwarabe_wcsc29_lib::application::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_converter::CsaConverter;
use kifuwarabe_wcsc29_lib::kifu_kif::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::object_rpm::cassette_deck::cassette_tape_editor::*;
use kifuwarabe_wcsc29_lib::object_rpm::cassette_tape_box_conveyor::*;
use kifuwarabe_wcsc29_lib::*;
use std::ffi::OsStr;
use std::path::Path;

use getopts::Options;
use std::env;

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

    // Record.
    let mut tape_box_conveyer = CassetteTapeBoxConveyor::new_empty();
    tape_box_conveyer.choice_box_manually(&tape_box_file_for_write);
    let mut recorder = CassetteTapeEditor::new_cassette_tape_editor();

    if !in_file.is_empty() {
        // 棋譜解析。
        let ext = get_extension_from_filename(&in_file)
            .unwrap()
            .to_uppercase();

        match ext.as_str() {
            "KIF" => {
                KifConverter::convert_kif(&in_file, &mut tape_box_conveyer, &mut recorder, &app);
            }
            "CSA" => {
                CsaConverter::convert_csa(&in_file, &mut tape_box_conveyer, &mut recorder, &app);
            }
            _ => print!("Pass extension: {}", ext),
        }
    } else {
        main_loop();
    }
}
