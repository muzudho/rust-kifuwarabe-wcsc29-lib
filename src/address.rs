use board::*;
use physical_record::*;

/// Vector に入れるときコピーする。
#[derive(Clone, Copy, PartialEq)]
pub struct Address {
    index: usize,
}
impl Address {
    pub fn create_by_index(index_num:usize) -> Address {
        Address {
            index: index_num,
        }
    }

    pub fn create_by_cell(file_num:i8, rank_num:i8, board_size:&BoardSize) -> Address {
        Address {
            index: board_size.file_rank_to_cell(file_num, rank_num),
        }
    }

    pub fn create_hand(phase_opt:Option<Phase>, piece_type:PieceType) -> Address {
        use board::Phase::*;
        use physical_record::PieceType::*;

        let index_num = match phase_opt {
            Some(phase) => {
                match phase {
                    First => {
                        match piece_type {
                            K => {82},
                            R => {83},
                            B => {84},
                            G => {85},
                            S => {86},
                            N => {87},
                            L => {88},
                            P => {89},
                            _ => panic!("Unexpected hand piece_type {}.", piece_type_to_sign(Some(piece_type)))
                        }
                    },
                    Second => {
                        match piece_type {
                            K => {90},
                            R => {91},
                            B => {92},
                            G => {93},
                            S => {94},
                            N => {95},
                            L => {96},
                            P => {97},
                            _ => panic!("Unexpected hand piece_type {}.", piece_type_to_sign(Some(piece_type)))
                        }
                    },
                }
            },
            None => {
                match piece_type {
                    K => {98},
                    R => {99},
                    B => {100},
                    G => {101},
                    S => {102},
                    N => {103},
                    L => {104},
                    P => {105},
                    _ => panic!("Unexpected hand piece_type {}.", piece_type_to_sign(Some(piece_type)))
                }
            },
        };

        Address {
            index: index_num,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    /// 盤上。
    pub fn is_on_board(&self) -> bool {
        self.index <= 80
    }

    /// 駒台
    pub fn is_hand(&self) -> bool {
        81 <= self.index && self.index <= 104
    }

    pub fn to_physical_sign(&self, board_size:&BoardSize) -> String {
        if self.is_on_board() {
            let (file, rank) = board_size.cell_to_file_rank(self.index);
            format!("{}{}", file, i8_to_rank_char(rank))
        } else if self.is_hand() {
            // 持ち駒
            use board::Piece::*;
            match self.index {
                82 => { format!("{}*", piece_to_sign(Some(K1)))},
                83 => { format!("{}*", piece_to_sign(Some(R1)))},
                84 => { format!("{}*", piece_to_sign(Some(B1)))},
                85 => { format!("{}*", piece_to_sign(Some(G1)))},
                86 => { format!("{}*", piece_to_sign(Some(S1)))},
                87 => { format!("{}*", piece_to_sign(Some(N1)))},
                88 => { format!("{}*", piece_to_sign(Some(L1)))},
                89 => { format!("{}*", piece_to_sign(Some(P1)))},
                90 => { format!("{}*", piece_to_sign(Some(K2)))},
                91 => { format!("{}*", piece_to_sign(Some(R2)))},
                92 => { format!("{}*", piece_to_sign(Some(B2)))},
                93 => { format!("{}*", piece_to_sign(Some(G2)))},
                94 => { format!("{}*", piece_to_sign(Some(S2)))},
                95 => { format!("{}*", piece_to_sign(Some(N2)))},
                96 => { format!("{}*", piece_to_sign(Some(L2)))},
                97 => { format!("{}*", piece_to_sign(Some(P2)))},
                98 => { format!("{}*", piece_to_sign(Some(K3)))},
                99 => { format!("{}*", piece_to_sign(Some(R3)))},
                100 => { format!("{}*", piece_to_sign(Some(B3)))},
                101 => { format!("{}*", piece_to_sign(Some(G3)))},
                102 => { format!("{}*", piece_to_sign(Some(S3)))},
                103 => { format!("{}*", piece_to_sign(Some(N3)))},
                104 => { format!("{}*", piece_to_sign(Some(L3)))},
                105 => { format!("{}*", piece_to_sign(Some(P3)))},
                _ => {panic!("Unexpected index print: {0}.", self.index);}
            }
        } else if self.index == 105 {
            "Sky".to_string()
        } else {
            panic!("Unexpected index print: {0}.", self.index);
        }
    }
}