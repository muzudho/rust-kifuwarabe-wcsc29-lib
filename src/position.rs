use record::*;
use std::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    First,
    Second,
    Empty,
}
pub fn player_to_sign(player:&Player) -> String {
    use position::Player::*;
    match *player {
        First => "b".to_string(),
        Second => "w".to_string(),
        Empty => "x".to_string(),
        _ => panic!("Unexpected player. *player as usize = {}.", *player as usize),
    }
}

pub const FILE_LEN: i8 = 9;
pub const RANK_LEN: i8 = 9;
pub fn file_rank_to_cell(file:i8, rank:i8) -> usize {
    (rank*FILE_LEN + file) as usize
}
pub fn cell_to_file_rank(cell:usize) -> (i8, i8) {
    ((cell%FILE_LEN as usize) as i8, (cell/FILE_LEN as usize) as i8)
}
pub fn reverse_cell(cell:usize) -> usize {
    RANK_LEN as usize * FILE_LEN as usize - cell
}

/// First turn player is 0.
/// Second turn player is 1.
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
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
    Num,
}
pub fn piece_to_sign(piece:&Piece) -> String {
    use position::Piece::*;
    match *piece {
        K0 => "K".to_string(),
        R0 => "R".to_string(),
        B0 => "B".to_string(),
        G0 => "G".to_string(),
        S0 => "S".to_string(),
        N0 => "N".to_string(),
        L0 => "L".to_string(),
        P0 => "P".to_string(),
        PR0 => "+R".to_string(),
        PB0 => "+B".to_string(),
        PS0 => "+S".to_string(),
        PN0 => "+N".to_string(),
        PL0 => "+L".to_string(),
        PP0 => "+P".to_string(),
        K1 => "k".to_string(),
        R1 => "r".to_string(),
        B1 => "b".to_string(),
        G1 => "g".to_string(),
        S1 => "s".to_string(),
        N1 => "n".to_string(),
        L1 => "l".to_string(),
        P1 => "p".to_string(),
        PR1 => "+r".to_string(),
        PB1 => "+b".to_string(),
        PS1 => "+s".to_string(),
        PN1 => "+n".to_string(),
        PL1 => "+l".to_string(),
        PP1 => "+p".to_string(),
        Empty => "".to_string(),
        Num => "?".to_string(),
    }
}
pub fn piece_to_piece_type(piece:&Piece) -> PieceType {
    use position::Piece::*;
    use record::PieceType::*;
    match *piece {
        K0 => K,
        R0 => R,
        B0 => B,
        G0 => G,
        S0 => S,
        N0 => N,
        L0 => L,
        P0 => P,
        PR0 => PR,
        PB0 => PB,
        PS0 => PS,
        PN0 => PN,
        PL0 => PL,
        PP0 => PP,
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
        Piece::Empty => PieceType::Empty,
        Piece::Num => PieceType::Empty,
    }
}
pub fn piece_to_player(piece:&Piece) -> Player {
    use position::Piece::*;
    match *piece {
        K0 | R0 | B0 | G0 | S0 | N0 | L0 | P0 | PR0 | PB0 | PS0 | PN0 | PL0 | PP0 => Player::First,
        K1 | R1 | B1 | G1 | S1 | N1 | L1 | P1 | PR1 | PB1 | PS1 | PN1 | PL1 | PP1 => Player::Second,
        Empty => Player::Empty,
        Num => panic!("Unexpected player. *piece as usize = {}.", *piece as usize),
    }
}

pub fn parse_sign_to_piece(line:&str, start:&mut i8) -> Piece {
    use position::Piece::*;

    if line.len() <= *start as usize + 1 {
        return Empty;
    }

    let sign = line.to_string().chars().next().unwrap();
    let pieceType = match sign {
        'K' => K0,
        'R' => R0,
        'B' => B0,
        'G' => G0,
        'S' => S0,
        'N' => N0,
        'L' => L0,
        'P' => P0,
        'k' => K1,
        'r' => R1,
        'b' => B1,
        'g' => G1,
        's' => S1,
        'n' => N1,
        'l' => L1,
        'p' => P1,
        '+' => {
            let sign = line.to_string().chars().next().unwrap();
            match sign {
                'R' => PR0,
                'B' => PB0,
                'S' => PS0,
                'N' => PN0,
                'L' => PL0,
                'P' => PP0,
                'r' => PR1,
                'b' => PB1,
                's' => PS1,
                'n' => PN1,
                'l' => PL1,
                'p' => PP1,
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        _ => panic!("Failed: Sfen unexpected piece."),
    };

    let sign = line.to_string().chars().next().unwrap();
    if sign == '*' {
        *start += 2;
        pieceType
    } else {
        panic!("Failed: Sfen unexpected drop.");
    }
}

pub fn promotion_piece(piece:&Piece) -> Piece {
    use position::Piece::*;

    match piece {
        R0 => PR0,
        B0 => PB0,
        S0 => PS0,
        N0 => PN0,
        L0 => PL0,
        P0 => PP0,
        R1 => PR1,
        B1 => PB1,
        S1 => PS1,
        N1 => PN1,
        L1 => PL1,
        P1 => PP1,
        _ => panic!("Failed: Sfen unexpected promotion.")
    }
}

// TODO
// #[derive(Clone, Copy)]
pub struct Position {
    // With frame. 11x11.
    pub board : [Piece;121],
    pub record : Record,
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
            record: Record::new(),
        }
    }

    pub fn parse(&mut self, line:&str) {
        use position::Piece::*;

        self.record.clear();

        let mut start = 0;

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
            
            if line.len() > 17 {
                // `position startpos moves `. [0]p, [1]o, ...
                start = 24;

                // Examples.
                // position startpos moves 2g2f 8c8d
                let mut temp_record = Record::new();
                temp_record.parse(line, &mut start);
                println!("info temp_record.items.len: {}", temp_record.items.len());

                // TODO 指し手通り、進めたい。
                for mov in &temp_record.items {
                    println!("info Move: `{}`.", mov.to_sign());
                    self.make_move(mov);
                    self.show_board();
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
                self.set_piece(file, rank, parse_sign_to_piece(line, &mut start));
                file += 1;
            } else if spaces == -1 {
                file = 1;
                rank = 9;
            } else {
                while spaces > 0 {
                    self.set_piece(file, rank, Empty);
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

    pub fn show_board(&self) {
        println!("info show_board begin...");
        
        let rank_array = ['?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

        for rank in 1..=9 {
            println!(
                "info {0} {1: >2}{2: >2}{3: >2}{4: >2}{5: >2}{6: >2}{7: >2}{8: >2}{9: >2}",
                rank_array[(10-rank) as usize],
                piece_to_sign(&self.get_piece(1, rank)),
                piece_to_sign(&self.get_piece(2, rank)),
                piece_to_sign(&self.get_piece(3, rank)),
                piece_to_sign(&self.get_piece(4, rank)),
                piece_to_sign(&self.get_piece(5, rank)),
                piece_to_sign(&self.get_piece(6, rank)),
                piece_to_sign(&self.get_piece(7, rank)),
                piece_to_sign(&self.get_piece(8, rank)),
                piece_to_sign(&self.get_piece(9, rank)));
        }
        println!("info    1 2 3 4 5 6 7 8 9");
        println!("info show_board end...");
    }

    pub fn get_piece(&self, file:i8, rank:i8) -> Piece {
        self.board[file_rank_to_cell(file, rank)]
    }

    fn remove_piece(&mut self, file:i8, rank:i8) -> Piece {
        use position::Piece::*;
        let piece = self.board[file_rank_to_cell(file, rank)];
        self.set_piece(file, rank, Empty);
        piece
    }

    pub fn set_piece(&mut self, file:i8, rank:i8, piece:Piece) {
        self.board[file_rank_to_cell(file, rank)] = piece;
    }

    pub fn make_move(&mut self, mov:&Move){
        use record::PieceType::*;
        
        if mov.drop != Empty {
            // TODO drop

        } else {
            let mut source_piece = self.remove_piece(mov.sourceFile, mov.sourceRank);
            if mov.promotion {
                source_piece = promotion_piece(&source_piece);
            }
            self.set_piece(mov.destinationFile, mov.destinationRank, source_piece);
            self.record.push(mov);
        }
    }
}