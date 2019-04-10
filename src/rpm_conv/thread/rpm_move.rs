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

    /*
    pub fn to_usi_sign(&self) -> String {
        UsiMove::create(
            src_file:i8,
            src_rank:i8,
            dst_file:i8,
            dst_rank:i8,
            pro:bool,
            dro:Option<PieceType>);
    }
     */

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