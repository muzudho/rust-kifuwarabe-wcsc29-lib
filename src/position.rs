/// First turn player is 0.
/// Second turn player is 1.
#[derive(Clone, Copy, PartialEq)]
pub enum Piece{
    // King is 玉.
    K0 = 0,
    // Rook is 飛.
    R0,
    // Bishop is 角.
    B0,
    // Gold is 金.
    G0,
    // Silver is 銀.
    S0,
    // kNight is 桂.
    N0,
    // Lance is 香.
    L0,
    // Pawn is 歩.
    P0,
    // Promoted rook is 竜.
    PR0,
    // Promoted bishop is 馬.
    PB0,
    // Promoted silver is 成銀.
    PS0,
    // Promoted knight is 成桂.
    PN0,
    // Promoted lance is 成香.
    PL0,
    // Promoted pawn is と.
    PP0,
    K1,
    R1,
    B1,
    G1,
    S1,
    N1,
    L1,
    P1,
    PR1,
    PB1,
    PS1,
    PN1,
    PL1,
    PP1,
    // Empty square.
    Empty,
    // Num is size or error.
    Num
}

pub fn sign_to_piece(character:&str) -> Piece {
    use position::Piece::*;
    match character {
        "K" => K0,
        "R" => R0,
        "B" => B0,
        "G" => G0,
        "S" => S0,
        "N" => N0,
        "L" => L0,
        "P" => P0,
        "+R" => PR0,
        "+B" => PB0,
        "+S" => PS0,
        "+N" => PN0,
        "+L" => PL0,
        "+P" => PP0,
        "k" => K1,
        "r" => R1,
        "b" => B1,
        "g" => G1,
        "s" => S1,
        "n" => N1,
        "l" => L1,
        "p" => P1,
        "+r" => PR1,
        "+b" => PB1,
        "+s" => PS1,
        "+n" => PN1,
        "+l" => PL1,
        "+p" => PP1,
        _ => Num,
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    // With frame. 11x11.
    pub board : [Piece;121],
}
impl Position {
    pub fn new() -> Position {
        use position::Piece::*;
        Position {
            board : [
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            ],
        }
    }

    pub fn get_piece(self, file:i8, rank:i8) -> Piece {
        self.board[((rank-8)*11 + file) as usize]
    }

    pub fn set_piece(mut self, file:i8, rank:i8, piece:Piece) {
        self.board[((rank-8)*11 + file) as usize] = piece;
    }

    pub fn parse(mut self, line:&str) {
        use position::Piece::*;
        let mut index = 0;

        if line.starts_with("position startpos") {
            self.board  = [
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, L1, N1, S1, G1, K1, G1, S1, N1, L1, Empty,
                Empty, Empty, R1, Empty, Empty, Empty, Empty, Empty, B1, Empty, Empty, 
                Empty, P1, P1, P1, P1, P1, P1, P1, P1, P1, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
                Empty, P0, P0, P0, P0, P0, P0, P0, P0, P0, Empty, 
                Empty, Empty, B0, Empty, Empty, Empty, Empty, Empty, R0, Empty, Empty, 
                Empty, L0, N0, S0, G0, K0, G0, S0, N0, L0, Empty, 
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            ];
            
            if line.len() == 17 {
                return;
            }

            // `position startpos moves `
            index = 25;

            // Examples.
            // position startpos moves 2g2f 8c8d

        } else if line.starts_with("position sfen ") {
            // TODO
            index = 14;
            let mut rank=9;
            let mut file=1;

            let pieceCharacter = &line[index..index+1];
            let mut spaces = match pieceCharacter {
                "1" => {1},
                "2" => {2},
                "3" => {3},
                "4" => {4},
                "5" => {5},
                "6" => {6},
                "7" => {7},
                "8" => {8},
                "9" => {9},
                "/" => {-1},
                _ => {0},
            };

            if spaces == 0 {
                self.set_piece(rank, file, sign_to_piece(pieceCharacter));
                file += 1;
            } else if spaces == -1 {
                file = 1;
                rank = 9;
            } else {
                while spaces > 0 {
                    self.set_piece(rank, file, Empty);
                    file += 1;
                    spaces -= 1;
                }
            }

            loop {
                if &line[index..index+1] == " " {
                    break;
                }
            }
        }
    }
}