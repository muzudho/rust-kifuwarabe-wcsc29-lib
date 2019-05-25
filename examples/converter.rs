extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;

use getopts::Options;
use kifuwarabe_wcsc29_lib::audio_compo::audio_rack::*;
use kifuwarabe_wcsc29_lib::audio_compo::cassette_deck::*;
use kifuwarabe_wcsc29_lib::instrument::position::*;
use kifuwarabe_wcsc29_lib::media::cassette_tape::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_converter::CsaConverter;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_tape::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_kif::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_kif::kif_tape::*;
use kifuwarabe_wcsc29_lib::studio::application::*;
use kifuwarabe_wcsc29_lib::*;
use std::env;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug)]
pub struct Arguments {
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub debug: bool,
}
impl Arguments {
    pub fn parse(app: &Application) -> Arguments {
        let args: Vec<String> = env::args().collect();

        let mut opts = Options::new();
        opts.optopt("i", "input", "set input record file name.", "NAME");
        opts.optopt("o", "output", "set output record file name.", "NAME");
        opts.optflag("d", "debug", "Debug.");

        let matches = opts
            .parse(&args[1..])
            .unwrap_or_else(|f| panic!(app.comm.panic(&f.to_string())));

        Arguments {
            input_file: matches.opt_str("input"),
            output_file: matches.opt_str("output"),
            debug: matches.opt_present("debug"),
        }
    }
}

fn get_extension_from_file_path(file_path: &str) -> Option<&str> {
    Path::new(file_path).extension().and_then(OsStr::to_str)
}

fn get_file_stem_from_file_path(file_path: &str) -> Option<&str> {
    Path::new(file_path).file_stem().and_then(OsStr::to_str)
}

fn main() {
    // The application contains all immutable content.
    let mut app = Application::new();

    // Command line arguments.
    let args = Arguments::parse(&app);

    let in_file = args
        .input_file
        .unwrap_or_else(|| panic!(app.comm.panic("Fail. args.input_file.")));

    // 保存先のファイル名を作るのに使う☆（＾～＾）
    let tape_file_name_without_extension = args
        .output_file
        .unwrap_or_else(|| panic!(app.comm.panic("Fail. args.output_file.")));

    if args.debug {
        app.kifuwarabe_flag = true;
        app.comm.println("Debug on!");
    }

    // Position.
    let mut position = Position::new_honshogi_origin(&app);

    // Deck.
    let mut rack = AudioRack::new(&app);
    let mut tape = CassetteTape::new_facing_right(&app);
    tape.set_file_full_name_without_extension(&tape_file_name_without_extension);
    rack.add_tape_to_tape_box(Slot::Learning, tape, &app);

    if !in_file.is_empty() {
        // 棋譜解析。
        let file_stem = get_file_stem_from_file_path(&in_file)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_file_stem_from_file_path.")));

        let extension = get_extension_from_file_path(&in_file)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_extension_from_file_path.")))
            .to_uppercase();

        match extension.as_str() {
            "KIF" => {
                // Training data.
                let mut ktape = KifTape::from_file(&in_file, &app);

                // Play out.
                KifConverter::play_out_kifu_tape(&ktape, &mut rack, &mut position, &app);

                // Tape label
                rack.set_source_file_of_tape_label(Slot::Learning, file_stem.to_string());

                // Write.
                rack.write_leaning_tapes_fragment(position.get_board_size(), &app);
            }
            "CSA" => {
                // Training data.
                let mut ctape = CsaTape::from_file(&in_file, &app);

                if app.is_debug() {
                    app.comm
                        .println(&format!("Ctape: '{}'", ctape.to_human_presentable()));
                }

                // Play out.
                CsaConverter::play_out_csa_tape(&ctape, &mut rack, &mut position, &app);

                // Tape label
                rack.set_source_file_of_tape_label(Slot::Learning, file_stem.to_string());

                // Write.
                rack.write_leaning_tapes_fragment(position.get_board_size(), &app);
            }
            _ => print!("Pass extension: {}", extension),
        }
    } else {
        main_loop();
    }
}
