use instrument::piece_etc::{PhysicalSign, PieceType};
use std::fmt;
///
/// Rpm棋譜のノートの操作属性。
///
/// 局面から独立しています。
///
use studio::address::{Address, Cell};
use studio::application::Application;
use studio::board_size::BoardSize;
use studio::common::caret::*;
use studio::common::closed_interval::*;
use studio::parser::Parser;

/// Vector に入れるときコピーする。
//#[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub struct ShogiNoteOpe {
    pub address: Option<Address>,
    /// +
    pub fingertip_turn: bool,
    /// -
    pub fingertip_rotate: bool,
    /// フェーズ・チェンジなら Ply、数が省略されている場合は -1。フェーズ・チェンジでないなら None。
    phase_change: Option<i16>,
    resign: bool,
}
impl fmt::Display for ShogiNoteOpe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.address {
            Some(address) => write!(f, "{}", address.get_index()),
            None => {
                if self.fingertip_turn {
                    write!(f, "+")
                } else if self.fingertip_rotate {
                    write!(f, "-")
                } else if let Some(ply) = self.phase_change {
                    if ply > -1 {
                        write!(f, "[{}]", ply)
                    } else {
                        write!(f, "|")
                    }
                } else if self.resign {
                    write!(f, "%resign")
                } else {
                    write!(f, "PANIC!")
                }
            }
        }
    }
}
impl ShogiNoteOpe {
    pub fn from_address(address: Address) -> Self {
        ShogiNoteOpe {
            address: Some(address),
            fingertip_turn: false,
            fingertip_rotate: false,
            phase_change: None,
            resign: false,
        }
    }

    pub fn turn_over() -> Self {
        ShogiNoteOpe {
            address: None,
            fingertip_turn: true,
            fingertip_rotate: false,
            phase_change: None,
            resign: false,
        }
    }

    pub fn rotate() -> Self {
        ShogiNoteOpe {
            address: None,
            fingertip_turn: false,
            fingertip_rotate: true,
            phase_change: None,
            resign: false,
        }
    }

    pub fn change_phase(ply: i16) -> Self {
        ShogiNoteOpe {
            address: None,
            fingertip_turn: false,
            fingertip_rotate: false,
            phase_change: Some(ply),
            resign: false,
        }
    }

    pub fn resign() -> Self {
        ShogiNoteOpe {
            address: None,
            fingertip_turn: false,
            fingertip_rotate: false,
            phase_change: None,
            resign: true,
        }
    }

    pub fn is_phase_change(&self) -> bool {
        if let Some(_ply) = self.phase_change {
            true
        } else {
            false
        }
    }

    /// Position に変更を与えずに行える動作☆（＾～＾）
    pub fn is_resign(&self) -> bool {
        self.resign
    }

    pub fn get_phase_change(&self) -> Option<i16> {
        self.phase_change
    }

    /// Human presentable.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        match self.address {
            Some(address) => {
                // 人に読みやすいセル表記にします。
                if address.is_fingertip() {
                    "SK".to_string()
                } else if address.is_on_board(board_size) {
                    board_size
                        .address_to_cell(address.get_index())
                        .to_human_presentable()
                } else if address.is_hand() {
                    address.get_hand_piece().unwrap().to_human_presentable()
                } else {
                    panic!(
                        "Unexpected address: {}.",
                        address.to_human_presentable(board_size)
                    )
                }
            }
            None => {
                if self.fingertip_turn {
                    "+".to_string()
                } else if self.fingertip_rotate {
                    "-".to_string()
                } else if let Some(ply) = self.phase_change {
                    if ply > -1 {
                        format!("[{}]", ply).to_string()
                    } else {
                        "|".to_string()
                    }
                } else if self.resign {
                    "%resign".to_string()
                } else {
                    "PANIC!".to_string()
                }
            }
        }
    }

    pub fn to_sign(&self, board_size: BoardSize) -> String {
        match self.address {
            Some(address) => address.to_physical_sign(board_size),
            None => {
                if self.fingertip_turn {
                    "+".to_string()
                } else if self.fingertip_rotate {
                    "-".to_string()
                } else if let Some(ply) = self.phase_change {
                    if ply > -1 {
                        format!("[{}]", ply)
                    } else {
                        "|".to_string()
                    }
                } else if self.resign {
                    "%resign".to_string()
                } else {
                    panic!("Unexpected physical move print.")
                }
            }
        }
    }

    /// 次のノート１つ読取☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `caret` - Token caret.
    ///
    /// # Returns
    ///
    /// (last_used_caret, note_ope_opt)
    pub fn parse_1ope(
        line: &str,
        caret: &mut Caret,
        board_size: BoardSize,
        app: &Application,
    ) -> (ClosedInterval, Option<ShogiNoteOpe>) {
        let mut closed_interval = ClosedInterval::new_facing_right();

        let mut n0 = caret.go_to_next(&app) as usize;
        closed_interval.intersect_caret_number(n0 as i16);

        let mut ch0 = line[n0..=n0].chars().nth(0).unwrap();
        match ch0 {
            ' ' => (closed_interval, None),
            '0' => {
                // 駒台。
                let mut n1 = caret.go_to_next(&app) as usize;
                let mut ch1 = line[n1..=n1].chars().nth(0).unwrap();

                if 2 < line.len() {
                    match ch1 {
                        'P' | 'p' | 'ﾅ' => {
                            // 成り駒を駒台に置いた、という記号 P,p,ﾅ は読み飛ばします。この経路では 1つずれます。
                            // ただし、ポーンの P, p と被っているので、次の文字があれば成り駒、なければキャンセルを判断します。

                            n1 = caret.go_to_next(&app) as usize;
                            ch1 = line[n1..=n1].chars().nth(0).unwrap();
                        }
                        _ => {}
                    };
                }

                // 駒の種類、フェーズ。
                let piece = PhysicalSign::default(ch1.to_string()).to_piece();

                //comm.print(&format!("{}{}{}", ch1, text15, ch2));
                let address =
                    Address::from_hand_ph_pt(piece.get_phase(), PieceType::from_piece(piece));
                //comm.println(&format!("address index = {}.", address.get_index()));

                closed_interval.intersect_caret_number(n1 as i16);
                (closed_interval, Some(ShogiNoteOpe::from_address(address)))
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // セル
                let mut n1 = caret.go_to_next(&app) as usize;
                let mut ch1 = line[n1..=n1].chars().nth(0).unwrap();

                // comm.print(&format!("Parse1Op: '{}', '{}'.", ch0, ch1));
                let address = Address::from_cell(
                    Cell::from_file_rank(
                        Parser::file_char_to_i8(ch0),
                        Parser::rank_char_to_i8(ch1),
                    ),
                    board_size,
                );

                closed_interval.intersect_caret_number(n1 as i16);
                (closed_interval, Some(ShogiNoteOpe::from_address(address)))
            }
            '+' => {
                // 成り。
                //comm.print(&ch1.to_string());
                (closed_interval, Some(ShogiNoteOpe::turn_over()))
            }
            '-' => {
                // １８０°回転。
                //comm.print(&ch1.to_string());
                (closed_interval, Some(ShogiNoteOpe::rotate()))
            }
            '|' => {
                // フェーズ交代。Ply は分からない。
                (closed_interval, Some(ShogiNoteOpe::change_phase(-1)))
            }
            '[' => {
                // フェーズ交代。 ']' まで読み飛ばす。
                let mut ply = 0;
                loop {
                    if line.len() <= n0 {
                        break;
                    }

                    n0 = caret.go_to_next(&app) as usize;
                    ch0 = line[n0..=n0].chars().nth(0).unwrap();
                    closed_interval.intersect_caret_number(n0 as i16);

                    if ch0 == ']' {
                        break;
                    }

                    // Ply カウント。
                    let num: i16 = ch0.to_string().parse::<i16>().unwrap();
                    ply *= 10;
                    ply += num;
                }
                (closed_interval, Some(ShogiNoteOpe::change_phase(ply)))
            }
            _ => {
                let last = line.len();
                panic!("Unexpected line '{}'.", &line[n0..last]);
            }
        }
    }
}
