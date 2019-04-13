use address::*;
use communication::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_note::*;
use rpm_conv::thread::rpm_operation_note::*;
use rpm_model::rpm_book_file::*;
use usi_conv::usi_move::*;

/// １手分。
#[derive(Debug)]
pub struct RpmMove {
    pub notes: Vec<RpmNote>,
}
/*
impl fmt::Display for RpmMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();

        let size = self.operation_notes.len();
        for i in 0..size {
            text = format!("{} ({} {})", text, self.operation_notes[i], self.piece_number_notes[i]).to_string()
        }

        write!(f, "{}", text)
    }
}
*/
impl RpmMove {
    pub fn new() -> RpmMove {
        RpmMove {
            notes: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// この指し手が、どの駒が動いたものによるものなのか、またどこにあった駒なのかを返します。
    pub fn to_first_touch_piece_id(&self, board_size:BoardSize) -> (PieceIdentify, Address) {
        // とりあえず USI move に変換するついでに、欲しい情報を得る。
        let (_umove, first_touch_id, first_touch_addr) = self.to_usi_move(board_size);

        (first_touch_id, first_touch_addr)
    }

    /// # Returns
    /// 
    /// Usi move,
    /// どの駒を動かした一手か,
    /// どこの駒を動かした一手か,
    pub fn to_usi_move(&self, board_size:BoardSize) -> (UsiMove, PieceIdentify, Address) {
        let mut i_token = 0;

        let mut src_opt = None;
        let mut dst_opt = None;
        let mut promotion = false;
        let mut drop_opt = None;
        let mut first_touch_id = None;
        let mut first_touch_address = None;
        for note in &self.notes {
            if let Some(address) = note.get_ope().address {
                if let Some(piece) = address.get_hand_piece() {
                    // 駒台
                    if i_token == 0 {
                        drop_opt = Some(piece_to_piece_type(piece));
                        first_touch_id = Some(note.get_id());
                        first_touch_address = Some(address);
                        i_token += 1;
                    }
                } else {
                    // 盤上
                    match i_token {
                        0 => {
                            src_opt = Some(board_size.address_to_cell(address.get_index()));
                            first_touch_id = Some(note.get_id());
                            first_touch_address = Some(address);
                            i_token += 1;
                        },
                        1 => {
                            dst_opt = Some(board_size.address_to_cell(address.get_index()));
                            // ２つ目に出てくる場合、１つ目は取った相手の駒の動き。
                            first_touch_id = Some(note.get_id());
                            first_touch_address = Some(address);
                            i_token += 1;
                        },
                        _ => {},
                    }
                }

            } else if note.get_ope().sky_turn {
                // +
                promotion = true;
            } else if note.get_ope().sky_rotate {
                // -
            }
        }

        let umove = if let Some(drop) = drop_opt {
            UsiMove::create_drop(
                dst_opt.unwrap(),
                drop,
                board_size)
        } else {
            UsiMove::create_walk(
                src_opt.unwrap(),
                dst_opt.unwrap(),
                promotion,
                board_size)
        };

        // USIの指し手が作れれば、 first touch が分からないことはないはず。
        (umove, PieceIdentify::from_number(first_touch_id.unwrap()).unwrap(), first_touch_address.unwrap())
    }

    pub fn to_operation_string(&self, board_size:BoardSize) -> String {
        let mut text = String::new();

        for i in 0..self.len() {
            let mut ply = -1;
            text = format!("{} {}", text, &self.notes[i].get_ope().to_sign(board_size, &mut ply));
        }

        text
    }

    pub fn to_identify_string(&self) -> String {
        let mut text = String::new();

        for i in 0..self.len() {
            text = format!("{} {}", text, &self.notes[i].get_id());
        }

        text
    }

    pub fn parse_1move(comm:&Communication, record_for_json:&RpmRecordForJson, note_idx:usize, board_size:BoardSize) -> Option<RpmMove> {
        let mut rmove = RpmMove::new();
        let size = record_for_json.body.operation.len();

        // TODO とりあえず　次のターンチェンジまで読み進める。
        'j_loop: for j in note_idx..size {
            let j_ope_token = &record_for_json.body.operation[j];

            let j_ope_note_opt;
            {
                let mut start = 0;
                j_ope_note_opt = RpmOpeNote::parse_1note(&comm, &j_ope_token, &mut start, board_size);
            }

            if let Some(j_ope_note) = j_ope_note_opt {
                if j_ope_note.is_phase_change() {
                    break 'j_loop;
                } else {
                    let j_num = &record_for_json.body.piece_number[j];
                    rmove.notes.push(RpmNote::create(j_ope_note, *j_num));
                }
            }
        }

        if rmove.is_empty() {
            None
        } else {
            Some(rmove)
        }
    }
}