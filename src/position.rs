/// First turn player is 0.
/// Second turn player is 1.
#[derive(Clone, Copy, PartialEq)]
pub enum Piece{
    // Raion is king.
    R0 = 0,
    // Kirin is rook.
    K0,
    // Zou is bishop.
    Z0,
    // Inu is gold.
    I0,
    // Neko is silver.
    N0,
    // Usagi is knight.
    U0,
    // Inosisi is lance.
    S0,
    // Hiyoko is pawn.
    H0,
    // Power-up kirin is promoted rook.
    PK0,
    // Power-up zou is promoted bishop.
    PZ0,
    // Power-up neko is promoted silver.
    PN0,
    // Power-up usagi is promoted knight.
    PU0,
    // Power-up inosisi is promoted lance.
    PS0,
    // Power-up hiyoko is promoted pawn.
    PH0,
    R1,
    K1,
    Z1,
    I1,
    N1,
    U1,
    S1,
    H1,
    PK1,
    PZ1,
    PN1,
    PU1,
    PS1,
    PH1,
    // Empty square.
    Empty,
    // Num is size or error.
    Num
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

    pub fn parse(mut self, line:&str) {
        use position::Piece::*;
        let mut index = 0;

        if line.starts_with("position startpos") {
            if line.len() == 17 {
                return;
            } else {

            }
        } else if line.starts_with("position sfen ") {
            index = 14;
            let mut rank=9;
            let mut file=1;

            if &line[index..index+1] == "L" {
                self.board[(rank-8)*11 + file] = S1
            }

            loop {
                if &line[index..index+1] == " " {
                    break;
                }
            }
        }
    }
}