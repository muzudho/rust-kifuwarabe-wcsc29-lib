/// フォーサイス エドワーズ記法
use communication::*;
use position::*;
use usi_conv::usi_move::*;
// use usi_conv::usi_record::*;
// use parser::*;
use piece_etc::*;
use std::*;

pub struct Fen {

}
impl Fen {
    // 解析と、局面の編集は同時に行う。
    pub fn parse_position(comm:&Communication, line:&str, start:&mut usize, position:&mut Position) -> bool {
        if line.starts_with("position startpos") {
            comm.println("#position startpos");
            // 平手初期局面にリセット。
            *start = "position startpos".len();
            position.reset_startpos();
            true
        } else if line.starts_with("position sfen ") {
            comm.println("#position sfen ");
            // TODO 初期局面を設定。
            position.reset_default();

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
                let piece_opt = parse_sign_line_to_piece(line, start);
                position.activate_piece(piece_opt, file, rank);
                file += 1;
            } else if spaces == -1 {
                file = 1;
                rank = 9;
            } else {
                while spaces > 0 {
                    position.set_id_piece(file, rank, None);
                    file += 1;
                    spaces -= 1;
                }
            }
            true
        } else {
            false
        }
    }

    /// ex.) Parses 7g7f.
    pub fn parse_usi_1move(comm:&Communication, line:&str, start:&mut usize) -> UsiMove {
        let drop_opt = parse_sign_to_drop(line, start);
        let mut source_file_num = 0;
        let mut source_rank_num = 0;
        if drop_opt == None {
            source_file_num = parse_sign_to_file(line, start);
            source_rank_num = parse_sign_to_rank(line, start);
        };

        let destination_file_num = parse_sign_to_file(line, start);
        let destination_rank_num = parse_sign_to_rank(line, start);
        let promotion_flag =
            if drop_opt == None {
                parse_sign_to_promotion(line, start)
            } else {
                false
            };

        UsiMove::create(
            source_file_num,
            source_rank_num,
            destination_file_num,
            destination_rank_num,
            promotion_flag,
            drop_opt)
    }
}

// フォーサイス エドワーズ記法に出てくる駒１つ分の読み込み。1～2文字。
pub fn parse_sign_line_to_piece(line:&str, start:&mut usize) -> Option<Piece> {
    use piece_etc::Piece::*;

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
