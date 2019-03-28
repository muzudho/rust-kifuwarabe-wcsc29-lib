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
    // 生.
    PK1,
    PK2,
    PK3,

    // Rook is 飛.
    R1,
    R2,
    R3,
    // Promoted rook is 竜.
    PR1,
    PR2,
    PR3,

    // Bishop is 角.
    B1,
    B2,
    B3,
    // Promoted bishop is 馬.
    PB1,
    PB2,
    PB3,

    // Gold is 金.
    G1,
    G2,
    G3,
    // 生.
    PG1,
    PG2,
    PG3,

    // Silver is 銀.
    S1,
    S2,
    S3,
    // Promoted silver is 成銀.
    PS1,
    PS2,
    PS3,

    // kNight is 桂.
    N1,
    N2,
    N3,
    // Promoted knight is 成桂.
    PN1,
    PN2,
    PN3,

    // Lance is 香.
    L1,
    L2,
    L3,
    // Promoted lance is 成香.
    PL1,
    PL2,
    PL3,

    // Pawn is 歩.
    P1,
    P2,
    P3,
    // Promoted pawn is と.
    PP1,
    PP2,
    PP3,
}

/// Perfect piece type.
#[derive(Clone, Copy, PartialEq)]
pub enum PieceType{
    // King is 玉.
    K = 0,
    // 生
    PK,

    // Rook is 飛.
    R,
    // Promoted rook is 竜.
    PR,

    // Bishop is 角.
    B,
    // Promoted bishop is 馬.
    PB,

    // Gold is 金.
    G,
    // 今
    PG,

    // Silver is 銀.
    S,
    // Promoted silver is 成銀.
    PS,

    // kNight is 桂.
    N,
    // Promoted knight is 成桂.
    PN,

    // Lance is 香.
    L,
    // Promoted lance is 成香.
    PL,

    // Pawn is 歩.
    P,
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
            PK1 => "+K",
            PK2 => "+k",
            PK3 => "+K",

            R1 => "R",
            R2 => "r",
            R3 => "R",
            PR1 => "+R",
            PR2 => "+r",
            PR3 => "+R",

            B1 => "B",
            B2 => "b",
            B3 => "B",
            PB1 => "+B",
            PB2 => "+b",
            PB3 => "+B",

            G1 => "G",
            G2 => "g",
            G3 => "G",
            PG1 => "+G",
            PG2 => "+g",
            PG3 => "+G",

            S1 => "S",
            S2 => "s",
            S3 => "S",
            PS1 => "+S",
            PS2 => "+s",
            PS3 => "+S",

            N1 => "N",
            N2 => "n",
            N3 => "N",
            PN1 => "+N",
            PN2 => "+n",
            PN3 => "+N",

            L1 => "L",
            L2 => "l",
            L3 => "L",
            PL1 => "+L",
            PL2 => "+l",
            PL3 => "+L",

            P1 => "P",
            P2 => "p",
            P3 => "P",
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
            // 逆さにできないから、半角カナにしているだけ☆（＾～＾）右側のスペースに18進数の背番号が入る予定☆（＾～＾）
            K1 => " ｵｳ ",
            K2 => " 玉 ",
            K3 => " 玉 ",
            PK1 => " ｲｸ ",
            PK2 => " 生 ",
            PK3 => " 生 ",

            R1 => " ヒ ",
            R2 => " 飛 ",
            R3 => " 飛 ",
            PR1 => " ﾘｭ ",
            PR2 => " 竜 ",
            PR3 => " 竜 ",

            B1 => " ｶｸ ",
            B2 => " 角 ",
            B3 => " 角 ",
            PB1 => " ｳﾏ ",
            PB2 => " 馬 ",
            PB3 => " 馬 ",

            G1 => " ｷﾝ ",
            G2 => " 金 ",
            G3 => " 金 ",
            PG1 => " Nｷ ",
            PG2 => " 今 ",
            PG3 => " 今 ",

            S1 => " ｷﾞ ",
            S2 => " 銀 ",
            S3 => " 銀 ",
            PS1 => " NG ",
            PS2 => " 全 ",
            PS3 => " 全 ",

            N1 => " ｹｲ ",
            N2 => " 桂 ",
            N3 => " 桂 ",
            PN1 => " Nｹ ",
            PN2 => " 圭 ",
            PN3 => " 圭 ",

            L1 => " ｷｮ ",
            L2 => " 香 ",
            L3 => " 香 ",
            PL1 => " Nﾔ ",
            PL2 => " 杏 ",
            PL3 => " 杏 ",

            P1 => " フ ",
            P2 => " 歩 ",
            P3 => " 歩 ",
            PP1 => " ト ",
            PP2 => " と ",
            PP3 => " と ",
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
        K2 => K,
        K3 => K,
        PK1 => PK,
        PK2 => PK,
        PK3 => PK,

        R1 => R,
        R2 => R,
        R3 => R,
        PR1 => PR,
        PR2 => PR,
        PR3 => PR,

        B1 => B,
        B2 => B,
        B3 => B,
        PB1 => PB,
        PB2 => PB,
        PB3 => PB,

        G1 => G,
        G2 => G,
        G3 => G,
        PG1 => PG,
        PG2 => PG,
        PG3 => PG,

        S1 => S,
        S2 => S,
        S3 => S,
        PS1 => PS,
        PS2 => PS,
        PS3 => PS,

        N1 => N,
        N2 => N,
        N3 => N,
        PN1 => PN,
        PN2 => PN,
        PN3 => PN,

        L1 => L,
        L2 => L,
        L3 => L,
        PL1 => PL,
        PL2 => PL,
        PL3 => PL,

        P1 => P,
        P2 => P,
        P3 => P,
        PP1 => PP,
        PP2 => PP,
        PP3 => PP,
    }
}
pub fn hand_piece_to_hand_index(piece:Piece) -> i8 {
    use piece_etc::Piece::*;
    match piece {
        K1 | PK1 => {0},
        K2 | PK2 => {8},
        K3 | PK3 => {16},

        R1 | PR1 => {1},
        R2 | PR2 => {9},
        R3 | PR3 => {17},
        
        B1 | PB1 => {2},
        B2 | PB2 => {10},
        B3 | PB3 => {18},

        G1 | PG1 => {3},
        G2 | PG2 => {11},
        G3 | PG3 => {19},

        S1 | PS1 => {4},
        S2 | PS2 => {12},
        S3 | PS3 => {20},

        N1 | PN1 => {5},
        N2 | PN2 => {13},
        N3 | PN3 => {21},

        L1 | PL1 => {6},
        L2 | PL2 => {14},
        L3 | PL3 => {22},

        P1 | PP1 => {7},
        P2 | PP2 => {15},
        P3 | PP3 => {23},
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
                K1 | PK1 | R1 | PR1 | B1 | PB1 | G1 | PG1 | S1 | PS1 | N1 | PN1 | L1 | PL1 | P1 | PP1 => Some(Phase::First),
                K2 | PK2 | R2 | PR2 | B2 | PB2 | G2 | PG2 | S2 | PS2 | N2 | PN2 | L2 | PL2 | P2 | PP2 => Some(Phase::Second),
                K3 | PK3 | R3 | PR3 | B3 | PB3 | G3 | PG3 | S3 | PS3 | N3 | PN3 | L3 | PL3 | P3 | PP3 => None,
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
            K1 => Some(PK1),
            K2 => Some(PK2),
            K3 => Some(PK3),
            PK1 => Some(K1),
            PK2 => Some(K2),
            PK3 => Some(K3),

            R1 => Some(PR1),
            R2 => Some(PR2),
            R3 => Some(PR3),
            PR1 => Some(R1),
            PR2 => Some(R2),
            PR3 => Some(R3),

            B1 => Some(PB1),
            B2 => Some(PB2),
            B3 => Some(PB3),
            PB1 => Some(B1),
            PB2 => Some(B2),
            PB3 => Some(B3),

            G1 => Some(PG1),
            G2 => Some(PG2),
            G3 => Some(PG3),
            PG1 => Some(G1),
            PG2 => Some(G2),
            PG3 => Some(G3),

            S1 => Some(PS1),
            S2 => Some(PS2),
            S3 => Some(PS3),
            PS1 => Some(S1),
            PS2 => Some(S2),
            PS3 => Some(S3),

            N1 => Some(PN1),
            N2 => Some(PN2),
            N3 => Some(PN3),
            PN1 => Some(N1),
            PN2 => Some(N2),
            PN3 => Some(N3),

            L1 => Some(PL1),
            L2 => Some(PL2),
            L3 => Some(PL3),
            PL1 => Some(L1),
            PL2 => Some(L2),
            PL3 => Some(L3),

            P1 => Some(PP1),
            P2 => Some(PP2),
            P3 => Some(PP3),
            PP1 => Some(P1),
            PP2 => Some(P2),
            PP3 => Some(P3),
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
            PK1 => Some(PK2),
            PK2 => Some(PK1),
            PK3 => Some(PK3),

            R1 => Some(R2),
            R2 => Some(R1),
            R3 => Some(R3),
            PR1 => Some(PR2),
            PR2 => Some(PR1),
            PR3 => Some(PR3),

            B1 => Some(B2),
            B2 => Some(B1),
            B3 => Some(B3),
            PB1 => Some(PB2),
            PB2 => Some(PB1),
            PB3 => Some(PB3),

            G1 => Some(PG2),
            G2 => Some(PG1),
            G3 => Some(PG3),
            PG1 => Some(G2),
            PG2 => Some(G1),
            PG3 => Some(G3),

            S1 => Some(S2),
            S2 => Some(S1),
            S3 => Some(S3),
            PS1 => Some(PS2),
            PS2 => Some(PS1),
            PS3 => Some(PS3),

            N1 => Some(N2),
            N2 => Some(N1),
            N3 => Some(N3),
            PN1 => Some(PN2),
            PN2 => Some(PN1),
            PN3 => Some(PN3),

            L1 => Some(L2),
            L2 => Some(L1),
            L3 => Some(L3),
            PL1 => Some(PL2),
            PL2 => Some(PL1),
            PL3 => Some(PL3),

            P1 => Some(P2),
            P2 => Some(P1),
            P3 => Some(P3),
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
            PK1 | PR1 | PB1 | PG1 | PS1 | PN1 | PL1 | PP1 |
            PK2 | PR2 | PB2 | PG2 | PS2 | PN2 | PL2 | PP2 |
            PK3 | PR3 | PB3 | PG3 | PS3 | PN3 | PL3 | PP3 => true,
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
            PK | PR | PB | PG | PS | PN | PL | PP => true,
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
                PK => "K".to_string(),

                R => "R".to_string(),
                PR => "+R".to_string(),

                B => "B".to_string(),
                PB => "+B".to_string(),

                G => "G".to_string(),
                PG => "G".to_string(),

                S => "S".to_string(),
                PS => "+S".to_string(),

                N => "N".to_string(),
                PN => "+N".to_string(),

                L => "L".to_string(),
                PL => "+L".to_string(),

                P => "P".to_string(),
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
                        PK => PK1,

                        R => R1,
                        PR => PR1,

                        B => B1,
                        PB => PB1,

                        G => G1,
                        PG => PG1,

                        S => S1,
                        PS => PS1,

                        N => N1,
                        PN => PN1,

                        L => L1,
                        PL => PL1,

                        P => P1,
                        PP => PP1,
                    }
                },
                Second => {
                    match piece_type {
                        K => K2,
                        PK => PK2,

                        R => R2,
                        PR => PR2,

                        B => B2,
                        PB => PB2,

                        G => G2,
                        PG => PG2,

                        S => S2,
                        PS => PS2,

                        N => N2,
                        PN => PN2,

                        L => L2,
                        PL => PL2,

                        P => P2,
                        PP => PP2,
                    }
                },
            }
        },
        None => {
            match piece_type {
                K => K3,
                PK => PK3,

                R => R3,
                PR => PR3,

                B => B3,
                PB => PB3,

                G => G3,
                PG => PG3,

                S => S3,
                PS => PS3,

                N => N3,
                PN => PN3,

                L => L3,
                PL => PL3,
                
                P => P3,
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
