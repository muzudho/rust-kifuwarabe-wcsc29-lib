use fen::*;
use std::*;
use position::*;
use logical_move::*;
use physical_move::*;

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
pub fn piece_type_to_sign(piece_type_opt:Option<PieceType>) -> String {
    use logical_record::PieceType::*;
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
pub fn piece_type_to_piece(phase:Phase, piece_type:PieceType) -> Piece {
    use position::Phase::*;
    use position::Piece::*;
    use logical_record::PieceType::*;
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
                _ => K1, // TODO Error
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
                _ => K2, // TODO Error
            }
        },
    }
}

pub fn parse_sign_to_drop(line:&str, start:&mut i8) -> Option<PieceType> {
    use logical_record::PieceType::*;

    if line.len() < *start as usize + 2 {
        return None;
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start as usize];
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
    let sign = v[*start as usize];
    if sign == '*' {
        *start += 2;
        Some(piece_type)
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

pub struct LogicalRecord {
    pub position: Position,
    pub items : Vec<LogicalMove>,
}
impl LogicalRecord {
    pub fn new() -> LogicalRecord {
        LogicalRecord {
            position: Position::new(),
            items: Vec::new(),
        }
    }

    pub fn parse1(&mut self, line:&str) {

        self.clear();

        let mut start = 0;

        if line.starts_with("position startpos") {
            self.position.board.set_startpos();
            
            if line.len() > 17 {
                // `position startpos moves `. [0]p, [1]o, ...
                start = 24;

                // Examples.
                // position startpos moves 2g2f 8c8d
                let mut temp_record = LogicalRecord::new();
                temp_record.parse2(line, &mut start);
                println!("info temp_record.items.len: {}", temp_record.items.len());

                // TODO 指し手通り、進めたい。
                for mov in &temp_record.items {
                    println!("info Move: `{}`.", mov.to_sign());
                    self.make_move(*mov);
                    self.position.board.print(self.get_current_phase());
                }
            }
        } else if line.starts_with("position sfen ") {
            // TODO sfen under construction.

            // `position sfen `. [0]p, [1]o, ...
            start = 14;
            let mut rank=9;
            let mut file=1;

            let sign = line.to_string().chars().next().unwrap();
            let mut spaces = match sign {
                '1' => {1},
                '2' => {2},
                '3' => {3},
                '4' => {4},
                '5' => {5},
                '6' => {6},
                '7' => {7},
                '8' => {8},
                '9' => {9},
                '/' => {-1},
                _ => {0},
            };

            if spaces == 0 {
                self.position.board.set_piece(file, rank, parse_sign_line_to_piece(line, &mut start));
                file += 1;
            } else if spaces == -1 {
                file = 1;
                rank = 9;
            } else {
                while spaces > 0 {
                    self.position.board.set_piece(file, rank, None);
                    file += 1;
                    spaces -= 1;
                }
            }

            loop {
                let sign = line.to_string().chars().next().unwrap();
                if sign == ' ' {
                    break;
                }
            }
        }
    }

    pub fn push(&mut self, mov:LogicalMove) {
        self.items.push(mov);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn parse2(&mut self, line:&str, start:&mut i8) {
        self.items.clear();

        loop {
            let lmove = Fen::parse3(&line, start);
            self.items.push(lmove);

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

    pub fn make_move(&mut self, mov:LogicalMove){
        use logical_record::PieceType::*;
        
        if mov.drop != None {
            // TODO drop

        } else {
            let mut source_piece = self.position.remove_piece(mov.source_file, mov.source_rank);
            if mov.promotion {
                source_piece = promotion_piece(source_piece);
            }
            self.position.board.set_piece(mov.destination_file, mov.destination_rank, source_piece);
            self.push(mov);
        }
    }
}
