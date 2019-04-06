use kif_conv::kif_move::*;
use piece_etc::*;
use position::*;
use std::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

 #[derive(Default)]
pub struct KifRecord {
    pub items : Vec<KifMove>,
}
impl KifRecord {
    pub fn new() -> KifRecord {
        KifRecord {
            items: Vec::new(),
        }
    }

    pub fn load(file:&str) -> KifRecord {
        let mut record = KifRecord::new();

        for result in BufReader::new(File::open(file).unwrap()).lines() {
            let line = result.unwrap();

            // スペースを除く、先頭が数字で始まる行は　指し手。
            if 4 < line.len() {
                let mut first_ch = line.trim_start().to_string();
                first_ch = first_ch.chars().nth(0).unwrap().to_string();
                match first_ch.parse::<i8>() {
                    Ok(x) => {
                        if let Some(kif_move) = KifMove::parse(&line) {
                            record.push(kif_move);
                        }
                    },
                    Err(err) => {
                        // この行は無視。
                    },
                }
            }
        }

        // '同'を解決する。
        let mut pre_file = 0;
        let mut pre_rank = 0;
        for mov in &mut record.items {
            if mov.is_same {
                mov.destination_file = pre_file;
                mov.destination_rank = pre_rank;
            }

            pre_file = mov.destination_file;
            pre_rank = mov.destination_rank;
        }

        // これでレコードはできあがり。
        record
    }

    pub fn push(&mut self, mov:KifMove) {
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
