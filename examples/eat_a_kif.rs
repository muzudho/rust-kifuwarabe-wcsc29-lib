extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;
extern crate regex;

use std::env;
use getopts::Options;

use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::kif_conv::kif_converter::*;


#[derive(Debug)]
struct Args {
  path: Option<String>,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "path", "set input csa file name.", "NAME");

    let matches = opts.parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    Args {
        path: matches.opt_str("path"),
    }
}

pub fn main()
{
    // Command line arguments.
    let args = parse_args();

    // Logging.
    let comm = Communication::new();
    let path = args.path.unwrap();
    comm.println(&format!("args.path = '{}'.", path));

    KifConverter::convert_kif(path);
}