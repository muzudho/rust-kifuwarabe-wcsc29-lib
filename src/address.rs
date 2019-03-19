use board::*;
use position::*;

pub struct Address {
    pub index: usize,
    pub hand: Option<Phase>,
}
impl Address {
    pub fn create_by_index(index_num:usize) -> Address {
        Address {
            index: index_num,
            hand: None,
        }
    }

    pub fn create_by_cell(file_num:i8, rank_num:i8, board:&Board) -> Address {
        Address {
            index: board.file_rank_to_cell(file_num, rank_num),
            hand: None,
        }
    }

    pub fn create_hand(hand_phase:&Phase) -> Address {
        Address {
            index: 0 as usize,
            hand: Some(*hand_phase),
        }
    }
}