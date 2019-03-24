pub fn phase_to_sign(phase:Phase) -> String {
    use board::Phase::*;
    match phase {
        First => "b".to_string(),
        Second => "w".to_string(),
        _ => panic!("Unexpected phase. *phase as usize = {}.", phase as usize),
    }
}

impl BoardSize {
    pub fn reverse_cell(self, cell:usize) -> usize {
        self.rank_len as usize * self.file_len as usize - cell
    }
}
impl Board {
    /// latest.
    pub fn add(&mut self, address:Address, piece:Piece) {
        if address.is_on_board(self.board_size) {
            match self.pieces[address.get_index()] {
                Some(piece2) => panic!("Piece already exists '{}'.", piece_to_sign(Some(piece2))),
                None => {
                    self.pieces[address.get_index()] = Some(piece);
                },
            }
        } else if address.is_hand() {
            self.hands[address.get_index() - self.board_size.len()] += 1
        }
    }

    /// Obsolute. new --> add().
    pub fn set_hand(&mut self, piece:Piece, num:i8) {
        use board::Piece::*;
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
}
