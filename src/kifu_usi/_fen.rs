// フォーサイス エドワーズ記法に出てくる駒１つ分の読み込み。前空白埋め2文字固定。
pub fn parse_sign_2char_to_piece(line:&str, start:&mut usize) -> Option<Piece> {
    use position::Piece::*;

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
