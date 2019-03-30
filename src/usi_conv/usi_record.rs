use std::*;
use usi_conv::fen::*;
use usi_conv::usi_move::*;
use piece_etc::*;
use position::*;

 #[derive(Default)]
pub struct UsiRecord {
    pub items : Vec<UsiMove>,
}
impl UsiRecord {
    pub fn new() -> UsiRecord {
        UsiRecord {
            items: Vec::new(),
        }
    }

    /*
    pub fn get_items(self) -> Vec<UsiMove> {
        self.items
    }
     */

    pub fn push(&mut self, mov:UsiMove) {
        self.items.push(mov);
    }

    /*
    pub fn clear(&mut self) {
        self.items.clear();
    }
     */

    pub fn parse2(&mut self, line:&str, start:&mut usize) {
        self.items.clear();

        loop {
            let lmove = Fen::parse3(&line, start);
            self.items.push(lmove);

            if *start + 1 < line.len() {
                *start += 1;
            } else {
                break;
            }
        }
    }

    pub fn get_current_phase(&self) -> Phase {
        match self.items.len() % 2 {
            0 => Phase::First,
            _ => Phase::Second,
        }
    }

    pub fn make_move(&mut self, mov:UsiMove, position:&mut Position){
        if mov.is_drop() {
            // TODO drop

        } else {
            let mut source_id_piece_opt = position.remove_id_piece(mov.source_file, mov.source_rank);
            if mov.promotion {
                if let Some(source_id_piece) = source_id_piece_opt {
                    source_id_piece.turn_over();
                }
            }
            position.set_id_piece(mov.destination_file, mov.destination_rank, source_id_piece_opt);
            self.push(mov);
        }
    }
}
