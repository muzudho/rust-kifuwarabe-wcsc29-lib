/// ```Shell
/// ### Example.
/// cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
/// cls
/// 
/// ### Compile.
/// cargo clippy --example csa_to_rpm
/// 
/// ### Run.
/// cargo run --example csa_to_rpm
/// ### '--' is separator. You can pass arguments to exe.
/// cargo run --example csa_to_rpm -- --path download-kifu/WCSC28_F6_PAL_HFW.csa
/// ```
/// 
/// Execution file.
/// C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/rust-kifuwarabe-wcsc29.exe

extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;
use std::env;
use getopts::Options;

use kifuwarabe_wcsc29_lib::common_operation::*;
use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_converter::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_move::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_record::*;
use kifuwarabe_wcsc29_lib::physical_record::*;
use kifuwarabe_wcsc29_lib::position::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub fn main() {
    let args = parse_args();

    let comm = Communication::new();
    let csa_path = args.path.unwrap();
    comm.println(&format!("args.path = '{}'.", csa_path));

    let mut physical_record = PhysicalRecord::default();
    let mut position = Position::default();

    let c_record = CsaRecord::load(&csa_path); // ex.) "download-kifu/WCSC28_F6_PAL_HFW.csa"
    CsaConverter::convert_record(&comm, &mut position, &c_record, &mut physical_record);
    CommonOperation::bo(&comm, &physical_record, &position);
}

/// CSA形式の棋譜を、フィジカル レコードに変換。
pub struct CsaToRmp {
}
impl CsaToRmp {
    pub fn load(file:&str) {
        for result in BufReader::new(File::open(file).unwrap()).lines() {
            let line = result.unwrap();

            if (line.starts_with('+') | line.starts_with('-') | line.starts_with('%')) && line.len()==7 {
                print!("{}  ", line);
                if let Some(csa_move) = CsaMove::parse(&line) {
                    println!("{}", csa_move.to_text());
                }
            // } else {
            //    println!("x {}", line);
            }
        }
    }
}
