use instrument::half_player_phase::*;
use instrument::position::*;
use regex::Regex;
use sheet_music_format::kifu_csa::csa_move::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::*;
use studio::application::Application;

#[derive(Default)]
pub struct CsaTape {
    pub moves: Vec<CsaMove>,
    game_date: String,
}
impl CsaTape {
    pub fn new() -> CsaTape {
        CsaTape {
            moves: Vec::new(),
            game_date: String::new(),
        }
    }

    // #####
    // # F #
    // #####

    pub fn from_file(file: &str, app: &Application) -> CsaTape {
        let mut tape = CsaTape::new();

        for line_result in
            BufReader::new(File::open(file).unwrap_or_else(|err| panic!(app.comm.panic_io(&err))))
                .lines()
        {
            let line = line_result.unwrap_or_else(|err| panic!(app.comm.panic_io(&err)));

            if (line.starts_with('+') | line.starts_with('-') | line.starts_with('%'))
                && line.len() == 7
            {
                // 7文字以上で、先頭が +, -, % で始まれば　指し手。
                print!("{}  ", line);
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
                let date_text = matched.get(1).map_or("", |m| m.as_str());
                tape.set_game_date(&date_text);
            }
        }

        tape
    }

    // #####
    // # G #
    // #####

    pub fn get_current_phase(&self) -> HalfPlayerPhaseValue {
        match self.moves.len() % 2 {
            0 => HalfPlayerPhaseValue::First,
            _ => HalfPlayerPhaseValue::Second,
        }
    }

    // 対局日。
    pub fn get_game_date(&self) -> String {
        self.game_date.to_string()
    }

    // #####
    // # M #
    // #####

    pub fn make_move(&mut self, cmove: CsaMove, position: &mut Position, app: &Application) {
        if cmove.is_drop() {
            // TODO drop

        } else {
            let source_id_piece_opt = position.remove_id_piece(
                cmove
                    .source
                    .unwrap_or_else(|| panic!(app.comm.panic("Fail. cmove.source."))),
            );

            // CSAの棋譜では、成ったかどうかは分からない。
            /*
            if cmove.promotion {
                source_piece = promotion_piece(source_piece);
            }
            */

            position.set_id_piece(cmove.destination, source_id_piece_opt);
            self.push_move(cmove);
        }
    }

    // #####
    // # P #
    // #####

    /// 指し手を追加。
    pub fn push_move(&mut self, mov: CsaMove) {
        self.moves.push(mov);
    }

    // #####
    // # S #
    // #####

    /// 対局日を記述。
    pub fn set_game_date(&mut self, date: &str) {
        self.game_date = date.to_string();
    }

    // #####
    // # T #
    // #####

    pub fn to_human_presentable(&self) -> String {
        let mut text = "[CTape: ".to_string();

        for cmove in &self.moves {
            text = format!("{} {}", text, cmove.to_human_presentable());
        }

        text = format!("{}]", text);

        text
    }
}
