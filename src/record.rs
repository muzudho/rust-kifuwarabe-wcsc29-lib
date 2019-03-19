use std::*;
use position::*;

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
pub fn piece_type_to_sign(piece_type_opt:&Option<PieceType>) -> String {
    use record::PieceType::*;
    match *piece_type_opt {
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
                Empty => "".to_string(),
                Num => "?".to_string(),
            }
        },
        None => {"".to_string()},
    }
}
pub fn piece_type_to_piece(phase:&Phase, piece_type:&PieceType) -> Piece {
    use position::Phase::*;
    use position::Piece::*;
    use record::PieceType::*;
    match *phase {
        First => {
            match *piece_type {
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
                _ => K1, // TODO Error
            }
        },
        Second => {
            match *piece_type {
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
                _ => K2, // TODO Error
            }
        },
    }
}

pub fn parse_sign_to_drop(line:&str, start:&mut i8) -> Option<PieceType> {
    use record::PieceType::*;

    if line.len() < *start as usize + 2 {
        return None;
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start as usize];
    let pieceType = match sign {
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
    let sign = v[*start as usize];
    if sign == '*' {
        *start += 2;
        Some(pieceType)
    } else {
        panic!("Failed: Sfen unexpected drop.");
    }
}

pub fn parse_sign_to_rank(line:&str, start:&mut i8) -> i8 {
    if line.len() < *start as usize + 1 {
        panic!("Failed: Unexpected file. len: {}, start: {}.", line.len(), *start);
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start as usize];
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

pub fn parse_sign_to_file(line:&str, start:&mut i8) -> i8 {
    if line.len() < *start as usize + 1 {
        panic!("Failed: Nothing file.");
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start as usize];
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

pub fn parse_sign_to_promotion(line:&str, start:&mut i8) -> bool {
    if line.len() < *start as usize + 1 {
        return false;
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start as usize];
    if sign == '+' {
        *start += 1;
        true
    } else {
        false
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    pub sourceFile:i8,
    pub sourceRank:i8,
    pub destinationFile:i8,
    pub destinationRank:i8,
    pub promotion:bool,
    pub drop:Option<PieceType>,
}
impl Move {
    pub fn new() -> Move {
        use record::PieceType::*;
        Move {
            sourceFile:0,
            sourceRank:0,
            destinationFile:0,
            destinationRank:0,
            promotion:false,
            drop:None,
        }
    }

    pub fn to_sign(&self) -> String {
        use record::PieceType::*;

        let mut sign = String::new();

        if self.drop != None {
            sign.push_str(&format!("{}*", piece_type_to_sign(&self.drop)));
        } else {
            sign.push_str(&format!("{}{}", self.sourceFile, rank_to_sign(self.sourceRank)));
        }

        sign.push_str(&format!("{}{}", self.destinationFile, rank_to_sign(self.destinationRank)));

        if self.promotion {
            sign.push_str("+");
        }

        sign
    }
}

pub struct Record {
    pub items : Vec<Move>,
}
impl Record {
    pub fn new() -> Record {
        Record {
            items:Vec::new(),
        }
    }

    pub fn push(&mut self, mov:&Move) {
        self.items.push(*mov);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn parse(&mut self, line:&str, start:&mut i8) {
        self.items.clear();

        loop {
            let drop = parse_sign_to_drop(line, start);

            let mut sourceFile = 0;
            let mut sourceRank = 0;
            if drop == None {
                sourceFile = parse_sign_to_file(line, start);
                sourceRank = parse_sign_to_rank(line, start);
            }

            let destinationFile = parse_sign_to_file(line, start);
            let destinationRank = parse_sign_to_rank(line, start);

            let mut promotion =
                if drop == None {
                    parse_sign_to_promotion(line, start)
                } else {
                    false
                };

            self.items.push(Move {
                sourceFile: sourceFile,
                sourceRank: sourceRank,
                destinationFile: destinationFile,
                destinationRank: destinationRank,
                promotion: promotion,
                drop: drop,
            });

            if *start as usize + 1 < line.len() {
                *start += 1;
            } else {
                break;
            }
        }
    }

    pub fn get_current_phase(&self) -> Phase {
        match self.items.len() % 2 {
            0 => Phase::First,
            _ => Phase::Second,
        }
    }
}
