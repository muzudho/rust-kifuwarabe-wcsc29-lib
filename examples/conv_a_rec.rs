extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;

use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_config::*;
use kifuwarabe_wcsc29_lib::conf::kifuwarabe_wcsc29_lib_config::*;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_converter::CsaConverter;
use kifuwarabe_wcsc29_lib::kifu_kif::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::kifu_rpm::object::rpm_cassette_tape_box_conveyor::*;
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
    let tape_box_file = args.output_file.unwrap();

    // Logging.
    let comm = Communication::new();

    // Config.
    let my_conf = KifuwarabeWcsc29LibConfig::load();
    let kw29_conf = KifuwarabeWcsc29Config::load(&my_conf);

    // Record.
    let mut tape_box_conveyer = RpmCassetteTapeBoxConveyor::new_empty();
    tape_box_conveyer.choice_box_manually(&tape_box_file);

    if !in_file.is_empty() {
        // 棋譜解析。
        let ext = get_extension_from_filename(&in_file)
            .unwrap()
            .to_uppercase();

        match ext.as_str() {
            "KIF" => {
                KifConverter::convert_kif(&kw29_conf, &in_file, &mut tape_box_conveyer, &comm);
            }
            "CSA" => {
                CsaConverter::convert_csa(&kw29_conf, &in_file, &mut tape_box_conveyer, &comm);
            }
            _ => print!("Pass extension: {}", ext),
        }
    } else {
        main_loop();
    }
}
