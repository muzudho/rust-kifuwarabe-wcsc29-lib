/// フォーサイス エドワーズ記法
use logical_move::*;
use logical_record::*;
use physical_record::*;
use position::*;
use std::*;

pub struct Fen {

}
impl Fen {
    pub fn parse1(line:&str) -> LogicalRecord {

        let mut logical_record = LogicalRecord::new();

        let mut start = 0;

        if line.starts_with("position startpos") {
            let mut position = Position::startpos();
            
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
                    logical_record.make_move(*mov, &mut position);
                    position.board.print(logical_record.get_current_phase());
                }
            }
        } else if line.starts_with("position sfen ") {
            let mut position = Position::default();
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
                position.board.set_piece(file, rank, parse_sign_line_to_piece(line, &mut start));
                file += 1;
            } else if spaces == -1 {
                file = 1;
                rank = 9;
            } else {
                while spaces > 0 {
                    position.board.set_piece(file, rank, None);
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

        logical_record
    }

    pub fn parse3(line:&str, start:&mut i8) -> LogicalMove {
        let drop = parse_sign_to_drop(line, start);

        let mut source_file = 0;
        let mut source_rank = 0;
        if drop == None {
            source_file = parse_sign_to_file(line, start);
            source_rank = parse_sign_to_rank(line, start);
        }

        let destination_file = parse_sign_to_file(line, start);
        let destination_rank = parse_sign_to_rank(line, start);

        let mut promotion =
            if drop == None {
                parse_sign_to_promotion(line, start)
            } else {
                false
            };

        LogicalMove {
            source_file: source_file,
            source_rank: source_rank,
            destination_file: destination_file,
            destination_rank: destination_rank,
            promotion: promotion,
            drop: drop,
        }
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
                'R' => Some(PR1),
                'B' => Some(PB1),
                'S' => Some(PS1),
                'N' => Some(PN1),
                'L' => Some(PL1),
                'P' => Some(PP1),
                'r' => Some(PR2),
                'b' => Some(PB2),
                's' => Some(PS2),
                'n' => Some(PN2),
                'l' => Some(PL2),
                'p' => Some(PP2),
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        ' ' => {
            // 前空白埋めの符号。
            *start += 2;
            match sign {
                'K' => Some(K1),
                'R' => Some(R1),
                'B' => Some(B1),
                'G' => Some(G1),
                'S' => Some(S1),
                'N' => Some(N1),
                'L' => Some(L1),
                'P' => Some(P1),
                'k' => Some(K2),
                'r' => Some(R2),
                'b' => Some(B2),
                'g' => Some(G2),
                's' => Some(S2),
                'n' => Some(N2),
                'l' => Some(L2),
                'p' => Some(P2),
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
                'R' => Some(PR1),
                'B' => Some(PB1),
                'S' => Some(PS1),
                'N' => Some(PN1),
                'L' => Some(PL1),
                'P' => Some(PP1),
                'r' => Some(PR2),
                'b' => Some(PB2),
                's' => Some(PS2),
                'n' => Some(PN2),
                'l' => Some(PL2),
                'p' => Some(PP2),
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        _ => {
            // 1文字の符号。
            *start += 1;
            match sign {
                'K' => Some(K1),
                'R' => Some(R1),
                'B' => Some(B1),
                'G' => Some(G1),
                'S' => Some(S1),
                'N' => Some(N1),
                'L' => Some(L1),
                'P' => Some(P1),
                'k' => Some(K2),
                'r' => Some(R2),
                'b' => Some(B2),
                'g' => Some(G2),
                's' => Some(S2),
                'n' => Some(N2),
                'l' => Some(L2),
                'p' => Some(P2),
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
    }
}
