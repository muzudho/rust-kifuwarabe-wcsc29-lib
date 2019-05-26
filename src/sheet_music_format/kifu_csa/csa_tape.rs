use instrument::half_player_phase::*;
use instrument::position::*;
use sheet_music_format::kifu_csa::csa_move::*;
use sheet_music_format::kifu_csa::csa_parser::*;
use sheet_music_format::tape_label::*;
use std::*;
use studio::application::Application;

#[derive(Default)]
pub struct CsaTape {
    // 指し手。
    pub moves: Vec<CsaMove>,

    // テープ・ラベル。
    tape_label: TapeLabel,
}
impl CsaTape {
    pub fn new() -> CsaTape {
        CsaTape {
            moves: Vec::new(),
            tape_label: TapeLabel::new(),
        }
    }

    // #####
    // # F #
    // #####

    pub fn from_file(file: &str, app: &Application) -> CsaTape {
        CsaParser::from_file(&file, &app)
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

    // テープ・ラベル。
    pub fn get_mut_tape_label(&mut self) -> &mut TapeLabel {
        &mut self.tape_label
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
