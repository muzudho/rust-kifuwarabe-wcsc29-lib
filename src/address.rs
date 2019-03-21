use board::*;

/// Vector に入れるときコピーする。
#[derive(Clone, Copy, PartialEq)]
pub struct Address {
    pub index: usize,
    pub hand: Option<Phase>,
}
impl Address {
    pub fn create_by_index(index_num:usize) -> Address {
        Address {
            index: index_num,
            hand: None,
        }
    }

    pub fn create_by_cell(file_num:i8, rank_num:i8, board_size:&BoardSize) -> Address {
        Address {
            index: board_size.file_rank_to_cell(file_num, rank_num),
            hand: None,
        }
    }

    pub fn create_hand(hand_phase:&Phase) -> Address {
        Address {
            index: 0 as usize,
            hand: Some(*hand_phase),
        }
    }

    pub fn to_physical_sign(&self, board_size:&BoardSize) -> String {
        if 0 <= self.index && self.index <= 80 {
            // 盤上
            let (file, rank) = board_size.cell_to_file_rank(self.index);
            format!("{}{}", file, i8_to_rank_char(rank))
        } else if 81 <= self.index && self.index <= 104 {
            // 持ち駒
            use board::Piece::*;
            match self.index {
                81 => { format!("{}*", piece_to_sign(Some(K1)))},
                82 => { format!("{}*", piece_to_sign(Some(R1)))},
                83 => { format!("{}*", piece_to_sign(Some(B1)))},
                84 => { format!("{}*", piece_to_sign(Some(G1)))},
                85 => { format!("{}*", piece_to_sign(Some(S1)))},
                86 => { format!("{}*", piece_to_sign(Some(N1)))},
                87 => { format!("{}*", piece_to_sign(Some(L1)))},
                88 => { format!("{}*", piece_to_sign(Some(P1)))},
                89 => { format!("{}*", piece_to_sign(Some(K2)))},
                90 => { format!("{}*", piece_to_sign(Some(R2)))},
                91 => { format!("{}*", piece_to_sign(Some(B2)))},
                92 => { format!("{}*", piece_to_sign(Some(G2)))},
                93 => { format!("{}*", piece_to_sign(Some(S2)))},
                94 => { format!("{}*", piece_to_sign(Some(N2)))},
                95 => { format!("{}*", piece_to_sign(Some(L2)))},
                96 => { format!("{}*", piece_to_sign(Some(P2)))},
                97 => { format!("{}*", piece_to_sign(Some(K3)))},
                98 => { format!("{}*", piece_to_sign(Some(R3)))},
                99 => { format!("{}*", piece_to_sign(Some(B3)))},
                100 => { format!("{}*", piece_to_sign(Some(G3)))},
                101 => { format!("{}*", piece_to_sign(Some(S3)))},
                102 => { format!("{}*", piece_to_sign(Some(N3)))},
                103 => { format!("{}*", piece_to_sign(Some(L3)))},
                104 => { format!("{}*", piece_to_sign(Some(P3)))},
                _ => {panic!("Unexpected index print: {0}.", self.index);}
            }
        } else if self.index == 105 {
            "Sky".to_string()
        } else {
            panic!("Unexpected index print: {0}.", self.index);
        }
    }
}