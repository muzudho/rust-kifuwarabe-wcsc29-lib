use std::*;

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
impl fmt::Display for PieceType{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use moves::PieceType::*;
        match *self {
            K => { write!(f," K")},
            R => { write!(f," R")},
            B => { write!(f," B")},
            G => { write!(f," G")},
            S => { write!(f," S")},
            N => { write!(f," N")},
            L => { write!(f," L")},
            P => { write!(f," P")},
            PR => { write!(f,"+R")},
            PB => { write!(f,"+B")},
            PS => { write!(f,"+S")},
            PN => { write!(f,"+N")},
            PL => { write!(f,"+L")},
            PP => { write!(f,"+P")},
            Empty => { write!(f,"  ")},
            Num => { write!(f,"??")},
        }
    }
}

pub fn parse_sign_to_drop(line:&str, start:&mut i8) -> PieceType {
    use moves::PieceType::*;

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
        _ => panic!("Failed: Unexpected rank.")
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
        _ => panic!("Failed: Unexpected file. line '{}' at {}, '{}'.", line, *start-1, sign)
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
        use moves::PieceType::*;
        Move {
            sourceFile:0,
            sourceRank:0,
            destinationFile:0,
            destinationRank:0,
            promotion:false,
            drop:Empty,
        }
    }
}

pub struct Moves {
    pub items : Vec<Move>,
}
impl Moves {
    pub fn new() -> Moves {
        Moves {
            items:Vec::new(),
        }
    }

    pub fn parse(&mut self, line:&str, start:&mut i8) {
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
}
