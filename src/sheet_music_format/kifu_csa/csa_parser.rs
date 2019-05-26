use regex::Regex;
use sheet_music_format::kifu_csa::csa_move::*;
use sheet_music_format::kifu_csa::csa_tape::*;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::*;
use studio::application::Application;

/* Example
V2.2
N+Apery
N-名人コブラ
$START_TIME:2018/05/05 09:44:47
PI
+
+2726FU
T8
-8384FU
T0
     */
pub struct CsaParser {}
impl CsaParser {
    pub fn from_file(file: &str, app: &Application) -> CsaTape {
        let mut tape = CsaTape::new();

        let file_stem = Path::new(&file)
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_file_stem_from_file_path.")));
        tape.get_mut_tape_label().set_name(file_stem);

        let mut num = 0;
        for line_result in
            BufReader::new(File::open(file).unwrap_or_else(|err| panic!(app.comm.panic_io(&err))))
                .lines()
        {
            let line = line_result.unwrap_or_else(|err| panic!(app.comm.panic_io(&err)));

            if num == 0 && line.starts_with("V") {
                // 最初の行で V で始まれば バージョン番号と予想。
                tape.get_mut_tape_label().set_format(&line);
            } else if (line.starts_with('+') | line.starts_with('-') | line.starts_with('%'))
                && line.len() == 7
            {
                // 7文字以上で、先頭が +, -, % で始まれば　指し手。
                // print!("{}  ", line);
                if let Some(csa_move) = CsaMove::parse(&line, &app) {
                    tape.push_move(csa_move);
                }
            } else if line.starts_with("$START_TIME:") {
                // https://www.debuggex.com/
                // ```
                // $START_TIME:2018/05/05 09:44:47
                // ```
                // $で始まれば情報の行。
                // 正規表現の $ は行末なので、エスケープします。
                let re = Regex::new(r"\$START_TIME:(.*)")
                    .unwrap_or_else(|f| panic!(app.comm.panic(&f.to_string())));
                let matched = re
                    .captures(&line)
                    .unwrap_or_else(|| panic!(app.comm.panic("Fail. regex parse.")));
                let matched_text = matched.get(1).map_or("", |m| m.as_str());
                tape.get_mut_tape_label().set_game_date(&matched_text);
            }

            num += 1;
        }

        tape
    }
}
