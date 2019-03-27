use parser::*;
use piece_etc::*;
use position::*;

/// Vector に入れるときコピーする。
#[derive(Clone, Copy, PartialEq)]
pub struct Address {
    index: usize,
}
impl Address {
    pub fn create_by_cell(file_num:i8, rank_num:i8, board_size:BoardSize) -> Address {
        Address {
            index: board_size.file_rank_to_cell(file_num, rank_num),
        }
    }

    pub fn create_by_hand(phase_opt:Option<Phase>, piece_type:PieceType) -> Address {
        use piece_etc::Phase::*;
        use piece_etc::PieceType::*;

        let index_num = match phase_opt {
            Some(phase) => {
                match phase {
                    First => {
                        match piece_type {
                            K => {82},
                            R | PR => {83},
                            B | PB => {84},
                            G => {85},
                            S | PS => {86},
                            N | PN => {87},
                            L | PL => {88},
                            P | PP => {89},
                            _ => panic!("Unexpected hand piece_type {}.", piece_type_to_sign(Some(piece_type)))
                        }
                    },
                    Second => {
                        match piece_type {
                            K => {90},
                            R | PR => {91},
                            B | PB => {92},
                            G => {93},
                            S | PS => {94},
                            N | PN => {95},
                            L | PL => {96},
                            P | PP => {97},
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

    pub fn get_index(self) -> usize {
        self.index
    }

    pub fn get_hand_index(self) -> usize {
        self.index - SKY_ADDRESS - 1
    }

    pub fn get_hand_piece(self) -> Option<Piece> {
        // 持ち駒
        use piece_etc::Piece::*;
        match self.index {
            82 => { Some(K1)},
            83 => { Some(R1)},
            84 => { Some(B1)},
            85 => { Some(G1)},
            86 => { Some(S1)},
            87 => { Some(N1)},
            88 => { Some(L1)},
            89 => { Some(P1)},
            90 => { Some(K2)},
            91 => { Some(R2)},
            92 => { Some(B2)},
            93 => { Some(G2)},
            94 => { Some(S2)},
            95 => { Some(N2)},
            96 => { Some(L2)},
            97 => { Some(P2)},
            98 => { Some(K3)},
            99 => { Some(R3)},
            100 => { Some(B3)},
            101 => { Some(G3)},
            102 => { Some(S3)},
            103 => { Some(N3)},
            104 => { Some(L3)},
            105 => { Some(P3)},
            _ => {panic!("Unexpected index print: {0}.", self.index);}
        }
    }

    /// 盤上。
    pub fn is_on_board(self, board_size:BoardSize) -> bool {
        self.index < board_size.len()
    }

    /// 駒台
    pub fn is_hand(self) -> bool {
        81 <= self.index && self.index <= 104
    }

    pub fn to_physical_sign(self, board_size:BoardSize) -> String {
        if self.is_on_board(board_size) {
            let (file, rank) = board_size.cell_to_file_rank(self.index);
            format!("{}{}", file, Parser::i8_to_rank_char(rank))
        } else if self.is_hand() {
            // 持ち駒
            format!("{}*", piece_to_sign(self.get_hand_piece()))
        } else if self.index == 105 {
            "Sky".to_string()
        } else {
            panic!("Unexpected index print: {0}.", self.index);
        }
    }
}