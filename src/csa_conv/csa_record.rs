use csa_conv::csa_move::*;
use piece_etc::*;
use position::*;
use std::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

 #[derive(Default)]
pub struct CsaRecord {
    pub items : Vec<CsaMove>,
}
impl CsaRecord {
    pub fn new() -> CsaRecord {
        CsaRecord {
            items: Vec::new(),
        }
    }

    pub fn load(file:&str) -> CsaRecord {
        let mut record = CsaRecord::new();

        for result in BufReader::new(File::open(file).unwrap()).lines() {
            let line = result.unwrap();

            if (line.starts_with('+') | line.starts_with('-') | line.starts_with('%')) && line.len()==7 {
                print!("{}  ", line);
                if let Some(csa_move) = CsaMove::parse(&line) {
                    record.push(csa_move);
                }
            }
        }

        record
    }

    pub fn push(&mut self, mov:CsaMove) {
        self.items.push(mov);
    }

    pub fn get_current_phase(&self) -> Phase {
        match self.items.len() % 2 {
            0 => Phase::First,
            _ => Phase::Second,
        }
    }

    pub fn make_move(&mut self, mov:CsaMove, position:&mut Position){
        if mov.is_drop() {
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
