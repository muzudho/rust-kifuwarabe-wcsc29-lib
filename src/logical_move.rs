use std::*;
use physical_record::*;

#[derive(Clone, Copy, PartialEq)]
pub struct LogicalMove {
    pub source_file:i8,
    pub source_rank:i8,
    pub destination_file:i8,
    pub destination_rank:i8,
    pub promotion:bool,
    pub drop:Option<PieceType>,
}
impl LogicalMove {
    pub fn new() -> LogicalMove {
        LogicalMove {
            source_file:0,
            source_rank:0,
            destination_file:0,
            destination_rank:0,
            promotion:false,
            drop:None,
        }
    }

    pub fn to_sign(self) -> String {
        let mut sign = String::new();

        if self.drop != None {
            sign.push_str(&format!("{}*", piece_type_to_sign(self.drop)));
        } else {
            sign.push_str(&format!("{}{}", self.source_file, rank_to_sign(self.source_rank)));
        }

        sign.push_str(&format!("{}{}", self.destination_file, rank_to_sign(self.destination_rank)));

        if self.promotion {
            sign.push_str("+");
        }

        sign
    }
}
