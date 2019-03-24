/// ```Shell
/// ### Example.
/// cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29
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

extern crate kifuwarabe_wcsc29;

use kifuwarabe_wcsc29::csa_conv::csa_move::CsaMove;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    CsaToPhy::load("download-kifu/WCSC28_F6_PAL_HFW.csa");
}

/// CSA形式の棋譜を、フィジカル レコードに変換。
pub struct CsaToPhy {
}
impl CsaToPhy {
    pub fn load(file:&str) {
        for result in BufReader::new(File::open(file).unwrap()).lines() {
            let line = result.unwrap();
            if (line.starts_with("+") | line.starts_with("-") | line.starts_with("%")) && line.len()==7 {
                let csa_move = CsaMove.parse(line);
                println!("{}  {}", line, csa_move.to_text());
            // } else {
            //    println!("x {}", line);
            }
        }
    }
}
