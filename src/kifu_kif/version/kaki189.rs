use kifu_kif::kif_move::*;
use kifu_kif::kif_tape::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::*;

/* Example
# ----  柿木将棋 V1.89 棋譜ファイル  ----
# ファイル名：kifu\morita.kif
# 対  局  日：1990/12/02(日)
# 開始時刻  ：10:21:22
# コメント  ：
手合割：平手
先手：通信:morita
後手：コンピュータ L1
手数----指手---------消費時間--
   1 ２六歩(27)   ( 0:16/ 0:00:16)
   2 ８四歩(83)   ( 0:01/ 0:00:01)
   3 ２五歩(26)   ( 0:01/ 0:00:17)
   4 ８五歩(84)   ( 0:01/ 0:00:02)
中略
 154 ４四金打     ( 0:31/ 0:43:56)
 155 中断         ( 0:07/ 0:46:39)
 */
/// Kifファイルには色んなパターンがあるようだ。
/// 柿木将棋 V1.89 棋譜ファイル
pub struct Kaki189 {}
impl Kaki189 {
    pub fn load(file: &str) -> KifTape {
        let mut record = KifTape::new();

        for result in BufReader::new(File::open(file).unwrap()).lines() {
            let line = result.unwrap();

            // スペースを除く、先頭が数字で始まる行は　指し手。
            if 4 < line.len() {
                let mut first_ch = line.trim_start().to_string();
                first_ch = first_ch.chars().nth(0).unwrap().to_string();
                match first_ch.parse::<i8>() {
                    Ok(_x) => {
                        if let Some(kif_move) = KifMove::parse(&line) {
                            record.push(kif_move);
                        }
                    }
                    Err(_err) => {
                        // この行は無視。
                    }
                }
            }
        }

        // '同'を解決する。
        let mut pre_cell = None;
        for mov in &mut record.items {
            if mov.is_same {
                mov.destination = pre_cell;
            }

            pre_cell = mov.destination;
        }

        // これでレコードはできあがり。
        record
    }
}
