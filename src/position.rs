use parser::*;
use physical_move::*;
use physical_record::*;
use std::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    /// Starting first.
    First,
    /// Starting second.
    Second,
}

/// First phase is 1.
/// Second phase is 2.
/// None phase is 3.
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    // King is 玉.
    K1 = 0,
    // Rook is 飛.
    R1,
    // Bishop is 角.
    B1,
    // Gold is 金.
    G1,
    // Silver is 銀.
    S1,
    // kNight is 桂.
    N1,
    // Lance is 香.
    L1,
    // Pawn is 歩.
    P1,
    // Promoted rook is 竜.
    PR1,
    // Promoted bishop is 馬.
    PB1,
    // Promoted silver is 成銀.
    PS1,
    // Promoted knight is 成桂.
    PN1,
    // Promoted lance is 成香.
    PL1,
    // Promoted pawn is と.
    PP1,
    K2,
    R2,
    B2,
    G2,
    S2,
    N2,
    L2,
    P2,
    PR2,
    PB2,
    PS2,
    PN2,
    PL2,
    PP2,
    K3,
    R3,
    B3,
    G3,
    S3,
    N3,
    L3,
    P3,
    PR3,
    PB3,
    PS3,
    PN3,
    PL3,
    PP3,
}
pub fn piece_to_sign(piece:Option<Piece>) -> String {
    match piece {
        Some(x) => {
            use position::Piece::*;
            match x {
                K1 => "K",
                R1 => "R",
                B1 => "B",
                G1 => "G",
                S1 => "S",
                N1 => "N",
                L1 => "L",
                P1 => "P",
                PR1 => "+R",
                PB1 => "+B",
                PS1 => "+S",
                PN1 => "+N",
                PL1 => "+L",
                PP1 => "+P",
                K2 => "k",
                R2 => "r",
                B2 => "b",
                G2 => "g",
                S2 => "s",
                N2 => "n",
                L2 => "l",
                P2 => "p",
                PR2 => "+r",
                PB2 => "+b",
                PS2 => "+s",
                PN2 => "+n",
                PL2 => "+l",
                PP2 => "+p",
                K3 => "K",
                R3 => "R",
                B3 => "B",
                G3 => "G",
                S3 => "S",
                N3 => "N",
                L3 => "L",
                P3 => "P",
                PR3 => "+R",
                PB3 => "+B",
                PS3 => "+S",
                PN3 => "+N",
                PL3 => "+L",
                PP3 => "+P",
            }
        },
        None => { "" }
    }.to_string()
}
pub fn piece_to_piece_type(piece:Piece) -> PieceType {
    use position::Piece::*;
    use physical_record::PieceType::*;
    match piece {
        K1 => K,
        R1 => R,
        B1 => B,
        G1 => G,
        S1 => S,
        N1 => N,
        L1 => L,
        P1 => P,
        PR1 => PR,
        PB1 => PB,
        PS1 => PS,
        PN1 => PN,
        PL1 => PL,
        PP1 => PP,
        K2 => K,
        R2 => R,
        B2 => B,
        G2 => G,
        S2 => S,
        N2 => N,
        L2 => L,
        P2 => P,
        PR2 => PR,
        PB2 => PB,
        PS2 => PS,
        PN2 => PN,
        PL2 => PL,
        PP2 => PP,
        K3 => K,
        R3 => R,
        B3 => B,
        G3 => G,
        S3 => S,
        N3 => N,
        L3 => L,
        P3 => P,
        PR3 => PR,
        PB3 => PB,
        PS3 => PS,
        PN3 => PN,
        PL3 => PL,
        PP3 => PP,
    }
}
pub fn piece_to_phase(piece:Option<Piece>) -> Option<Phase> {
    match piece {
        Some(x) => {
            use position::Piece::*;
            match x {
                K1 | R1 | B1 | G1 | S1 | N1 | L1 | P1 | PR1 | PB1 | PS1 | PN1 | PL1 | PP1 => Some(Phase::First),
                K2 | R2 | B2 | G2 | S2 | N2 | L2 | P2 | PR2 | PB2 | PS2 | PN2 | PL2 | PP2 => Some(Phase::Second),
                _ => panic!("Unexpected phase. *piece as usize = {}.", x as usize),
            }
        },
        None => None,
    }
}

pub fn promotion_piece(piece:Option<Piece>) -> Option<Piece> {
    match piece {
        Some(x) => {
            use position::Piece::*;
            match x {
                R1 => Some(PR1),
                B1 => Some(PB1),
                S1 => Some(PS1),
                N1 => Some(PN1),
                L1 => Some(PL1),
                P1 => Some(PP1),
                R2 => Some(PR2),
                B2 => Some(PB2),
                S2 => Some(PS2),
                N2 => Some(PN2),
                L2 => Some(PL2),
                P2 => Some(PP2),
                _ => panic!("Failed: Sfen unexpected promotion.")
            }
        },
        None => None,
    }
}
pub fn rotate_piece(piece:Option<Piece>) -> Option<Piece> {
    match piece {
        Some(x) => {
            use position::Piece::*;
            match x {
                K1 => Some(K2),
                R1 => Some(R2),
                B1 => Some(B2),
                G1 => Some(G2),
                S1 => Some(S2),
                N1 => Some(N2),
                L1 => Some(L2),
                P1 => Some(P2),
                PR1 => Some(PR2),
                PB1 => Some(PB2),
                PS1 => Some(PS2),
                PN1 => Some(PN2),
                PL1 => Some(PL2),
                PP1 => Some(PP2),
                K2 => Some(K1),
                R2 => Some(R1),
                B2 => Some(B1),
                G2 => Some(G1),
                S2 => Some(S1),
                N2 => Some(N1),
                L2 => Some(L1),
                P2 => Some(P1),
                PR2 => Some(PR1),
                PB2 => Some(PB1),
                PS2 => Some(PS1),
                PN2 => Some(PN1),
                PL2 => Some(PL1),
                PP2 => Some(PP1),
                K3 => Some(K3),
                R3 => Some(R3),
                B3 => Some(B3),
                G3 => Some(G3),
                S3 => Some(S3),
                N3 => Some(N3),
                L3 => Some(L3),
                P3 => Some(P3),
                PR3 => Some(PR3),
                PB3 => Some(PB3),
                PS3 => Some(PS3),
                PN3 => Some(PN3),
                PL3 => Some(PL3),
                PP3 => Some(PP3),
            }
        },
        None => { None }
    }
}
pub fn is_promotion_piece(piece_opt:Option<Piece>) -> bool {
    match piece_opt {
        Some(piece) => {
            use position::Piece::*;
            match piece {
                PR1 | PB1 | PS1 | PN1 | PL1 | PP1 |
                PR2 | PB2 | PS2 | PN2 | PL2 | PP2 |
                PR3 | PB3 | PS3 | PN3 | PL3 | PP3 => true,
                _ => false,
            }
        },
        None => false,
    }
}

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
pub fn i8_to_rank_char(rank:i8) -> char {
    match rank {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        9 => 'i',
        _ => {panic!("Unexpected rank: {0}", rank)},
    }
}

pub const DEFAULT_FILE_LEN: usize = 9;
pub const DEFAULT_RANK_LEN: usize = 9;
pub const SKY_LEN: usize = 1;
pub const SKY_ADDRESS: usize = 81;
pub const DEFAULT_BOARD_SIZE: usize = (DEFAULT_FILE_LEN * DEFAULT_RANK_LEN + SKY_LEN) as usize;
pub const HANDS_LEN: usize = 3 * 8;

#[derive(Clone, Copy, PartialEq)]
pub struct BoardSize {
    pub file_len: i8,
    pub rank_len: i8,
}
impl BoardSize {
    pub fn create_hon_shogi() -> BoardSize {
        BoardSize {
            file_len: DEFAULT_FILE_LEN as i8,
            rank_len: DEFAULT_RANK_LEN as i8,
        }
    }

    pub fn file_rank_to_cell(self, file:i8, rank:i8) -> usize {
        ((rank-1)*self.file_len + (file-1)) as usize
    }
    pub fn cell_to_file_rank(self, cell:usize) -> (i8, i8) {
        ((cell%self.file_len as usize) as i8 + 1, (cell/self.file_len as usize) as i8 + 1)
    }
    pub fn len(self) -> usize {
        (self.file_len * self.rank_len) as usize
    }
}

pub struct Position {
    phase: Phase,
    board_size: BoardSize,
    pub pieces: [Option<Piece>; DEFAULT_BOARD_SIZE],
    /// R, B, G, S, N, L, P, r, b, g, s, n, l, p.
    pub hands: [i8; HANDS_LEN],
}
impl Position {
    pub fn default() -> Position {
        Position {
            phase: Phase::First,
            board_size: BoardSize::create_hon_shogi(),
            pieces: [None; DEFAULT_BOARD_SIZE],
            hands: [0; HANDS_LEN],
        }
    }

    pub fn reset_default(&mut self) {
        self.board_size = BoardSize::create_hon_shogi();
        self.pieces = [None; DEFAULT_BOARD_SIZE];
        self.hands = [
                0, 0, 0, 0, 0, 0, 0, 0, // First phase.
                0, 0, 0, 0, 0, 0, 0, 0, // Second phase.
                2, 2, 2, 4, 4, 4, 4, 18,]; // None phase.
    }

    pub fn reset_startpos(&mut self) {
        use position::Piece::*;
        self.board_size = BoardSize::create_hon_shogi();
        // Flip horizontal.
        self.pieces = [
                Some(L2), Some(N2), Some(S2), Some(G2), Some(K2), Some(G2), Some(S2), Some(N2), Some(L2),
                None, Some(B2), None, None, None, None, None, Some(R2), None,
                Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2),
                None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None,
                Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1),
                None, Some(R1), None, None, None, None, None, Some(B1), None,
                Some(L1), Some(N1), Some(S1), Some(G1), Some(K1), Some(G1), Some(S1), Some(N1), Some(L1),
                None, // Sky
        ];
        self.hands = [0; HANDS_LEN];
    }

    pub fn get_phase(&self) -> Phase {
        self.phase
    }

    pub fn get_board_size(&self) -> BoardSize {
        self.board_size
    }

    pub fn get_piece(&self, file:i8, rank:i8) -> Option<Piece> {
        let address = self.board_size.file_rank_to_cell(file, rank);
        self.pieces[address]
    }

    pub fn get_piece_by_address(&self, address:usize) -> Option<Piece> {
        self.pieces[address]
    }

    /// Obsolute. new --> add().
    pub fn set_piece(&mut self, file:i8, rank:i8, piece:Option<Piece>) {
        let cell = self.board_size.file_rank_to_cell(file, rank);
        self.pieces[cell] = piece;
    }

    pub fn remove_piece(&mut self, file:i8, rank:i8) -> Option<Piece> {
        let cell = self.get_board_size().file_rank_to_cell(file, rank);
        let piece = self.pieces[cell];
        self.set_piece(file, rank, None);
        piece
    }

    pub fn get_hand(&self, piece:Piece) -> i8 {
        use position::Piece::*;
        match piece {
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
            _ => panic!("Unexpected hand '{}'.", piece_to_sign(Some(piece))),
        }
    }

    pub fn touch(&mut self, physical_move:&PhysicalMove) {
        match physical_move.address {
            // どこかを指定した。
            Some(address) => {
                match self.pieces[address.get_index()] {
                    Some(piece) => {
                        // 駒の場所を指定した。
                        match self.pieces[SKY_ADDRESS] {
                            Some(_piece) => {
                                // 指には何も持ってない。
                            },
                            None => {
                                // 指で駒をつかむ。
                                self.pieces[SKY_ADDRESS] = Some(piece);
                                self.pieces[address.get_index()] = None;
                            },
                        }
                    },
                    None => {
                        // 空き升を指定した。
                        match self.pieces[SKY_ADDRESS] {
                            Some(piece) => {
                                // 指につまんでいる駒を置く。
                                self.pieces[SKY_ADDRESS] = None;
                                self.pieces[address.get_index()] = Some(piece);
                            },
                            None => {
                            },
                        }
                    },
                }
            },
            None => {
                if physical_move.phase_change {
                    // TODO phase change.
                    use position::Phase::*;
                    self.phase = match self.phase {
                        First => {Second},
                        Second => {First},
                    };
                } else {
                    match self.pieces[SKY_ADDRESS] {
                        Some(piece) => {
                            if physical_move.sky_turn {
                                self.pieces[SKY_ADDRESS] = promotion_piece(Some(piece));
                            } else if physical_move.sky_rotate {
                                self.pieces[SKY_ADDRESS] = rotate_piece(Some(piece));
                            };
                        },
                        None => {
                        },
                    }
                }
            }
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
        let count = self.get_hand(piece);
        let num_label = if 1 < count {count.to_string()} else {"".to_string()};
        let ch = if 0 < count {
            piece_type_to_sign(Some(piece_type))
        } else {
            "".to_string()
        };

        format!("{}{}", num_label, ch)
    }

    /// Point of symmetory.
    pub fn to_text(&self, phase:Phase) -> String {
        use position::Phase::*;
        let mut content = String::new();

        Parser::appendln(&mut content, &format!("              {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>3}",
            self.print_hand(Some(Phase::First), PieceType::K),
            self.print_hand(Some(Phase::First), PieceType::R),
            self.print_hand(Some(Phase::First), PieceType::B),
            self.print_hand(Some(Phase::First), PieceType::G),
            self.print_hand(Some(Phase::First), PieceType::S),
            self.print_hand(Some(Phase::First), PieceType::N),
            self.print_hand(Some(Phase::First), PieceType::L),
            self.print_hand(Some(Phase::First), PieceType::P)));

        match phase {
            First => {
                // hand.
                Parser::appendln(&mut content, &format!("|         |  +-------------------+"));
            },
            Second => {
                Parser::appendln(&mut content, &format!("             +-------------------+"));
            },
        }

        for y in 0..=8 {
            let rank = 9 - y;

            // 先手の手。
            match phase {
                First => {
                    match rank {
                        9 => {Parser::append(&mut content, &format!("|         | "))},
                        8 => {Parser::append(&mut content, &format!("+---+ +-+ | "))},
                        7 => {Parser::append(&mut content, &format!("    | | | | "))},
                        6 => {Parser::append(&mut content, &format!("    | | | | ",))},
                        5 => {Parser::append(&mut content, &format!("    +-+ +-+ ",))},
                        4 => {Parser::append(&mut content, &format!("      {:>2}    ", piece_to_sign(self.get_piece_by_address(SKY_ADDRESS))))},
                        3 => {Parser::append(&mut content, &format!("            "))},
                        2 => {Parser::append(&mut content, &format!("            "))},
                        1 => {Parser::append(&mut content, &format!("            "))},
                        _ => {},
                    };
                },
                Second => {Parser::append(&mut content, &format!("            "))},
            }

            Parser::append(&mut content, &format!(
                "{0}|{1: >2}{2: >2}{3: >2}{4: >2}{5: >2}{6: >2}{7: >2}{8: >2}{9: >2}",
                i8_to_rank_char(rank),
                piece_to_sign(self.get_piece(1, rank)),
                piece_to_sign(self.get_piece(2, rank)),
                piece_to_sign(self.get_piece(3, rank)),
                piece_to_sign(self.get_piece(4, rank)),
                piece_to_sign(self.get_piece(5, rank)),
                piece_to_sign(self.get_piece(6, rank)),
                piece_to_sign(self.get_piece(7, rank)),
                piece_to_sign(self.get_piece(8, rank)),
                piece_to_sign(self.get_piece(9, rank))));

            // Right boarder and None phase hands.
            match rank {
                9 => {Parser::append(&mut content, &format!(" |   "))},
                8 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::K)))},
                7 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::R)))},
                6 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::B)))},
                5 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::G)))},
                4 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::S)))},
                3 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::N)))},
                2 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::L)))},
                1 => {Parser::append(&mut content, &format!(" |{:>3}", self.print_hand(None, PieceType::P)))},
                _ => {},
            };

            // Second player finger.
            match phase {
                First => {},
                Second => {
                    match rank {
                        9 => {},
                        8 => {},
                        6 => {Parser::append(&mut content, &format!("   {:>2}", piece_to_sign(self.get_piece_by_address(SKY_ADDRESS))))},
                        5 => {Parser::append(&mut content, &format!(" +-+ +-+"))},
                        4 => {Parser::append(&mut content, &format!(" | | | |"))},
                        3 => {Parser::append(&mut content, &format!(" | | | |"))},
                        2 => {Parser::append(&mut content, &format!(" | +-+ +---+"))},
                        1 => {Parser::append(&mut content, &format!(" |         |"))},
                        _ => {},
                    };                    
                },
            }

            Parser::append_ln(&mut content);
        }

        match phase {
            First => {
                Parser::appendln(&mut content, &format!("             +-------------------+"));
            },
            Second => {
                // hand.
                Parser::appendln(&mut content, &format!("             +-------------------+    |         |"));
            },
        }

        Parser::appendln(&mut content, &format!("               1 2 3 4 5 6 7 8 9"));

        // Second phase hand.
        Parser::appendln(&mut content, &format!("              {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>3}",
            self.print_hand(Some(Phase::Second), PieceType::K),
            self.print_hand(Some(Phase::Second), PieceType::R),
            self.print_hand(Some(Phase::Second), PieceType::B),
            self.print_hand(Some(Phase::Second), PieceType::G),
            self.print_hand(Some(Phase::Second), PieceType::S),
            self.print_hand(Some(Phase::Second), PieceType::N),
            self.print_hand(Some(Phase::Second), PieceType::L),
            self.print_hand(Some(Phase::Second), PieceType::P)
            ));

        content
    }
}