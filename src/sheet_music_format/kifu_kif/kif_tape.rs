use sheet_music_format::kifu_kif::kif_move::*;
use sheet_music_format::kifu_kif::version::kaki189::*;
use std::*;
use studio::application::Application;

#[derive(Default)]
pub struct KifTape {
    pub items: Vec<KifMove>,
}
impl KifTape {
    pub fn new() -> KifTape {
        KifTape { items: Vec::new() }
    }

    pub fn from_file(file: &str, app: &Application) -> KifTape {
        // バージョンがいろいろあるようだ。
        Kaki189::from_file(file, &app)
    }

    pub fn push(&mut self, mov: KifMove) {
        self.items.push(mov);
    }

    /*
    pub fn get_current_phase(&self) -> Phase {
        match self.items.len() % 2 {
            0 => Phase::First,
            _ => Phase::Second,
        }
    }

    pub fn make_move(&mut self, cmove:KifMove, position:&mut Position){
        if cmove.is_drop() {
            // TODO drop

        } else {
            let source_id_piece_opt = position.remove_id_piece(cmove.source_file, cmove.source_rank);

            // CSAの棋譜では、成ったかどうかは分からない。
            /*
            if cmove.promotion {
                source_piece = promotion_piece(source_piece);
            }
            */

            position.set_id_piece(cmove.destination_file, cmove.destination_rank, source_id_piece_opt);
            self.push(cmove);
        }
    }
    */
}
