use address::*;
use std::*;
use piece_etc::*;
use position::*;

#[derive(Clone, Copy, PartialEq)]
pub struct UsiMove {
    pub source:Option<Cell>,
    pub destination:Option<Cell>,
    pub promotion:bool,
    drop:Option<PieceType>,
    resign: bool,
}
impl UsiMove {
    /// 盤上の駒を動かすとき。
    pub fn create_walk(
        src:Cell,
        dst:Cell,
        pro:bool,
        board_size:BoardSize) -> UsiMove {

        debug_assert!(0<src.get_file() && src.get_file() <=board_size.get_file_len(), "Src file: {}.", src.to_scalar().to_string());
        debug_assert!(0<src.get_rank() && src.get_rank() <=board_size.get_rank_len(), "Src rank: {}.", src.to_scalar().to_string());
        debug_assert!(0<dst.get_file() && dst.get_file() <=board_size.get_file_len(), "Dst file: {}.", dst.to_scalar().to_string());
        debug_assert!(0<dst.get_rank() && dst.get_rank() <=board_size.get_rank_len(), "Dst rank: {}.", dst.to_scalar().to_string());

        UsiMove {
            source: Some(src),
            destination: Some(dst),
            promotion: pro,
            drop: None,
            resign: false,
        }
    }

    /// 打つとき。
    pub fn create_drop(
        dst:Cell,
        dro:PieceType,
        board_size:BoardSize) -> UsiMove {

        debug_assert!(0<dst.get_file() && dst.get_file() <=board_size.get_file_len(), "Dst file: {}.", dst.to_scalar().to_string());
        debug_assert!(0<dst.get_rank() && dst.get_rank() <=board_size.get_rank_len(), "Dst rank: {}.", dst.to_scalar().to_string());

        UsiMove {
            source: None,
            destination: Some(dst),
            promotion: false,
            drop: Some(dro),
            resign: false,
        }
    }

    pub fn create_resign() -> UsiMove {
        UsiMove {
            source: None,
            destination: None,
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
            sign.push_str(&format!("{}{}", self.source.unwrap().get_file(), rank_to_sign(self.source.unwrap().get_rank())));
        }

        sign.push_str(&format!("{}{}", self.destination.unwrap().get_file(), rank_to_sign(self.destination.unwrap().get_rank())));

        if self.promotion {
            sign.push_str("+");
        }

        sign
    }
}
