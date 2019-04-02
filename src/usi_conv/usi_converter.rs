use address::*;
use communication::*;
use common_operation::*;
use usi_conv::usi_move::*;
use usi_conv::usi_record::*;
use physical_move::*;
use physical_record::*;
// use piece_etc::*;
use position::*;

pub struct UsiConverter {

}
impl UsiConverter {
    pub fn convert_move(umove:UsiMove, position:&Position) -> Vec<PhysicalMove> {
        let mut pmoves = Vec::new();

        if umove.is_resign() {
            pmoves.push(PhysicalMove::create_resign());
            return pmoves;
        }

        let destination_address = Address::create_by_cell(
            umove.destination_file,
            umove.destination_rank,
            position.get_board_size()
        );
        
        match umove.get_drop()
        {
            Some(drop) => {
                // 駒を打つ動きの場合

                // hand-off
                let hand_off = PhysicalMove::create_by_address(Address::create_by_hand(Some(position.get_phase()), drop));
                pmoves.push(hand_off);

                // hand-on
                let hand_on = PhysicalMove::create_by_address(destination_address);
                pmoves.push(hand_on);
            },
            None => {
                // 駒を進める動きの場合
                if let Some(id_piece) = position.get_id_piece_by_address(destination_address.get_index()) {
                    // 駒を取る動きが入る場合

                    // hand-off
                    let hand_off = PhysicalMove::create_by_address(destination_address);
                    pmoves.push(hand_off);

                    // hand-turn
                    if id_piece.is_promoted() {
                        let hand_turn = PhysicalMove::turn_over();
                        pmoves.push(hand_turn);
                    }

                    // hand-rotate
                    let hand_rotate = PhysicalMove::rotate();
                    pmoves.push(hand_rotate);

                    // hand-on
                    let up = id_piece.get_type();
                    let hand_on = PhysicalMove::create_by_address(Address::create_by_hand(Some(position.get_phase()), up));
                    pmoves.push(hand_on);
                }

                // board-off
                let board_off = PhysicalMove::create_by_address(Address::create_by_cell(
                    umove.source_file,
                    umove.source_rank,
                    position.get_board_size()
                ));
                pmoves.push(board_off);

                // board-turn-over
                if umove.promotion {
                    let board_turn = PhysicalMove::turn_over();
                    pmoves.push(board_turn);
                }

                // board-on
                let board_on = PhysicalMove::create_by_address(destination_address);
                pmoves.push(board_on);
            },
        }

        // change-phase
        let change_phase = PhysicalMove::change_phase();
        pmoves.push(change_phase);

        pmoves
    }

    /// # Arguments
    /// 
    /// * 'position' - USIレコードと 初期局面を合わせてください。
    /// 
    pub fn convert_record(
        comm:&Communication,
        position:&mut Position,
        urecord:&UsiRecord,
        precord:&mut PhysicalRecord) {

        // 局面を動かしながら変換していく。
        let mut ply = 0;
        for umove in &urecord.items {
            let pmoves = UsiConverter::convert_move(*umove, position);
            //comm.println(&format!("Pmoves len: {}.", pmoves.len()));

            for pmove in pmoves {
                //comm.println(&format!("Pmove: '{}'.", pmove.to_sign(position.get_board_size(), &mut ply)));
                CommonOperation::go(comm, precord, &pmove, position);
            }
        }
    }
}