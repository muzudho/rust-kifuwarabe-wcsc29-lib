use instrument::half_player_phase::HalfPlayerPhaseValue;
use instrument::piece_etc::*;
use std::fmt;
use studio::board_size::*;

/// TODO 指先マス。暫定。
pub const FINGERTIP_ADDRESS: usize = 81;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    file: i8,
    rank: i8,
}
impl Cell {
    /// ボード・サイズは考慮しません。
    pub fn from_file_rank(file_num: i8, rank_num: i8) -> Cell {
        Cell {
            file: file_num,
            rank: rank_num,
        }
    }

    pub fn from_scalar(scalar: i8) -> Cell {
        Cell {
            file: scalar / 10,
            rank: scalar % 10,
        }
    }

    pub fn get_file(self) -> i8 {
        self.file
    }

    pub fn get_rank(self) -> i8 {
        self.rank
    }

    pub fn to_scalar(self) -> i8 {
        self.file * 10 + self.rank
    }

    // For log.
    pub fn to_human_presentable(self) -> String {
        self.to_scalar().to_string()
    }
}

/// Vector に入れるときコピーする。
#[derive(Clone, Copy, PartialEq)] // Debug,
pub struct Address {
    index: usize,
}
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ad.{}", self.index)
    }
}
impl Address {
    pub fn from_raw(raw: usize) -> Self {
        Address { index: raw }
    }

    /// 盤上の駒の番地。
    pub fn from_cell(cell: Cell, board_size: BoardSize) -> Self {
        Address {
            index: board_size.cell_to_address(cell),
        }
    }

    pub fn from_fingertip() -> Self {
        Address {
            index: FINGERTIP_ADDRESS,
        }
    }

    pub fn from_hand_pi(pi: Piece) -> Self {
        Address::from_hand_ph_pt(pi.get_phase(), pi.get_type())
    }

    pub fn from_hand_ph_pt(phase_value: HalfPlayerPhaseValue, pt: PieceType) -> Self {
        use instrument::half_player_phase::HalfPlayerPhaseValue::*;
        use instrument::piece_etc::PieceType::*;

        let index_num = match phase_value {
            First => match pt {
                K => 82,
                R | PR => 83,
                B | PB => 84,
                G => 85,
                S | PS => 86,
                N | PN => 87,
                L | PL => 88,
                P | PP => 89,
                _ => panic!("Unexpected first hand piece_type {}.", pt.to_sign()),
            },
            Second => match pt {
                K => 90,
                R | PR => 91,
                B | PB => 92,
                G => 93,
                S | PS => 94,
                N | PN => 95,
                L | PL => 96,
                P | PP => 97,
                _ => panic!("Unexpected second hand piece_type {}.", pt.to_sign()),
            },
            OnePointFive | ZeroPointFive => match pt {
                K => 98,
                R | PR => 99,
                B | PB => 100,
                G => 101,
                S | PS => 102,
                N | PN => 103,
                L | PL => 104,
                P | PP => 105,
                _ => panic!("Unexpected 0.5, 1.5 hand piece_type {}.", pt.to_sign()),
            },
        };

        Address { index: index_num }
    }

    /// Human presentable.
    pub fn to_human_presentable(self, board_size: BoardSize) -> String {
        if self.is_on_board(board_size) {
            // 盤上。
            board_size
                .address_to_cell(self.index)
                .to_human_presentable()
        // それ以外。
        } else if let Some(piece) = self.get_hand_piece() {
            format!("Hand: {}", piece.to_sign()).to_string()
        } else {
            panic!("Unexpected address: {}.", self.index);
        }
    }

    /// 盤上。
    pub fn is_on_board(self, board_size: BoardSize) -> bool {
        self.index < board_size.len()
    }

    /// 駒台
    pub fn is_hand(self) -> bool {
        // TODO マジックナンバーを解消したい。
        82 <= self.index && self.index <= 105
    }

    pub fn is_fingertip(self) -> bool {
        FINGERTIP_ADDRESS == self.index
    }

    /// 盤上であれば、セル番地へ変換。それ以外は None。
    pub fn to_cell(self, board_size: BoardSize) -> Option<Cell> {
        if self.is_on_board(board_size) {
            Some(board_size.address_to_cell(self.index))
        } else {
            None
        }
    }

    pub fn get_index(self) -> usize {
        self.index
    }

    pub fn get_hand_index(self) -> usize {
        self.index - FINGERTIP_ADDRESS - 1
    }

    /// 持ち駒
    pub fn get_hand_piece(self) -> Option<Piece> {
        // TODO マジックナンバーを解消したい。
        use instrument::piece_etc::Piece::*;
        match self.index {
            82 => Some(K1),
            83 => Some(R1),
            84 => Some(B1),
            85 => Some(G1),
            86 => Some(S1),
            87 => Some(N1),
            88 => Some(L1),
            89 => Some(P1),

            90 => Some(K2),
            91 => Some(R2),
            92 => Some(B2),
            93 => Some(G2),
            94 => Some(S2),
            95 => Some(N2),
            96 => Some(L2),
            97 => Some(P2),

            98 => Some(K3),
            99 => Some(R3),
            100 => Some(B3),
            101 => Some(G3),
            102 => Some(S3),
            103 => Some(N3),
            104 => Some(L3),
            105 => Some(P3),
            _ => {
                None
                // panic!("Unexpected index print: {0}.", self.index);
            }
        }
    }

    /// 基本2桁、Sky 3桁。（指先のことを Sky と表示する仕様）
    pub fn to_physical_sign(self, board_size: BoardSize) -> String {
        if self.is_on_board(board_size) {
            let cell = board_size.address_to_cell(self.index);
            cell.to_scalar().to_string()
        } else if self.is_fingertip() {
            "Sky".to_string()
        } else if self.is_hand() {
            if let Some(piece) = self.get_hand_piece() {
                // 持ち駒
                format!(
                    "0{}", // "{}*"
                    piece.to_sign()
                )
            } else {
                panic!("Unexpected index print(10): {0}.", self.index);
            }
        } else {
            panic!("Unexpected index print(20): {0}.", self.index);
        }
    }
}
