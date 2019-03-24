use std::*;
use piece_etc::*;

#[derive(Clone, Copy, PartialEq)]
pub struct UsiMove {
    pub source_file:i8,
    pub source_rank:i8,
    pub destination_file:i8,
    pub destination_rank:i8,
    pub promotion:bool,
    pub drop:Option<PieceType>,
}
impl UsiMove {
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
