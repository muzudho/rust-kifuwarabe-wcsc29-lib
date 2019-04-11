use std::*;
use piece_etc::*;
use position::*;

#[derive(Clone, Copy, PartialEq)]
pub struct UsiMove {
    pub source_file:i8,
    pub source_rank:i8,
    pub destination_file:i8,
    pub destination_rank:i8,
    pub promotion:bool,
    drop:Option<PieceType>,
    resign: bool,
}
impl UsiMove {
    /// 盤上の駒を動かすとき。
    pub fn create_walk(
        src_file:i8,
        src_rank:i8,
        dst_file:i8,
        dst_rank:i8,
        pro:bool,
        board_size:BoardSize) -> UsiMove {

        debug_assert!(0<src_file && src_file <=board_size.get_file_len(), "Src file: {}.", src_file);
        debug_assert!(0<src_rank && src_rank <=board_size.get_rank_len(), "Src rank: {}.", src_rank);
        debug_assert!(0<dst_file && dst_file <=board_size.get_file_len(), "Dst file: {}.", dst_file);
        debug_assert!(0<dst_rank && dst_rank <=board_size.get_rank_len(), "Dst rank: {}.", dst_rank);

        UsiMove {
            source_file: src_file,
            source_rank: src_rank,
            destination_file: dst_file,
            destination_rank: dst_rank,
            promotion: pro,
            drop: None,
            resign: false,
        }
    }

    /// 打つとき。
    pub fn create_drop(
        dst_file:i8,
        dst_rank:i8,
        dro:PieceType,
        board_size:BoardSize) -> UsiMove {

        debug_assert!(0<dst_file && dst_file <=board_size.get_file_len(), "Dst file: {}.", dst_file);
        debug_assert!(0<dst_rank && dst_rank <=board_size.get_rank_len(), "Dst rank: {}.", dst_rank);

        UsiMove {
            source_file: -1,
            source_rank: -1,
            destination_file: dst_file,
            destination_rank: dst_rank,
            promotion: false,
            drop: Some(dro),
            resign: false,
        }
    }

    pub fn create_resign() -> UsiMove {
        UsiMove {
            source_file: -1,
            source_rank: -1,
            destination_file: -1,
            destination_rank: -1,
            promotion: false,
            drop: None,
            resign: true,
        }
    }

    pub fn is_drop(self) -> bool {
        self.drop != None
    }

    pub fn get_drop(self) -> Option<PieceType> {
        self.drop
    }

    pub fn set_resign(&mut self, yes:bool) {
        self.resign = yes
    }

    pub fn is_resign(self) -> bool {
        self.resign
    }

    /// USI符号。
    pub fn to_sign(self) -> String {
        if self.resign {
            return "resign".to_string();
        }

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
