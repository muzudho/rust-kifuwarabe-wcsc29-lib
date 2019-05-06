use instrument::half_player_phase::*;
use instrument::position::*;
use sheet_music_format::kifu_csa::csa_move::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::*;
use studio::application::Application;

#[derive(Default)]
pub struct CsaTape {
    pub items: Vec<CsaMove>,
}
impl CsaTape {
    pub fn new() -> CsaTape {
        CsaTape { items: Vec::new() }
    }

    pub fn from_file(file: &str, app: &Application) -> CsaTape {
        let mut record = CsaTape::new();

        for result in
            BufReader::new(File::open(file).unwrap_or_else(|err| panic!(app.comm.panic_io(&err))))
                .lines()
        {
            let line = result.unwrap_or_else(|err| panic!(app.comm.panic_io(&err)));

            if (line.starts_with('+') | line.starts_with('-') | line.starts_with('%'))
                && line.len() == 7
            {
                print!("{}  ", line);
                if let Some(csa_move) = CsaMove::parse(&line, &app) {
                    record.push(csa_move);
                }
            }
        }

        record
    }

    pub fn push(&mut self, mov: CsaMove) {
        self.items.push(mov);
    }

    pub fn get_current_phase(&self) -> HalfPlayerPhaseValue {
        match self.items.len() % 2 {
            0 => HalfPlayerPhaseValue::First,
            _ => HalfPlayerPhaseValue::Second,
        }
    }

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
            self.push(cmove);
        }
    }

    pub fn to_human_presentable(&self) -> String {
        let mut text = "[CTape: ".to_string();

        for cmove in &self.items {
            text = format!("{} {}", text, cmove.to_human_presentable());
        }

        text = format!("{}]", text);

        text
    }
}
