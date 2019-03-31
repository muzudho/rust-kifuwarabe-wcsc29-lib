use common_operation::*;
use communication::*;
use piece_etc::*;
use position::*;
use std::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use usi_conv::fen::*;
use usi_conv::usi_move::*;

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

    pub fn load(comm:&Communication, file:&str) -> UsiRecord {
        let mut position = Position::default();
        let mut urecord = UsiRecord::new();

        for result in BufReader::new(File::open(file).unwrap()).lines() {
            let line = result.unwrap();

            comm.println(&format!("{}  ", line));
            let mut start = 0;
            if Fen::parse_position(&comm, &line, &mut start, &mut position) {
                comm.println("Position parsed.");

                if let Some(parsed_urecord) = CommonOperation::read_usi_moves(&comm, &line, &mut start, &mut position) {
                    comm.println("Moves parsed.");
                    urecord = parsed_urecord;
                };
            }
            
            break;
        }

        urecord
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

    /// ex.) Parses 7g7f 3c3d.
    pub fn parse_usi_some_moves(&mut self, comm:&Communication, line:&str, start:&mut usize) {
        self.items.clear();

        loop {
            let lmove = Fen::parse_usi_1move(&comm, &line, start);
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
            let source_id_piece_opt = position.remove_id_piece(mov.source_file, mov.source_rank);
            if mov.promotion {
                if let Some(mut source_id_piece) = source_id_piece_opt {
                    source_id_piece.turn_over();
                }
            }
            position.set_id_piece(mov.destination_file, mov.destination_rank, source_id_piece_opt);
            self.push(mov);
        }
    }
}
