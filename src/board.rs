use address::*;
use position::*;

pub const DEFAULT_FILE_LEN: i8 = 9;
pub const DEFAULT_RANK_LEN: i8 = 9;
pub const HANDS_LEN: i8 = 14;
pub const SKY_LEN: i8 = 1;
pub const SKY_ADDRESS: i8 = 89;
pub const DEFAULT_BOARD_SIZE: usize = (DEFAULT_FILE_LEN * DEFAULT_RANK_LEN + HANDS_LEN + SKY_LEN) as usize;
pub const HANDS_SIZE: usize = 14;

pub struct Board {
    pub file_len: i8,
    pub rank_len: i8,
    pub board_size: usize,
    pub pieces: [Option<Piece>; DEFAULT_BOARD_SIZE],
    /// R, B, G, S, N, L, P, r, b, g, s, n, l, p.
    pub hands: [i8; HANDS_SIZE],
}
impl Board {
    pub fn new() -> Board {
        Board {
            file_len: DEFAULT_FILE_LEN,
            rank_len: DEFAULT_RANK_LEN,
            board_size: (DEFAULT_RANK_LEN * DEFAULT_FILE_LEN) as usize,
            pieces: [None; DEFAULT_BOARD_SIZE],
            hands: [0; HANDS_SIZE],
        }
    }

    pub fn set_startpos(&mut self) {
        use position::Piece::*;
        // Flip horizontal.
        self.pieces  = [
            Some(L1), Some(N1), Some(S1), Some(G1), Some(K1), Some(G1), Some(S1), Some(N1), Some(L1),
            None, Some(B1), None, None, None, None, None, Some(R1), None,
            Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1),
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0),
            None, Some(R0), None, None, None, None, None, Some(B0), None,
            Some(L0), Some(N0), Some(S0), Some(G0), Some(K0), Some(G0), Some(S0), Some(N0), Some(L0),
            None, None, None, None, None, None, None, // Do not use.
            None, None, None, None, None, None, None, // Do not use.
            None, // Sky
        ];
    }

    pub fn file_rank_to_cell(&self, file:i8, rank:i8) -> usize {
        ((rank-1)*self.file_len + (file-1)) as usize
    }
    pub fn cell_to_file_rank(&self, cell:usize) -> (i8, i8) {
        ((cell%self.file_len as usize) as i8, (cell/self.file_len as usize) as i8)
    }
    pub fn reverse_cell(&self, cell:usize) -> usize {
        self.rank_len as usize * self.file_len as usize - cell
    }

    pub fn get_piece(&self, file:i8, rank:i8) -> Option<Piece> {
        let address = self.file_rank_to_cell(file, rank);
        self.pieces[address]
    }

    pub fn get_piece_by_address(&self, address:i8) -> Option<Piece> {
        self.pieces[address as usize]
    }

    /// Obsolute. new --> add().
    pub fn set_piece(&mut self, file:i8, rank:i8, piece:Option<Piece>) {
        let cell = self.file_rank_to_cell(file, rank);
        self.pieces[cell] = piece;
    }

    pub fn get_hand(&self, piece:&Piece) -> i8 {
        use position::Piece::*;
        match *piece {
            R0 => {self.hands[0]},
            B0 => {self.hands[1]},
            G0 => {self.hands[2]},
            S0 => {self.hands[3]},
            N0 => {self.hands[4]},
            L0 => {self.hands[5]},
            P0 => {self.hands[6]},
            R1 => {self.hands[7]},
            B1 => {self.hands[8]},
            G1 => {self.hands[9]},
            S1 => {self.hands[10]},
            N1 => {self.hands[11]},
            L1 => {self.hands[12]},
            P1 => {self.hands[13]},
            _ => panic!("Unexpected hand '{}'.", piece_to_sign(&Some(*piece))),
        }
    }

    /// Obsolute. new --> add().
    pub fn set_hand(&mut self, piece:&Piece, num:i8) {
        use position::Piece::*;
        match *piece {
            R0 => {self.hands[0] = num},
            B0 => {self.hands[1] = num},
            G0 => {self.hands[2] = num},
            S0 => {self.hands[3] = num},
            N0 => {self.hands[4] = num},
            L0 => {self.hands[5] = num},
            P0 => {self.hands[6] = num},
            R1 => {self.hands[7] = num},
            B1 => {self.hands[8] = num},
            G1 => {self.hands[9] = num},
            S1 => {self.hands[10] = num},
            N1 => {self.hands[11] = num},
            L1 => {self.hands[12] = num},
            P1 => {self.hands[13] = num},
            _ => panic!("Unexpected hand '{}'.", piece_to_sign(&Some(*piece))),
        }
    }

    /// latest.
    pub fn add(&mut self, address:&Address, piece:Piece) {
        match address.hand {
            Some(phase) => {
                use position::Phase::*;
                use position::Piece::*;
                match phase {
                    First => {
                        match piece {
                            R0 | R1 => {self.hands[0] += 1},
                            B0 | B1 => {self.hands[1] += 1},
                            G0 | G1 => {self.hands[2] += 1},
                            S0 | S1 => {self.hands[3] += 1},
                            N0 | N1 => {self.hands[4] += 1},
                            L0 | L1 => {self.hands[5] += 1},
                            P0 | P1 => {self.hands[6] += 1},
                            _ => panic!("Unexpected hand '{}' on first.", piece_to_sign(&Some(piece))),
                        }
                    },
                    Second => {
                        match piece {
                            R0 | R1 => {self.hands[7] += 1},
                            B0 | B1 => {self.hands[8] += 1},
                            G0 | G1 => {self.hands[9] += 1},
                            S0 | S1 => {self.hands[10] += 1},
                            N0 | N1 => {self.hands[11] += 1},
                            L0 | L1 => {self.hands[12] += 1},
                            P0 | P1 => {self.hands[13] += 1},
                            _ => panic!("Unexpected hand '{}' on second.", piece_to_sign(&Some(piece))),
                        }
                    },
                    _ => panic!("Unexpected phase: {}.", phase_to_sign(&phase)),
                }
            },
            None => {
                let cell = self.file_rank_to_cell(address.file, address.rank);
                match self.pieces[cell] {
                    Some(piece2) => panic!("Piece already exists '{}'.", piece_to_sign(&Some(piece2))),
                    None => {
                        self.pieces[cell] = Some(piece);
                    },
                }
            },
        }
    }

    /// Point of symmetory.
    pub fn print(&self) {
        use position::Piece::*;
        let rank_array = ['?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

        // First phase hand.
        let h = [
            self.get_hand(&R0),
            self.get_hand(&B0),
            self.get_hand(&G0),
            self.get_hand(&S0),
            self.get_hand(&N0),
            self.get_hand(&L0),
            self.get_hand(&P0),
            self.get_hand(&R1),
            self.get_hand(&B1),
            self.get_hand(&G1),
            self.get_hand(&S1),
            self.get_hand(&N1),
            self.get_hand(&L1),
            self.get_hand(&P1),
        ];
        println!("  {}{} {}{} {}{} {}{} {}{} {}{} {:2>}{}",
            if 1 < h[0] { h[0].to_string() } else {" ".to_string()},
            if 0 < h[0] { "R"} else {" "},
            if 1 < h[1] { h[1].to_string() } else {" ".to_string()},
            if 0 < h[1] { "B"} else {" "},
            if 1 < h[2] { h[2].to_string() } else {" ".to_string()},
            if 0 < h[2] { "G"} else {" "},
            if 1 < h[3] { h[3].to_string() } else {" ".to_string()},
            if 0 < h[3] { "S"} else {" "},
            if 1 < h[4] { h[4].to_string() } else {" ".to_string()},
            if 0 < h[4] { "N"} else {" "},
            if 1 < h[5] { h[5].to_string() } else {" ".to_string()},
            if 0 < h[5] { "L"} else {" "},
            if 1 < h[6] { h[6].to_string() } else {"  ".to_string()},
            if 0 < h[6] { "P"} else {" "}
            );
        println!("  ------------------");

        for y in 0..=8 {
            let rank = 9 - y;
            print!(
                "{0} {1: >2}{2: >2}{3: >2}{4: >2}{5: >2}{6: >2}{7: >2}{8: >2}{9: >2}",
                rank_array[rank as usize],
                piece_to_sign(&self.get_piece(1, rank)),
                piece_to_sign(&self.get_piece(2, rank)),
                piece_to_sign(&self.get_piece(3, rank)),
                piece_to_sign(&self.get_piece(4, rank)),
                piece_to_sign(&self.get_piece(5, rank)),
                piece_to_sign(&self.get_piece(6, rank)),
                piece_to_sign(&self.get_piece(7, rank)),
                piece_to_sign(&self.get_piece(8, rank)),
                piece_to_sign(&self.get_piece(9, rank)));
            
            if rank == 6 {
                print!(" +--+");
            } else if rank == 5 {
                print!(" |{:>2}|", piece_to_sign(&self.get_piece_by_address(SKY_ADDRESS)));
            } else if rank == 4 {
                print!(" +--+");
            }

            println!();
        }
        println!("   1 2 3 4 5 6 7 8 9");
        println!("  ------------------");
        println!("  {}{} {}{} {}{} {}{} {}{} {}{} {:2>}{}",
            if 1 < h[7] { h[0].to_string() } else {" ".to_string()},
            if 0 < h[7] { "r"} else {" "},
            if 1 < h[8] { h[1].to_string() } else {" ".to_string()},
            if 0 < h[8] { "b"} else {" "},
            if 1 < h[9] { h[2].to_string() } else {" ".to_string()},
            if 0 < h[9] { "g"} else {" "},
            if 1 < h[10] { h[3].to_string() } else {" ".to_string()},
            if 0 < h[10] { "s"} else {" "},
            if 1 < h[11] { h[4].to_string() } else {" ".to_string()},
            if 0 < h[11] { "n"} else {" "},
            if 1 < h[12] { h[5].to_string() } else {" ".to_string()},
            if 0 < h[12] { "l"} else {" "},
            if 1 < h[13] { h[6].to_string() } else {"  ".to_string()},
            if 0 < h[13] { "p"} else {" "}
            );

        // Second phase hand.
    }
}