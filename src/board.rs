use address::*;
use position::*;
use logical_record::*;
use physical_move::*;

pub fn file_char_to_i8(ch:char) -> i8 {
    match ch {
        '1' => {1},
        '2' => {2},
        '3' => {3},
        '4' => {4},
        '5' => {5},
        '6' => {6},
        '7' => {7},
        '8' => {8},
        '9' => {9},
        _ => {panic!("Unexpected file char: '{0}'", ch)},
    }
}
pub fn rank_char_to_i8(ch:char) -> i8 {
    match ch {
        'a' => {1},
        'b' => {2},
        'c' => {3},
        'd' => {4},
        'e' => {5},
        'f' => {6},
        'g' => {7},
        'h' => {8},
        'i' => {9},
        _ => {panic!("Unexpected rank char: '{0}'", ch)},
    }
}

pub const DEFAULT_FILE_LEN: usize = 9;
pub const DEFAULT_RANK_LEN: usize = 9;
pub const HANDS_LEN: usize = 3 * 8;
pub const SKY_LEN: usize = 1;
pub const SKY_ADDRESS: usize = 89;
pub const DEFAULT_BOARD_SIZE: usize = (DEFAULT_FILE_LEN * DEFAULT_RANK_LEN + HANDS_LEN + SKY_LEN) as usize;

pub struct Board {
    pub file_len: i8,
    pub rank_len: i8,
    pub board_size: usize,
    pub pieces: [Option<Piece>; DEFAULT_BOARD_SIZE],
    /// R, B, G, S, N, L, P, r, b, g, s, n, l, p.
    pub hands: [i8; HANDS_LEN],
}
impl Board {
    pub fn new() -> Board {
        Board {
            file_len: DEFAULT_FILE_LEN as i8,
            rank_len: DEFAULT_RANK_LEN as i8,
            board_size: (DEFAULT_RANK_LEN * DEFAULT_FILE_LEN) as usize,
            pieces: [None; DEFAULT_BOARD_SIZE],
            hands: [0; HANDS_LEN],
        }
    }

    pub fn set_startpos(&mut self) {
        use position::Piece::*;
        // Flip horizontal.
        self.pieces  = [
            Some(L2), Some(N2), Some(S2), Some(G2), Some(K2), Some(G2), Some(S2), Some(N2), Some(L2),
            None, Some(B2), None, None, None, None, None, Some(R2), None,
            Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2),
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1),
            None, Some(R1), None, None, None, None, None, Some(B1), None,
            Some(L1), Some(N1), Some(S1), Some(G1), Some(K1), Some(G1), Some(S1), Some(N1), Some(L1),
            None, None, None, None, None, None, None, None, // First phase.
            None, None, None, None, None, None, None, None, // Second phase.
            None, None, None, None, None, None, None, None, // None phase.
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
            K1 => {self.hands[0]},
            R1 => {self.hands[1]},
            B1 => {self.hands[2]},
            G1 => {self.hands[3]},
            S1 => {self.hands[4]},
            N1 => {self.hands[5]},
            L1 => {self.hands[6]},
            P1 => {self.hands[7]},
            K2 => {self.hands[8]},
            R2 => {self.hands[9]},
            B2 => {self.hands[10]},
            G2 => {self.hands[11]},
            S2 => {self.hands[12]},
            N2 => {self.hands[13]},
            L2 => {self.hands[14]},
            P2 => {self.hands[15]},
            K3 => {self.hands[16]},
            R3 => {self.hands[17]},
            B3 => {self.hands[18]},
            G3 => {self.hands[19]},
            S3 => {self.hands[20]},
            N3 => {self.hands[21]},
            L3 => {self.hands[22]},
            P3 => {self.hands[23]},
            _ => panic!("Unexpected hand '{}'.", piece_to_sign(Some(*piece))),
        }
    }

    /// Obsolute. new --> add().
    pub fn set_hand(&mut self, piece:Piece, num:i8) {
        use position::Piece::*;
        match piece {
            K1 => {self.hands[0] = num},
            R1 => {self.hands[1] = num},
            B1 => {self.hands[2] = num},
            G1 => {self.hands[3] = num},
            S1 => {self.hands[4] = num},
            N1 => {self.hands[5] = num},
            L1 => {self.hands[6] = num},
            P1 => {self.hands[7] = num},
            K2 => {self.hands[8] = num},
            R2 => {self.hands[9] = num},
            B2 => {self.hands[10] = num},
            G2 => {self.hands[11] = num},
            S2 => {self.hands[12] = num},
            N2 => {self.hands[13] = num},
            L2 => {self.hands[14] = num},
            P2 => {self.hands[15] = num},
            K3 => {self.hands[16] = num},
            R3 => {self.hands[17] = num},
            B3 => {self.hands[18] = num},
            G3 => {self.hands[19] = num},
            S3 => {self.hands[20] = num},
            N3 => {self.hands[21] = num},
            L3 => {self.hands[22] = num},
            P3 => {self.hands[23] = num},
            _ => panic!("Unexpected hand '{}'.", piece_to_sign(Some(piece))),
        }
    }

    pub fn touch(&mut self, physical_move:PhysicalMove) {
        match physical_move.address {
            Some(address) => {
                match self.pieces[address.index] {
                    Some(piece) => {
                        match self.pieces[SKY_ADDRESS] {
                            Some(piece) => {

                            },
                            None => {
                                self.pieces[SKY_ADDRESS] = Some(piece);
                                self.pieces[address.index] = None;
                            },
                        }
                    },
                    None => {
                        match self.pieces[SKY_ADDRESS] {
                            Some(piece) => {
                                self.pieces[SKY_ADDRESS] = None;
                                self.pieces[address.index] = Some(piece);
                            },
                            None => {
                            },
                        }
                    },
                }
            },
            None => {

            }
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
                            R1 | R2 => {self.hands[0] += 1},
                            B1 | B2 => {self.hands[1] += 1},
                            G1 | G2 => {self.hands[2] += 1},
                            S1 | S2 => {self.hands[3] += 1},
                            N1 | N2 => {self.hands[4] += 1},
                            L1 | L2 => {self.hands[5] += 1},
                            P1 | P2 => {self.hands[6] += 1},
                            _ => panic!("Unexpected hand '{}' on first.", piece_to_sign(Some(piece))),
                        }
                    },
                    Second => {
                        match piece {
                            R1 | R2 => {self.hands[7] += 1},
                            B1 | B2 => {self.hands[8] += 1},
                            G1 | G2 => {self.hands[9] += 1},
                            S1 | S2 => {self.hands[10] += 1},
                            N1 | N2 => {self.hands[11] += 1},
                            L1 | L2 => {self.hands[12] += 1},
                            P1 | P2 => {self.hands[13] += 1},
                            _ => panic!("Unexpected hand '{}' on second.", piece_to_sign(Some(piece))),
                        }
                    },
                }
            },
            None => {
                match self.pieces[address.index] {
                    Some(piece2) => panic!("Piece already exists '{}'.", piece_to_sign(Some(piece2))),
                    None => {
                        self.pieces[address.index] = Some(piece);
                    },
                }
            },
        }
    }

    pub fn print_hand(&self, phase_opt:Option<Phase>, piece_type:PieceType) -> String {
        let piece = match phase_opt {
            Some(phase) => {piece_type_to_piece(phase, piece_type)},
            None => {
                // 使っていない駒。とりあえず先手を指定。
                piece_type_to_piece(Phase::First, piece_type)
            },
        };
        let count = self.get_hand(&piece);
        let num_label = if 1 < count {count.to_string()} else {"".to_string()};
        let ch = if 0 < count {
            piece_type_to_sign(Some(piece_type))
        } else {
            "".to_string()
        };

        format!("{}{}", num_label, ch)
    }

    /// Point of symmetory.
    pub fn print(&self, phase:Phase) {
        use position::Phase::*;
        let rank_array = ['?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

        println!("              {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>3}",
            self.print_hand(Some(Phase::First), PieceType::K),
            self.print_hand(Some(Phase::First), PieceType::R),
            self.print_hand(Some(Phase::First), PieceType::B),
            self.print_hand(Some(Phase::First), PieceType::G),
            self.print_hand(Some(Phase::First), PieceType::S),
            self.print_hand(Some(Phase::First), PieceType::N),
            self.print_hand(Some(Phase::First), PieceType::L),
            self.print_hand(Some(Phase::First), PieceType::P)
            );

        match phase {
            First => {
                // hand.
                println!("|         |  +-------------------+");
            },
            Second => {
                println!("             +-------------------+");
            },
        }

        for y in 0..=8 {
            let rank = 9 - y;

            // 先手の手。
            match phase {
                First => {
                    match rank {
                        9 => {print!("|         | ")},
                        8 => {print!("+---+ +-+ | ")},
                        7 => {print!("    | | | | ")},
                        6 => {print!("    | | | | ")},
                        5 => {print!("    +-+ +-+ ")},
                        4 => {print!("       {:1}    ", piece_to_sign(self.get_piece_by_address(SKY_ADDRESS as i8)))},
                        3 => {print!("            ")},
                        2 => {print!("            ")},
                        1 => {print!("            ")},
                        _ => {},
                    };
                },
                Second => {print!("            ")},
            }

            print!(
                "{0}|{1: >2}{2: >2}{3: >2}{4: >2}{5: >2}{6: >2}{7: >2}{8: >2}{9: >2}",
                rank_array[rank as usize],
                piece_to_sign(self.get_piece(1, rank)),
                piece_to_sign(self.get_piece(2, rank)),
                piece_to_sign(self.get_piece(3, rank)),
                piece_to_sign(self.get_piece(4, rank)),
                piece_to_sign(self.get_piece(5, rank)),
                piece_to_sign(self.get_piece(6, rank)),
                piece_to_sign(self.get_piece(7, rank)),
                piece_to_sign(self.get_piece(8, rank)),
                piece_to_sign(self.get_piece(9, rank)));

            // Right boarder and None phase hands.
            match rank {
                9 => {print!(" |   ")},
                8 => {print!(" |{:>3}", self.print_hand(None, PieceType::K))},
                7 => {print!(" |{:>3}", self.print_hand(None, PieceType::R))},
                6 => {print!(" |{:>3}", self.print_hand(None, PieceType::B))},
                5 => {print!(" |{:>3}", self.print_hand(None, PieceType::G))},
                4 => {print!(" |{:>3}", self.print_hand(None, PieceType::S))},
                3 => {print!(" |{:>3}", self.print_hand(None, PieceType::N))},
                2 => {print!(" |{:>3}", self.print_hand(None, PieceType::L))},
                1 => {print!(" |{:>3}", self.print_hand(None, PieceType::P))},
                _ => {},
            };

            // Second player finger.
            match phase {
                First => {},
                Second => {
                    match rank {
                        9 => {},
                        8 => {},
                        6 => {print!("    {:1}", piece_to_sign(self.get_piece_by_address(SKY_ADDRESS as i8)))},
                        5 => {print!(" +-+ +-+")},
                        4 => {print!(" | | | |")},
                        3 => {print!(" | | | |")},
                        2 => {print!(" | +-+ +---+")},
                        1 => {print!(" |         |")},
                        _ => {},
                    };                    
                },
            }

            println!();
        }

        match phase {
            First => {
                println!("             +-------------------+");
            },
            Second => {
                // hand.
                println!("             +-------------------+    |         |");
            },
        }

        println!("               1 2 3 4 5 6 7 8 9");

        // Second phase hand.
        println!("              {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>3}",
            self.print_hand(Some(Phase::Second), PieceType::K),
            self.print_hand(Some(Phase::Second), PieceType::R),
            self.print_hand(Some(Phase::Second), PieceType::B),
            self.print_hand(Some(Phase::Second), PieceType::G),
            self.print_hand(Some(Phase::Second), PieceType::S),
            self.print_hand(Some(Phase::Second), PieceType::N),
            self.print_hand(Some(Phase::Second), PieceType::L),
            self.print_hand(Some(Phase::Second), PieceType::P)
            );
    }
}