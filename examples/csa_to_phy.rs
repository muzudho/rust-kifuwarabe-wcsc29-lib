/// ```Shell
/// ### Example.
/// cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
/// cls
/// 
/// ### Compile.
/// cargo clippy --example csa_to_phy
/// 
/// ### Run.
/// cargo run --example csa_to_phy
/// ```
/// 
/// Execution file.
/// C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/rust-kifuwarabe-wcsc29.exe

extern crate kifuwarabe_wcsc29_lib;

use kifuwarabe_wcsc29_lib::common_operation::*;
use kifuwarabe_wcsc29_lib::communication::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_converter::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_move::*;
use kifuwarabe_wcsc29_lib::csa_conv::csa_record::*;
use kifuwarabe_wcsc29_lib::physical_record::*;
use kifuwarabe_wcsc29_lib::position::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let comm = Communication::new();
    let mut physical_record = PhysicalRecord::new();
    let mut position = Position::default();

    let c_record = CsaRecord::load("download-kifu/WCSC28_F6_PAL_HFW.csa");
    CsaConverter::convert_record(&comm, &mut position, &c_record, &mut physical_record);
    CommonOperation::bo(&comm, &physical_record, &position);
}

/// CSA形式の棋譜を、フィジカル レコードに変換。
pub struct CsaToPhy {
}
impl CsaToPhy {
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
