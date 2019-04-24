use kifu_kif::version::kaki189::*;
use kifu_kif::kif_move::*;
use std::*;

#[derive(Default)]
pub struct KifRecord {
    pub items: Vec<KifMove>,
}
impl KifRecord {
    pub fn new() -> KifRecord {
        KifRecord { items: Vec::new() }
    }

    pub fn load(file: &str) -> KifRecord {
        // バージョンがいろいろあるようだ。
        Kaki189::load(file)
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
