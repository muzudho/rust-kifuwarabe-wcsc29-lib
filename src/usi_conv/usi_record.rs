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
        if mov.drop != None {
            // TODO drop

        } else {
            let mut source_piece = position.remove_piece(mov.source_file, mov.source_rank);
            if mov.promotion {
                source_piece = promotion_piece(source_piece);
            }
            position.set_piece(mov.destination_file, mov.destination_rank, source_piece);
            self.push(mov);
        }
    }
}
