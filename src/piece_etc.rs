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
    K2,
    K3,
    // Rook is 飛.
    R1,
    R2,
    R3,
    // Bishop is 角.
    B1,
    B2,
    B3,
    // Gold is 金.
    G1,
    G2,
    G3,
    // Silver is 銀.
    S1,
    S2,
    S3,
    // kNight is 桂.
    N1,
    N2,
    N3,
    // Lance is 香.
    L1,
    L2,
    L3,
    // Pawn is 歩.
    P1,
    P2,
    P3,
    // Promoted rook is 竜.
    PR1,
    PR2,
    PR3,
    // Promoted bishop is 馬.
    PB1,
    PB2,
    PB3,
    // Promoted silver is 成銀.
    PS1,
    PS2,
    PS3,
    // Promoted knight is 成桂.
    PN1,
    PN2,
    PN3,
    // Promoted lance is 成香.
    PL1,
    PL2,
    PL3,
    // Promoted pawn is と.
    PP1,
    PP2,
    PP3,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType{
    // King is 玉.
    K = 0,
    // Rook is 飛.
    R,
    // Bishop is 角.
    B,
    // Gold is 金.
    G,
    // Silver is 銀.
    S,
    // kNight is 桂.
    N,
    // Lance is 香.
    L,
    // Pawn is 歩.
    P,
    // Promoted rook is 竜.
    PR,
    // Promoted bishop is 馬.
    PB,
    // Promoted silver is 成銀.
    PS,
    // Promoted knight is 成桂.
    PN,
    // Promoted lance is 成香.
    PL,
    // Promoted pawn is と.
    PP,
}
pub fn phase_to_sign(phase:Phase) -> String {
    use piece_etc::Phase::*;
    match phase {
        First => "b".to_string(),
        Second => "w".to_string(),
        _ => panic!("Unexpected phase. *phase as usize = {}.", phase as usize),
    }
}

pub fn piece_to_sign(piece_opt:Option<Piece>) -> String {
    if let Some(piece) = piece_opt {
        use piece_etc::Piece::*;
        match piece {
            K1 => "K",
            K2 => "k",
            K3 => "K",

            R1 => "R",
            R2 => "r",
            R3 => "R",

            B1 => "B",
            B2 => "b",
            B3 => "B",

            G1 => "G",
            G2 => "g",
            G3 => "G",

            S1 => "S",
            S2 => "s",
            S3 => "S",

            N1 => "N",
            N2 => "n",
            N3 => "N",

            L1 => "L",
            L2 => "l",
            L3 => "L",

            P1 => "P",
            P2 => "p",
            P3 => "P",

            PR1 => "+R",
            PR2 => "+r",
            PR3 => "+R",

            PB1 => "+B",
            PB2 => "+b",
            PB3 => "+B",

            PS1 => "+S",
            PS2 => "+s",
            PS3 => "+S",

            PN1 => "+N",
            PN2 => "+n",
            PN3 => "+N",

            PL1 => "+L",
            PL2 => "+l",
            PL3 => "+L",

            PP1 => "+P",
            PP2 => "+p",
            PP3 => "+P",
        }
    } else {
        ""
    }.to_string()
}
/// 横幅は半角4文字。
pub fn piece_to_display(piece_opt:Option<Piece>) -> String {
    if let Some(piece) = piece_opt {
        use piece_etc::Piece::*;
        match piece {
            // 逆さにできないから、アルファベットにしているだけ☆（＾～＾）
            K1 => "OU  ",
            K2 => "玉  ",
            K3 => "玉  ",

            R1 => "HI  ",
            R2 => "飛  ",
            R3 => "飛  ",

            B1 => "KA  ",
            B2 => "角  ",
            B3 => "角  ",

            G1 => "KI  ",
            G2 => "金  ",
            G3 => "金  ",

            S1 => "GI  ",
            S2 => "銀  ",
            S3 => "銀  ",

            N1 => "KE  ",
            N2 => "桂  ",
            N3 => "桂  ",

            L1 => "KY  ",
            L2 => "香  ",
            L3 => "香  ",

            P1 => "FU  ",
            P2 => "歩  ",
            P3 => "歩  ",

            PR1 => "RY  ",
            PR2 => "竜  ",
            PR3 => "竜  ",

            PB1 => "UM  ",
            PB2 => "馬  ",
            PB3 => "馬  ",

            PS1 => "NG  ",
            PS2 => "全  ",
            PS3 => "全  ",

            PN1 => "NK  ",
            PN2 => "圭  ",
            PN3 => "圭  ",

            PL1 => "NY  ",
            PL2 => "杏  ",
            PL3 => "杏  ",

            PP1 => "TO  ",
            PP2 => "と  ",
            PP3 => "と  ",
        }
    } else {
        "    "
    }.to_string()
}
pub fn piece_to_piece_type(piece:Piece) -> PieceType {
    use piece_etc::Piece::*;
    use piece_etc::PieceType::*;
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
pub fn hand_piece_to_hand_index(piece:Piece) -> i8 {
    use piece_etc::Piece::*;
    match piece {
        K1 => {0},
        R1 => {1},
        B1 => {2},
        G1 => {3},
        S1 => {4},
        N1 => {5},
        L1 => {6},
        P1 => {7},
        K2 => {8},
        R2 => {9},
        B2 => {10},
        G2 => {11},
        S2 => {12},
        N2 => {13},
        L2 => {14},
        P2 => {15},
        K3 => {16},
        R3 => {17},
        B3 => {18},
        G3 => {19},
        S3 => {20},
        N3 => {21},
        L3 => {22},
        P3 => {23},
        _ => panic!("Unexpected hand '{}'.", piece_to_sign(Some(piece))),
    }
}
pub fn sign_to_piece_type(sign:&str) -> PieceType {
    use piece_etc::PieceType::*;
    match sign {
        "K" | "k" => K,
        "R" | "r" => R,
        "B" | "b" => B,
        "G" | "g" => G,
        "S" | "s" => S,
        "N" | "n" => N,
        "L" | "l" => L,
        "P" | "p" => P,
        "PR" | "pr" => PR,
        "PB" | "pb" => PB,
        "PS" | "ps" => PS,
        "PN" | "pn" => PN,
        "PL" | "pl" => PL,
        "PP" | "pp" => PP,
        _ => panic!("Unexpected sign: '{}'.", sign)
    }
}
pub fn piece_to_phase(piece:Option<Piece>) -> Option<Phase> {
    match piece {
        Some(x) => {
            use piece_etc::Piece::*;
            match x {
                K1 | R1 | B1 | G1 | S1 | N1 | L1 | P1 | PR1 | PB1 | PS1 | PN1 | PL1 | PP1 => Some(Phase::First),
                K2 | R2 | B2 | G2 | S2 | N2 | L2 | P2 | PR2 | PB2 | PS2 | PN2 | PL2 | PP2 => Some(Phase::Second),
                _ => panic!("Unexpected phase. *piece as usize = {}.", x as usize),
            }
        },
        None => None,
    }
}

pub fn promotion_piece(piece_opt:Option<Piece>) -> Option<Piece> {
    if let Some(piece) = piece_opt {
        use piece_etc::Piece::*;
        match piece {
            K1 => Some(K1),
            K2 => Some(K2),
            K3 => Some(K3),

            R1 => Some(PR1),
            R2 => Some(PR2),
            R3 => Some(PR3),

            B1 => Some(PB1),
            B2 => Some(PB2),
            B3 => Some(PB3),

            G1 => Some(G1),
            G2 => Some(G2),
            G3 => Some(G3),

            S1 => Some(PS1),
            S2 => Some(PS2),
            S3 => Some(PS3),

            N1 => Some(PN1),
            N2 => Some(PN2),
            N3 => Some(PN3),

            L1 => Some(PL1),
            L2 => Some(PL2),
            L3 => Some(PL3),

            P1 => Some(PP1),
            P2 => Some(PP2),
            P3 => Some(PP3),

            PR1 => Some(PR1),
            PR2 => Some(PR2),
            PR3 => Some(PR3),

            PB1 => Some(PB1),
            PB2 => Some(PB2),
            PB3 => Some(PB3),

            PS1 => Some(PS1),
            PS2 => Some(PS2),
            PS3 => Some(PS3),

            PN1 => Some(PN1),
            PN2 => Some(PN2),
            PN3 => Some(PN3),

            PL1 => Some(PL1),
            PL2 => Some(PL2),
            PL3 => Some(PL3),

            PP1 => Some(PP1),
            PP2 => Some(PP2),
            PP3 => Some(PP3),
            _ => panic!("Failed: Sfen unexpected promotion.")
        }
    } else {
        None
    }
}
pub fn rotate_piece(piece_opt:Option<Piece>) -> Option<Piece> {
    if let Some(piece) = piece_opt {
        use piece_etc::Piece::*;
        match piece {
            K1 => Some(K2),
            K2 => Some(K1),
            K3 => Some(K3),

            R1 => Some(R2),
            R2 => Some(R1),
            R3 => Some(R3),

            B1 => Some(B2),
            B2 => Some(B1),
            B3 => Some(B3),

            G1 => Some(G2),
            G2 => Some(G1),
            G3 => Some(G3),

            S1 => Some(S2),
            S2 => Some(S1),
            S3 => Some(S3),

            N1 => Some(N2),
            N2 => Some(N1),
            N3 => Some(N3),

            L1 => Some(L2),
            L2 => Some(L1),
            L3 => Some(L3),

            P1 => Some(P2),
            P2 => Some(P1),
            P3 => Some(P3),

            PR1 => Some(PR2),
            PR2 => Some(PR1),
            PR3 => Some(PR3),

            PB1 => Some(PB2),
            PB2 => Some(PB1),
            PB3 => Some(PB3),

            PS1 => Some(PS2),
            PS2 => Some(PS1),
            PS3 => Some(PS3),

            PN1 => Some(PN2),
            PN2 => Some(PN1),
            PN3 => Some(PN3),

            PL1 => Some(PL2),
            PL2 => Some(PL1),
            PL3 => Some(PL3),

            PP1 => Some(PP2),
            PP2 => Some(PP1),
            PP3 => Some(PP3),
        }
    } else {
        None
    }
}
pub fn is_promoted_piece(piece_opt:Option<Piece>) -> bool {
    if let Some(piece) = piece_opt {
        use piece_etc::Piece::*;
        match piece {
            PR1 | PB1 | PS1 | PN1 | PL1 | PP1 |
            PR2 | PB2 | PS2 | PN2 | PL2 | PP2 |
            PR3 | PB3 | PS3 | PN3 | PL3 | PP3 => true,
            _ => false,
        }
    } else {
        false
    }
}
pub fn is_promoted_piece_type(piece_type_opt:Option<PieceType>) -> bool {
    if let Some(piece_type) = piece_type_opt {
        use piece_etc::PieceType::*;
        match piece_type {
            PR | PB | PS | PN | PL | PP => true,
            _ => false,
        }
    } else {
        false
    }
}
pub fn piece_type_to_sign(piece_type_opt:Option<PieceType>) -> String {
    use piece_etc::PieceType::*;
    match piece_type_opt {
        Some(piece_type) => {
            match piece_type {
                K => "K".to_string(),
                R => "R".to_string(),
                B => "B".to_string(),
                G => "G".to_string(),
                S => "S".to_string(),
                N => "N".to_string(),
                L => "L".to_string(),
                P => "P".to_string(),
                PR => "+R".to_string(),
                PB => "+B".to_string(),
                PS => "+S".to_string(),
                PN => "+N".to_string(),
                PL => "+L".to_string(),
                PP => "+P".to_string(),
            }
        },
        None => {"".to_string()},
    }
}
pub fn piece_type_to_piece(phase_opt:Option<Phase>, piece_type:PieceType) -> Piece {
    use piece_etc::Phase::*;
    use piece_etc::Piece::*;
    use piece_etc::PieceType::*;
    match phase_opt {
        Some(phase) => {
            match phase {
                First => {
                    match piece_type {
                        K => K1,
                        R => R1,
                        B => B1,
                        G => G1,
                        S => S1,
                        N => N1,
                        L => L1,
                        P => P1,
                        PR => PR1,
                        PB => PB1,
                        PS => PS1,
                        PN => PN1,
                        PL => PL1,
                        PP => PP1,
                    }
                },
                Second => {
                    match piece_type {
                        K => K2,
                        R => R2,
                        B => B2,
                        G => G2,
                        S => S2,
                        N => N2,
                        L => L2,
                        P => P2,
                        PR => PR2,
                        PB => PB2,
                        PS => PS2,
                        PN => PN2,
                        PL => PL2,
                        PP => PP2,
                    }
                },
            }
        },
        None => {
            match piece_type {
                K => K3,
                R => R3,
                B => B3,
                G => G3,
                S => S3,
                N => N3,
                L => L3,
                P => P3,
                PR => PR3,
                PB => PB3,
                PS => PS3,
                PN => PN3,
                PL => PL3,
                PP => PP3,
            }
        }
    }
}

pub fn parse_sign_to_drop(line:&str, start:&mut usize) -> Option<PieceType> {
    use piece_etc::PieceType::*;

    if line.len() < *start + 2 {
        return None;
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    let piece_type = match sign {
        'R' => {R},
        'B' => {B},
        'G' => {G},
        'S' => {S},
        'N' => {N},
        'L' => {L},
        'P' => {P},
        _ => {return None;},
    };

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    if sign == '*' {
        *start += 2;
        Some(piece_type)
    } else {
        panic!("Failed: Sfen unexpected drop.");
    }
}

pub fn parse_sign_to_rank(line:&str, start:&mut usize) -> i8 {
    if line.len() < *start + 1 {
        panic!("Failed: Unexpected file. len: {}, start: {}.", line.len(), start);
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    *start += 1;
    match sign {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        _ => panic!("Failed: Unexpected rank. line `{}` at {}, `{}`.", line, *start-1, sign)
    }
}

pub fn rank_to_sign(sign:i8) -> char {
    match sign {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        9 => 'i',
        _ => panic!("Failed: Unexpected rank number `{}`.", sign)
    }
}

pub fn parse_sign_to_file(line:&str, start:&mut usize) -> i8 {
    if line.len() < *start as usize + 1 {
        panic!("Failed: Nothing file.");
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    *start += 1;
    match sign {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Failed: Unexpected file. line `{}` at {}, `{}`.", line, *start-1, sign)
    }
}

pub fn parse_sign_to_promotion(line:&str, start:&mut usize) -> bool {
    if line.len() < *start as usize + 1 {
        return false;
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    if sign == '+' {
        *start += 1;
        true
    } else {
        false
    }
}
