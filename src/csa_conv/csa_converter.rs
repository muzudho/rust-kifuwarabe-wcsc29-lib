use address::*;
use communication::*;
use common_operation::*;
use csa_conv::csa_move::*;
use csa_conv::csa_record::*;
use physical_move::*;
use physical_record::*;
use piece_etc::*;
use position::*;

pub struct CsaConverter {

}
impl CsaConverter {
    /// 変換には、現局面が必要。
    pub fn convert_move(cmove:&CsaMove, position:&Position) -> Vec<PhysicalMove> {
        let mut physical_moves = Vec::new();

        let destination_address = Address::create_by_cell(
            cmove.destination_file,
            cmove.destination_rank,
            position.get_board_size()
        );
        
        match cmove.get_drop()
        {
            Some(drop) => {
                // 駒を打つ動きの場合

                // hand-off
                let hand_off = PhysicalMove::create_by_address(Address::create_by_hand(Some(position.get_phase()), drop));
                physical_moves.push(hand_off);

                // hand-on
                let hand_on = PhysicalMove::create_by_address(destination_address);
                physical_moves.push(hand_on);
            },
            None => {
                match position.get_piece_by_address(destination_address.get_index()) {
                    Some(piece) => {
                        // 駒を取る動きの場合

                        // hand-off
                        let hand_off = PhysicalMove::create_by_address(destination_address);
                        physical_moves.push(hand_off);

                        // hand-turn
                        if is_promotion_piece(Some(piece)) {
                            let hand_turn = PhysicalMove::turn_over();
                            physical_moves.push(hand_turn);
                        }

                        // hand-rotate
                        let hand_rotate = PhysicalMove::rotate();
                        physical_moves.push(hand_rotate);

                        // hand-on
                        let up = piece_to_piece_type(piece);
                        let hand_on = PhysicalMove::create_by_address(Address::create_by_hand(Some(position.get_phase()), up));
                        physical_moves.push(hand_on);
                    },
                    None => {
                        // 駒を進める動きの場合

                        // board-off
                        let board_off = PhysicalMove::create_by_address(Address::create_by_cell(
                            cmove.source_file,
                            cmove.source_rank,
                            position.get_board_size()
                        ));
                        physical_moves.push(board_off);

                        // board-turn-over
                        if cmove.promotion {
                            let board_turn = PhysicalMove::turn_over();
                            physical_moves.push(board_turn);
                        }

                        // board-on
                        let board_on = PhysicalMove::create_by_address(destination_address);
                        physical_moves.push(board_on);
                    },
                }
            },
        }

        physical_moves
    }

    /// 変換には、初期局面が必要。
    pub fn convert_record(
        comm:&Communication,
        position:&mut Position,
        c_record:&CsaRecord,
        physical_record:&mut PhysicalRecord) {

        for cmove in &c_record.items {
            let physical_moves = CsaConverter::convert_move(
                cmove,
                position);

            for physical_move in physical_moves {
                CommonOperation::go(comm, physical_record, &physical_move, position);
            }
        }
    }
}