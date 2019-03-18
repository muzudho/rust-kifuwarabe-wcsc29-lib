use position::*;

pub struct Address {
    pub file: i8,
    pub rank: i8,
    pub hand: Option<Phase>,
}
impl Address {
    pub fn create_on_board(file_num:i8, rank_num:i8) -> Address {
        Address {
            file: file_num,
            rank: rank_num,
            hand: None,
        }
    }

    pub fn create_hand(hand_phase:&Phase) -> Address {
        Address {
            file: 0,
            rank: 0,
            hand: Some(*hand_phase),
        }
    }
}