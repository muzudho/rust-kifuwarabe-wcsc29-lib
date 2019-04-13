use address::*;
use communication::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_operation_note::*;
use rpm_model::rpm_book_file::*;
use usi_conv::usi_move::*;

/// １手分。
#[derive(Debug)]
pub struct RpmMove {
    pub operation_notes: Vec<RpmOpeNote>,
    pub piece_number_notes: Vec<i8>,
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
            operation_notes: Vec::new(),
            piece_number_notes: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.operation_notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.operation_notes.is_empty()
    }

    /// この指し手が、どの駒が動いたものによるものなのか、またどこにあった駒なのかを返します。
    pub fn to_first_touch_piece_id(&self, board_size:BoardSize) -> (PieceIdentify, Address) {
        // とりあえず USI move に変換。
        let umove = self.to_usi_move(board_size);

        (umove.source)
    }

    pub fn to_usi_move(&self, board_size:BoardSize) -> UsiMove {
        let mut i_location = 0;

        let mut src_opt = None;
        let mut dst_opt = None;
        let mut promotion = false;
        let mut drop_opt = None;
        for note in &self.operation_notes {
            if let Some(address) = note.address {

                if let Some(piece) = address.get_hand_piece() {
                    if i_location == 0 {
                        drop_opt = Some(piece_to_piece_type(piece));
                        i_location += 1;
                    }
                } else {
                    match i_location {
                        0 => {
                            src_opt = Some(board_size.address_to_cell(address.get_index()));
                            i_location += 1;
                        },
                        1 => {
                            dst_opt = Some(board_size.address_to_cell(address.get_index()));
                            i_location += 1;
                        },
                        _ => {},
                    }
                }

            } else if note.sky_turn {
                // +
                promotion = true;
            } else if note.sky_rotate {
                // -
            }
        }

        if let Some(drop) = drop_opt {
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
        }
    }

    pub fn to_operation_string(&self, board_size:BoardSize) -> String {
        let mut text = String::new();

        for i in 0..self.len() {
            let mut ply = -1;
            text = format!("{} {}", text, &self.operation_notes[i].to_sign(board_size, &mut ply));
        }

        text
    }

    pub fn to_identify_string(&self) -> String {
        let mut text = String::new();

        for i in 0..self.len() {
            text = format!("{} {}", text, &self.piece_number_notes[i]);
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
                    rmove.operation_notes.push(j_ope_note);
                    let j_num = &record_for_json.body.piece_number[j];
                    rmove.piece_number_notes.push(*j_num);
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