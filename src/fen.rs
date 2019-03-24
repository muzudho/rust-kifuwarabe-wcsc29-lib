/// フォーサイス エドワーズ記法
use board::*;
use logical_move::*;
use logical_record::*;
use parser::*;
use physical_record::*;
use std::*;

pub struct Fen {

}
impl Fen {
    pub fn parse_board(line:&str, start:&mut usize, board:&mut Board) -> bool {
        if line.starts_with("position startpos") {
            *start = "position startpos".len();
            board.reset_startpos();
            true
        } else if line.starts_with("position sfen ") {
            board.reset_default();
            // TODO sfen under construction.

            // `position sfen `. [0]p, [1]o, ...
            *start = 14;
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
                board.set_piece(file, rank, parse_sign_line_to_piece(line, start));
                file += 1;
            } else if spaces == -1 {
                file = 1;
                rank = 9;
            } else {
                while spaces > 0 {
                    board.set_piece(file, rank, None);
                    file += 1;
                    spaces -= 1;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn parse_moves(line:&str, start:&mut usize, board:&mut Board) -> Option<LogicalRecord> {
        if Parser::match_keyword(&line, "moves", start) || 
            Parser::match_keyword(&line, " moves", start) {
        } else {
            return None;
        }

        let mut logical_record = LogicalRecord::new();

        // `position startpos moves `. [0]p, [1]o, ...

        // Examples.
        // position startpos moves 2g2f 8c8d
        let mut temp_record = LogicalRecord::new();
        temp_record.parse2(line, start);
        println!("info temp_record.items.len: {}", temp_record.items.len());

        // TODO 指し手通り、進めたい。
        for mov in &temp_record.items {
            println!("info Move: `{}`.", mov.to_sign());
            logical_record.make_move(*mov, board);
            board.println(logical_record.get_current_phase());
        }

        Some(logical_record)
    }

    pub fn parse3(line:&str, start:&mut usize) -> LogicalMove {
        println!("parse3 start: {0}.", start);
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
pub fn parse_sign_2char_to_piece(line:&str, start:&mut usize) -> Option<Piece> {
    use board::Piece::*;

    // スタートが文字列の終端を読み終わっていれば、結果は空。
    if line.len() <= *start {
        return None;
    }

    match &line[*start..=*start] {
        "+" => {
            // 1文字目が + なら２文字。
            *start += 1;
            match &line[*start..=*start] {
                "R" => {*start += 1; Some(PR1)},
                "B" => {*start += 1; Some(PB1)},
                "S" => {*start += 1; Some(PS1)},
                "N" => {*start += 1; Some(PN1)},
                "L" => {*start += 1; Some(PL1)},
                "P" => {*start += 1; Some(PP1)},
                "r" => {*start += 1; Some(PR2)},
                "b" => {*start += 1; Some(PB2)},
                "s" => {*start += 1; Some(PS2)},
                "n" => {*start += 1; Some(PN2)},
                "l" => {*start += 1; Some(PL2)},
                "p" => {*start += 1; Some(PP2)},
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        " " => {
            // 前空白埋めの符号。
            *start += 1;
            match &line[*start..=*start] {
                "K" => {*start += 1; Some(K1)},
                "R" => {*start += 1; Some(R1)},
                "B" => {*start += 1; Some(B1)},
                "G" => {*start += 1; Some(G1)},
                "S" => {*start += 1; Some(S1)},
                "N" => {*start += 1; Some(N1)},
                "L" => {*start += 1; Some(L1)},
                "P" => {*start += 1; Some(P1)},
                "k" => {*start += 1; Some(K2)},
                "r" => {*start += 1; Some(R2)},
                "b" => {*start += 1; Some(B2)},
                "g" => {*start += 1; Some(G2)},
                "s" => {*start += 1; Some(S2)},
                "n" => {*start += 1; Some(N2)},
                "l" => {*start += 1; Some(L2)},
                "p" => {*start += 1; Some(P2)},
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        _ => panic!("Failed: Sfen unexpected piece."),
    }
}

// フォーサイス エドワーズ記法に出てくる駒１つ分の読み込み。1～2文字。
pub fn parse_sign_line_to_piece(line:&str, start:&mut usize) -> Option<Piece> {
    use board::Piece::*;

    // スタートが文字列の終端を読み終わっていれば、結果は空。
    if line.len() <= *start as usize {
        return None;
    }

    match &line[*start..=*start] {
        "+" => {
            // 1文字目が + なら２文字。
            *start += 1;
            match &line[*start..=*start] {
                "R" => {*start += 1; Some(PR1)},
                "B" => {*start += 1; Some(PB1)},
                "S" => {*start += 1; Some(PS1)},
                "N" => {*start += 1; Some(PN1)},
                "L" => {*start += 1; Some(PL1)},
                "P" => {*start += 1; Some(PP1)},
                "r" => {*start += 1; Some(PR2)},
                "b" => {*start += 1; Some(PB2)},
                "s" => {*start += 1; Some(PS2)},
                "n" => {*start += 1; Some(PN2)},
                "l" => {*start += 1; Some(PL2)},
                "p" => {*start += 1; Some(PP2)},
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
        _ => {
            // 1文字の符号。
            *start += 1;
            match &line[*start..=*start] {
                "K" => {*start += 1; Some(K1)},
                "R" => {*start += 1; Some(R1)},
                "B" => {*start += 1; Some(B1)},
                "G" => {*start += 1; Some(G1)},
                "S" => {*start += 1; Some(S1)},
                "N" => {*start += 1; Some(N1)},
                "L" => {*start += 1; Some(L1)},
                "P" => {*start += 1; Some(P1)},
                "k" => {*start += 1; Some(K2)},
                "r" => {*start += 1; Some(R2)},
                "b" => {*start += 1; Some(B2)},
                "g" => {*start += 1; Some(G2)},
                "s" => {*start += 1; Some(S2)},
                "n" => {*start += 1; Some(N2)},
                "l" => {*start += 1; Some(L2)},
                "p" => {*start += 1; Some(P2)},
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        },
    }
}
