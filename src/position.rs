use board::*;
use physical_record::*;
use std::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    /// Starting first.
    First,
    /// Starting second.
    Second,
}
pub fn phase_to_sign(phase:Phase) -> String {
    use position::Phase::*;
    match phase {
        First => "b".to_string(),
        Second => "w".to_string(),
        _ => panic!("Unexpected phase. *phase as usize = {}.", phase as usize),
    }
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

pub struct Position {
    pub board : Board,
}
impl Position {
    pub fn default() -> Position {
        Position {
            board : Board::default(),
        }
    }
    pub fn startpos() -> Position {
        Position {
            board : Board::startpos(),
        }
    }

    pub fn remove_piece(&mut self, file:i8, rank:i8) -> Option<Piece> {
        let cell = self.board.get_board_size().file_rank_to_cell(file, rank);
        let piece = self.board.pieces[cell];
        self.board.set_piece(file, rank, None);
        piece
    }
}