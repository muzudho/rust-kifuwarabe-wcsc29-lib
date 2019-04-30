use instrument::piece_etc::*;
use instrument::position::*;
use live::ohashi_player::*;
use sheet_music_format::kifu_usi::usi_move::*;
use sheet_music_format::kifu_usi::usi_position::*;
use std::*;
use studio::address::*;
use studio::application::Application;
use studio::board_size::*;
use studio::communication::*;
use video_recorder::cassette_deck::*;

/// フォーサイス エドワーズ記法
pub struct Fen {}
impl Fen {
    pub fn do_sfen(line: &str, start: &mut usize, position: &mut Position) -> bool {
        // ゲームに使う駒がまだ決まっていないところから始めます。
        position.reset_empty_position();

        let rank = 9;
        let mut file = 1;

        let sign = line.to_string().chars().next().unwrap();
        let mut spaces = match sign {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '/' => -1,
            _ => 0,
        };

        if spaces == 0 {
            let piece_opt = parse_sign_line_to_piece(line, start);
            position.activate_piece(piece_opt, Cell::from_file_rank(file, rank));
        /* file += 1; */
        } else if spaces == -1 {
            /* file = 1; */
            /* rank = 9; */
        } else {
            while spaces > 0 {
                position.set_id_piece(Cell::from_file_rank(file, rank), None);
                file += 1;
                spaces -= 1;
            }
        }
        true
    }

    // 本将棋のオリジン局面から、初期局面解析。
    pub fn parse_initial_position(
        line: &str,
        start: &mut usize,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) -> bool {
        match UsiPosition::parse_startpos_test(line, start, &app.comm) {
            Some(is_startpos) => {
                if is_startpos {
                    // 大橋流を指せるところまで、局面を戻す☆（＾～＾）
                    OhashiPlayer::clear_to_honshogi_origin(position, deck, &app);
                    // 大橋流で初期局面まで指す☆（＾～＾）
                    OhashiPlayer::play_ohashi_starting(position, deck, &app);
                    true
                } else {
                    // 指定局面を、初期局面とする☆（＾～＾）
                    Fen::do_sfen(line, start, position)
                }
            }
            None => false,
        }
    }

    /// ex.) Parses 7g7f.
    pub fn parse_usi_1move(
        _comm: &Communication,
        line: &str,
        start: &mut usize,
        board_size: BoardSize,
    ) -> UsiMove {
        let drop_opt = parse_sign_to_drop(line, start);
        let source_opt = if drop_opt == None {
            Some(Cell::from_file_rank(
                parse_sign_to_file(line, start),
                parse_sign_to_rank(line, start),
            ))
        } else {
            None
        };

        let destination = Cell::from_file_rank(
            parse_sign_to_file(line, start),
            parse_sign_to_rank(line, start),
        );

        let promotion_flag = if drop_opt == None {
            parse_sign_to_promotion(line, start)
        } else {
            false
        };

        if let Some(drop) = drop_opt {
            UsiMove::create_drop(destination, drop, board_size)
        } else {
            UsiMove::create_walk(source_opt.unwrap(), destination, promotion_flag, board_size)
        }
    }
}

// フォーサイス エドワーズ記法に出てくる駒１つ分の読み込み。1～2文字。
pub fn parse_sign_line_to_piece(line: &str, start: &mut usize) -> Option<Piece> {
    use instrument::piece_etc::Piece::*;

    // スタートが文字列の終端を読み終わっていれば、結果は空。
    if line.len() <= *start as usize {
        return None;
    }

    match &line[*start..=*start] {
        "+" => {
            // 1文字目が + なら２文字。
            *start += 1;
            match &line[*start..=*start] {
                "R" => {
                    *start += 1;
                    Some(PR1)
                }
                "B" => {
                    *start += 1;
                    Some(PB1)
                }
                "S" => {
                    *start += 1;
                    Some(PS1)
                }
                "N" => {
                    *start += 1;
                    Some(PN1)
                }
                "L" => {
                    *start += 1;
                    Some(PL1)
                }
                "P" => {
                    *start += 1;
                    Some(PP1)
                }
                "r" => {
                    *start += 1;
                    Some(PR2)
                }
                "b" => {
                    *start += 1;
                    Some(PB2)
                }
                "s" => {
                    *start += 1;
                    Some(PS2)
                }
                "n" => {
                    *start += 1;
                    Some(PN2)
                }
                "l" => {
                    *start += 1;
                    Some(PL2)
                }
                "p" => {
                    *start += 1;
                    Some(PP2)
                }
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        }
        _ => {
            // 1文字の符号。
            *start += 1;
            match &line[*start..=*start] {
                "K" => {
                    *start += 1;
                    Some(K1)
                }
                "R" => {
                    *start += 1;
                    Some(R1)
                }
                "B" => {
                    *start += 1;
                    Some(B1)
                }
                "G" => {
                    *start += 1;
                    Some(G1)
                }
                "S" => {
                    *start += 1;
                    Some(S1)
                }
                "N" => {
                    *start += 1;
                    Some(N1)
                }
                "L" => {
                    *start += 1;
                    Some(L1)
                }
                "P" => {
                    *start += 1;
                    Some(P1)
                }
                "k" => {
                    *start += 1;
                    Some(K2)
                }
                "r" => {
                    *start += 1;
                    Some(R2)
                }
                "b" => {
                    *start += 1;
                    Some(B2)
                }
                "g" => {
                    *start += 1;
                    Some(G2)
                }
                "s" => {
                    *start += 1;
                    Some(S2)
                }
                "n" => {
                    *start += 1;
                    Some(N2)
                }
                "l" => {
                    *start += 1;
                    Some(L2)
                }
                "p" => {
                    *start += 1;
                    Some(P2)
                }
                _ => panic!("Failed: Sfen unexpected piece."),
            }
        }
    }
}
