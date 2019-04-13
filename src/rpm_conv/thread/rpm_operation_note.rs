use address::*;
use communication::*;
use parser::*;
use piece_etc::*;
use position::*;

/// Vector に入れるときコピーする。
#[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub struct RpmOpeNote {
    pub address: Option<Address>,
    /// +
    pub sky_turn: bool,
    /// -
    pub sky_rotate: bool,
    phase_change: bool,
    resign: bool,
}
/*
impl fmt::Display for RpmOpeNote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.address {
            Some(address) => {
                write!(f, "{}", address.get_index())
            },
            None => {
                if self.sky_turn {
                    write!(f, "+")
                } else if self.sky_rotate {
                    write!(f, "-")
                } else if self.phase_change {
                    write!(f, "|")
                } else if self.resign {
                    write!(f, "%resign")
                } else {
                    write!(f, "PANIC!")
                }
            },
        }
    }
}
*/
impl RpmOpeNote {
    pub fn create_by_address(address:Address) -> RpmOpeNote {
        RpmOpeNote {
            address: Some(address),
            sky_turn: false,
            sky_rotate: false,
            phase_change: false,
            resign: false,
        }
    }

    pub fn turn_over() -> RpmOpeNote {
        RpmOpeNote {
            address: None,
            sky_turn: true,
            sky_rotate: false,
            phase_change: false,
            resign: false,
        }
    }

    pub fn rotate() -> RpmOpeNote {
        RpmOpeNote {
            address: None,
            sky_turn: false,
            sky_rotate: true,
            phase_change: false,
            resign: false,
        }
    }

    pub fn change_phase() -> RpmOpeNote {
        RpmOpeNote {
            address: None,
            sky_turn: false,
            sky_rotate: false,
            phase_change: true,
            resign: false,
        }
    }

    pub fn create_resign() -> RpmOpeNote {
        RpmOpeNote {
            address: None,
            sky_turn: false,
            sky_rotate: false,
            phase_change: false,
            resign: true,
        }
    }

    pub fn is_phase_change(&self) -> bool {
        self.phase_change
    }

    /// # Arguments
    /// 
    /// * `ply` - 手数表示に使う。負数を入れるとタテボウになる。
    pub fn to_sign(&self, board_size:BoardSize, ply:&mut i16) -> String {
        match self.address {
            Some(address) => {
                address.to_physical_sign(board_size)
            },
            None => {
                if self.sky_turn {
                    "+".to_string()
                } else if self.sky_rotate {
                    "-".to_string()
                } else if self.phase_change {
                    // TODO 手数が出てきた方が嬉しいので [2] といった数で挟みたい。
                    if *ply < 0 {
                        "|".to_string()
                    } else {
                        *ply += 1;
                        format!("[{}]", ply)
                    }
                } else if self.resign {
                    "%resign".to_string()
                } else {
                    panic!("Unexpected physical move print.")
                }
            },
        }
    }

    /// ノート１つ読取☆（＾～＾）
    pub fn parse_1note(_comm:&Communication, line:&str, start:&mut usize, board_size:BoardSize) -> Option<RpmOpeNote> {
        let ch1 = line[*start..=*start].chars().nth(0).unwrap();
        match ch1 {
            ' ' => {
                //comm.print(&ch1.to_string());
                *start += 1;
                None
            }
            '0' => {
                // 駒台。
                *start += 1;

                let ch2 = line[*start..=*start].chars().nth(0).unwrap();
                *start += 1;

                //let text15;
                match ch2 {
                    'P' | 'p' | 'ﾅ' => {
                        // 成り駒は、不成駒と同じところに置くので、成りのマークは読み飛ばす。
                        //text15 = line[*start..=*start].chars().nth(0).unwrap().to_string();
                        *start += 1;
                    },
                    _ => {
                        // Ignored.
                        //text15 = "".to_string();
                    },
                };

                // 駒の種類、フェーズ。
                let piece = PhysicalSign::default(ch2.to_string()).to_piece();

                //comm.print(&format!("{}{}{}", ch1, text15, ch2));
                let address = Address::create_by_hand(
                    piece_to_phase(Some(piece)),
                    piece_to_piece_type(piece));
                //comm.println(&format!("address index = {}.", address.get_index()));
                Some(RpmOpeNote::create_by_address(address))
            },
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // セル
                *start += 1;
                let ch2 = line[*start..=*start].chars().nth(0).unwrap();
                *start += 1;
                //comm.print(&format!("{}{}", ch1, ch2));
                let address = Address::from_cell(
                    Cell::from_file_rank(
                        Parser::file_char_to_i8(ch1),
                        Parser::rank_char_to_i8(ch2)), board_size);
                Some(RpmOpeNote::create_by_address(address))
            },
            '+' => {
                // 成り。
                //comm.print(&ch1.to_string());
                *start += 1;
                Some(RpmOpeNote::turn_over())
            },
            '-' => {
                // １８０°回転。
                //comm.print(&ch1.to_string());
                *start += 1;
                Some(RpmOpeNote::rotate())
            },
            '|' => {
                // フェーズ交代。
                //comm.print(&ch1.to_string());
                *start += 1;
                Some(RpmOpeNote::change_phase())
            },
            '[' => {
                // フェーズ交代。 ']' まで読み飛ばす。
                //comm.print(&ch1.to_string());
                *start += 1;
                loop {
                    if line.len() <= *start {
                        break;
                    }
                    
                    let sub_ch = line[*start..=*start].chars().nth(0).unwrap();
                    //comm.print(&sub_ch.to_string());
                    *start += 1;

                    if sub_ch == ']' {
                        break;
                    }
                };
                Some(RpmOpeNote::change_phase())
            },
            _ => {
                let last = line.len();
                panic!("Unexpected line '{}'.", &line[*start..last]);
            }
        }
    }
}
