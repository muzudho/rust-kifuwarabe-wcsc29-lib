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
    // No drop.
    Empty,
    // Num is size or error.
    Num
}
pub fn piece_type_to_sign(piece_type:&PieceType) -> String {
    use record::PieceType::*;
    match *piece_type {
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
}

pub fn parse_sign_to_drop(line:&str, start:&mut i8) -> PieceType {
    use record::PieceType::*;

    if line.len() < *start as usize + 2 {
        return Empty;
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
        _ => {return Empty;},
    };

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start as usize];
    if sign == '*' {
        *start += 2;
        pieceType
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
    pub drop:PieceType
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
            drop:Empty,
        }
    }

    pub fn to_sign(&self) -> String {
        use record::PieceType::*;

        let mut sign = String::new();

        if self.drop != Empty {
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
            if drop == PieceType::Empty {
                sourceFile = parse_sign_to_file(line, start);
                sourceRank = parse_sign_to_rank(line, start);
            }

            let destinationFile = parse_sign_to_file(line, start);
            let destinationRank = parse_sign_to_rank(line, start);

            let mut promotion =
                if drop == PieceType::Empty {
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

    pub fn get_current_player(&self) -> Player {
        match self.items.len() % 2 {
            0 => Player::First,
            _ => Player::Second,
        }
    }
}
