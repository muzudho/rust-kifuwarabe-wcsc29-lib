use piece_etc::*;
use position::*;
use rpm_conv::rpm_operation_note::*;
use usi_conv::usi_move::*;

/// １手分。
pub struct RpmMove {
    pub operation_notes: Vec<RpmOpeNote>,
    pub piece_number_notes: Vec<i8>,
}
impl RpmMove {
    pub fn new() -> RpmMove {
        RpmMove {
            operation_notes: Vec::new(),
            piece_number_notes: Vec::new(),
        }
    }

    pub fn len_note(&self) -> usize {
        self.operation_notes.len()
    }

    pub fn is_empty_note(&self) -> bool {
        self.operation_notes.is_empty()
    }

    pub fn to_usi_sign(&self, board_size:BoardSize) -> UsiMove {
        let mut i_location = 0;

        let mut sfile = -1;
        let mut srank = -1;
        let mut dfile = -1;
        let mut drank = -1;
        let mut promotion = false;
        let mut drop = None;
        for note in &self.operation_notes {
            if let Some(address) = note.address {

                if let Some(piece) = address.get_hand_piece() {
                    if i_location == 0 {
                        drop = Some(piece_to_piece_type(piece));
                        i_location += 1;
                    }
                } else {
                    match i_location {
                        0 => {
                            let (sf, sr) = board_size.address_to_file_rank(address.get_index());
                            sfile = sf;
                            srank = sr;
                            i_location += 1;
                        },
                        1 => {
                            let (df, dr) = board_size.address_to_file_rank(address.get_index());
                            dfile = df;
                            drank = dr;
                            i_location += 1;
                        },
                        _ => {

                        },
                    }
                }

            } else if note.sky_turn {
                // +
                promotion = true;
            } else if note.sky_rotate {
                // -
            } else {

            }
        }

        UsiMove::create(
            sfile,
            srank,
            dfile,
            drank,
            promotion,
            drop)
    }

    pub fn to_operation_string(&self, board_size:BoardSize) -> String {
        let mut text = String::new();

        for i in 0..self.len_note() {
            let mut ply = -1;
            text = format!("{} {}", text, &self.operation_notes[i].to_sign(board_size, &mut ply));
        }

        text
    }

    pub fn to_identify_string(&self) -> String {
        let mut text = String::new();

        for i in 0..self.len_note() {
            text = format!("{} {}", text, &self.piece_number_notes[i]);
        }

        text
    }
}