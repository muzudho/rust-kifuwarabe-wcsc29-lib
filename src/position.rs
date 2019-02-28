use board::*;
use record::*;
use std::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    /// Starting first.
    First,
    /// Starting second.
    Second,
}
pub fn phase_to_sign(phase:&Phase) -> String {
    use position::Phase::*;
    match *phase {
        First => "b".to_string(),
        Second => "w".to_string(),
        _ => panic!("Unexpected phase. *phase as usize = {}.", *phase as usize),
    }
}

/// First turn phase is 0.
/// Second turn phase is 1.
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
}
pub fn piece_to_sign(piece:&Option<Piece>) -> String {
    match piece {
        Some(x) => {
            use position::Piece::*;
            match x {
                K0 => "K",
                R0 => "R",
                B0 => "B",
                G0 => "G",
                S0 => "S",
                N0 => "N",
                L0 => "L",
                P0 => "P",
                PR0 => "+R",
                PB0 => "+B",
                PS0 => "+S",
                PN0 => "+N",
                PL0 => "+L",
                PP0 => "+P",
                K1 => "k",
                R1 => "r",
                B1 => "b",
                G1 => "g",
                S1 => "s",
                N1 => "n",
                L1 => "l",
                P1 => "p",
                PR1 => "+r",
                PB1 => "+b",
                PS1 => "+s",
                PN1 => "+n",
                PL1 => "+l",
                PP1 => "+p",
                Empty => "",
                Num => "?",
            }
        },
        None => { "" }
    }.to_string()
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
    }
}
pub fn piece_to_phase(piece:&Option<Piece>) -> Option<Phase> {
    match piece {
        Some(x) => {
            use position::Piece::*;
            match x {
                K0 | R0 | B0 | G0 | S0 | N0 | L0 | P0 | PR0 | PB0 | PS0 | PN0 | PL0 | PP0 => Some(Phase::First),
                K1 | R1 | B1 | G1 | S1 | N1 | L1 | P1 | PR1 | PB1 | PS1 | PN1 | PL1 | PP1 => Some(Phase::Second),
                _ => panic!("Unexpected phase. *piece as usize = {}.", *x as usize),
            }
        },
        None => None,
    }
}

// フォーサイス エドワーズ記法に出てくる駒１つ分の読み込み。前空白埋め2文字固定。
pub fn parse_sign_2char_to_piece(line:&str, start:&mut i8) -> Option<Piece> {
    use position::Piece::*;

    // スタートが文字列の終端を読み終わっていれば、結果は空。
    if line.len() < *start as usize {
        return None;
    }

    // とりあえず スタートの数だけ進める。
    let mut sign = '?';
    for i in 0..*start {
        sign = line.to_string().chars().next().unwrap();
    };

    match sign {
        '+' => {
            // 1文字目が + なら２文字。
            let sign = line.to_string().chars().next().unwrap();
            *start += 2;
            match sign {
                'R' => Some(PR0),
                'B' => Some(PB0),
                'S' => Some(PS0),
                'N' => Some(PN0),
                'L' => Some(PL0),
                'P' => Some(PP0),
                'r' => Some(PR1),
                'b' => Some(PB1),
                's' => Some(PS1),
                'n' => Some(PN1),
                'l' => Some(PL1),
                'p' => Some(PP1),
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        ' ' => {
            // 前空白埋めの符号。
            *start += 2;
            match sign {
                'K' => Some(K0),
                'R' => Some(R0),
                'B' => Some(B0),
                'G' => Some(G0),
                'S' => Some(S0),
                'N' => Some(N0),
                'L' => Some(L0),
                'P' => Some(P0),
                'k' => Some(K1),
                'r' => Some(R1),
                'b' => Some(B1),
                'g' => Some(G1),
                's' => Some(S1),
                'n' => Some(N1),
                'l' => Some(L1),
                'p' => Some(P1),
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        _ => panic!("Failed: Sfen unexpected piece."),
    }
}

// フォーサイス エドワーズ記法に出てくる駒１つ分の読み込み。1～2文字。
pub fn parse_sign_line_to_piece(line:&str, start:&mut i8) -> Option<Piece> {
    use position::Piece::*;

    // スタートが文字列の終端を読み終わっていれば、結果は空。
    if line.len() < *start as usize {
        return None;
    }

    // とりあえず スタートの数だけ進める。
    let mut sign = '?';
    for i in 0..*start {
        sign = line.to_string().chars().next().unwrap();
    };

    match sign {
        '+' => {
            // 1文字目が + なら２文字。
            let sign = line.to_string().chars().next().unwrap();
            *start += 2;
            match sign {
                'R' => Some(PR0),
                'B' => Some(PB0),
                'S' => Some(PS0),
                'N' => Some(PN0),
                'L' => Some(PL0),
                'P' => Some(PP0),
                'r' => Some(PR1),
                'b' => Some(PB1),
                's' => Some(PS1),
                'n' => Some(PN1),
                'l' => Some(PL1),
                'p' => Some(PP1),
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        _ => {
            // 1文字の符号。
            *start += 1;
            match sign {
                'K' => Some(K0),
                'R' => Some(R0),
                'B' => Some(B0),
                'G' => Some(G0),
                'S' => Some(S0),
                'N' => Some(N0),
                'L' => Some(L0),
                'P' => Some(P0),
                'k' => Some(K1),
                'r' => Some(R1),
                'b' => Some(B1),
                'g' => Some(G1),
                's' => Some(S1),
                'n' => Some(N1),
                'l' => Some(L1),
                'p' => Some(P1),
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
    }
}

pub fn promotion_piece(piece:&Option<Piece>) -> Option<Piece> {
    match piece {
        Some(x) => {
            use position::Piece::*;
            match x {
                R0 => Some(PR0),
                B0 => Some(PB0),
                S0 => Some(PS0),
                N0 => Some(PN0),
                L0 => Some(PL0),
                P0 => Some(PP0),
                R1 => Some(PR1),
                B1 => Some(PB1),
                S1 => Some(PS1),
                N1 => Some(PN1),
                L1 => Some(PL1),
                P1 => Some(PP1),
                _ => panic!("Failed: Sfen unexpected promotion.")
            }
        },
        None => None,
    }
}

pub struct Position {
    pub board : Board,
    pub record : Record,
}
impl Position {
    pub fn new() -> Position {
        Position {
            board : Board::new(),
            record: Record::new(),
        }
    }

    pub fn parse(&mut self, line:&str) {

        self.record.clear();

        let mut start = 0;

        if line.starts_with("position startpos") {
            self.board.set_startpos();
            
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
                self.set_piece(file, rank, parse_sign_line_to_piece(line, &mut start));
                file += 1;
            } else if spaces == -1 {
                file = 1;
                rank = 9;
            } else {
                while spaces > 0 {
                    self.set_piece(file, rank, None);
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

    pub fn get_piece(&self, file:i8, rank:i8) -> Option<Piece> {
        let cell = self.board.file_rank_to_cell(file, rank);
        self.board.pieces[cell]
    }

    fn remove_piece(&mut self, file:i8, rank:i8) -> Option<Piece> {
        let cell = self.board.file_rank_to_cell(file, rank);
        let piece = self.board.pieces[cell];
        self.set_piece(file, rank, None);
        piece
    }

    pub fn set_piece(&mut self, file:i8, rank:i8, piece:Option<Piece>) {
        let cell = self.board.file_rank_to_cell(file, rank);
        self.board.pieces[cell] = piece;
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