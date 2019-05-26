use regex::Regex;
use sheet_music_format::kifu_kif::kif_move::*;
use sheet_music_format::kifu_kif::kif_tape::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::*;
use studio::application::Application;

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
    pub fn from_file(file: &str, app: &Application) -> KifTape {
        let mut tape = KifTape::new();

        for result in
            BufReader::new(File::open(file).unwrap_or_else(|err| panic!(app.comm.panic_io(&err))))
                .lines()
        {
            let line = result.unwrap_or_else(|err| panic!(app.comm.panic_io(&err)));

            // 4文字以上で。
            if line.starts_with("# 対  局  日：") {
                let re = Regex::new(r"# 対  局  日：(.*)")
                    .unwrap_or_else(|f| panic!(app.comm.panic(&f.to_string())));
                let matched = re
                    .captures(&line)
                    .unwrap_or_else(|| panic!(app.comm.panic("Fail. regex parse.")));
                let date_text = matched.get(1).map_or("", |m| m.as_str());
                tape.set_game_date(&date_text);
            } else if 4 < line.len() {
                // 先頭の空白を省き。
                let mut first_ch = line.trim_start().to_string();
                first_ch = first_ch
                    .chars()
                    .nth(0)
                    .unwrap_or_else(|| panic!(app.comm.panic("Fail. n0.")))
                    .to_string();
                match first_ch.parse::<i8>() {
                    Ok(_x) => {
                        // 数字で始まる行は　指し手。
                        if let Some(kif_move) = KifMove::parse(&line, &app) {
                            tape.push_move(kif_move);
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
        for mov in &mut tape.moves {
            if mov.is_same {
                mov.destination = pre_cell;
            }

            pre_cell = mov.destination;
        }

        // これでテープはできあがり。
        tape
    }
}
