pub fn sign_to_piece(phase_opt:Option<Phase>, sign:String) -> Piece {
    use position::Phase::*;
    use position::Piece::*;
    let s = sign.as_str();
    match phase_opt {
        Some(phase) => {
            match phase {
                First => {
                    match s {
                        "K" => K1,
                        "R" => R1,
                        "B" => B1,
                        "G" => G1,
                        "S" => S1,
                        "N" => N1,
                        "L" => L1,
                        "P" => P1,
                        "PR" => PR1,
                        "PB" => PB1,
                        "PS" => PS1,
                        "PN" => PN1,
                        "PL" => PL1,
                        "PP" => PP1,
                        _ => panic!("Unexpected sign of First: '{}'.", sign)
                    }
                },
                Second => {
                    match s{
                        "K" => K2,
                        "R" => R2,
                        "B" => B2,
                        "G" => G2,
                        "S" => S2,
                        "N" => N2,
                        "L" => L2,
                        "P" => P2,
                        "PR" => PR2,
                        "PB" => PB2,
                        "PS" => PS2,
                        "PN" => PN2,
                        "PL" => PL2,
                        "PP" => PP2,
                        _ => panic!("Unexpected sign of Second: '{}'.", sign)
                    }
                },
            }
        },
        None => {
            match s {
                "K" => K3,
                "R" => R3,
                "B" => B3,
                "G" => G3,
                "S" => S3,
                "N" => N3,
                "L" => L3,
                "P" => P3,
                "PR" => PR3,
                "PB" => PB3,
                "PS" => PS3,
                "PN" => PN3,
                "PL" => PL3,
                "PP" => PP3,
                _ => panic!("Unexpected sign on None: '{}'.", sign)
            }
        }
    }
}

impl BoardSize {
    pub fn reverse_cell(self, cell:usize) -> usize {
        self.rank_len as usize * self.file_len as usize - cell
    }
}
impl Position {
    /// latest.
    pub fn add(&mut self, address:Address, piece:Piece) {
        if address.is_on_board(self.board_size) {
            match self.board[address.get_index()] {
                Some(piece2) => panic!("Piece already exists '{}'.", piece_to_sign(Some(piece2))),
                None => {
                    self.board[address.get_index()] = Some(piece);
                },
            }
        } else if address.is_hand() {
            self.hands[address.get_index() - self.board_size.len()] += 1
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

    /// 持ち駒の１行表示
    fn to_hand_text(&self, _comm:&Communication, phase_opt:Option<Phase>, piece_type:PieceType) -> String {
        
        let piece = piece_type_to_piece(phase_opt, piece_type);
        let count = self.get_hand(piece);
        let coefficient = if 1 < count {count.to_string()} else {"".to_string()};
        // comm.println(&format!("piece_type: '{}', hand_count: {}, coefficient: {}.", piece_type_to_sign(Some(piece_type)), count, coefficient));
        let ch = if 0 < count {
            piece_type_to_sign(Some(piece_type))
        } else {
            "".to_string()
        };

        format!("{}{}", coefficient, ch)
    }
}
