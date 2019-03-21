/// フォーサイス エドワーズ記法
use board::*;
use logical_move::*;
use logical_record::*;
use position::*;
use std::*;

pub struct Fen {

}
impl Fen {
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
