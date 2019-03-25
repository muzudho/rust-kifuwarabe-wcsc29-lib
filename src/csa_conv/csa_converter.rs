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
    pub fn convert_move(
        comm:&Communication,
        cmove:&CsaMove,
        position:&Position,
        ply:i16) -> Vec<PhysicalMove> {
        let mut p_moves = Vec::new();

        let destination_address = Address::create_by_cell(
            cmove.destination_file,
            cmove.destination_rank,
            position.get_board_size()
        );
        
        if let Some(drop) = cmove.get_drop() {
            // 駒を打つ動きの場合

            // hand-off
            let hand_off = PhysicalMove::create_by_address(Address::create_by_hand(Some(position.get_phase()), drop));
            p_moves.push(hand_off);

            // hand-on
            let hand_on = PhysicalMove::create_by_address(destination_address);
            p_moves.push(hand_on);
        } else {
            // 駒を進める動きの場合
            if let Some(capture_piece) = position.get_piece_by_address(destination_address.get_index()) {
                // 駒を取る動きが入る場合
                comm.println(&format!("[{}] 駒を取る動きが入る場合 {}", ply, piece_to_sign(Some(capture_piece))));

                // hand-off
                let hand_off = PhysicalMove::create_by_address(destination_address);
                p_moves.push(hand_off);

                // hand-turn
                if is_promotion_piece(Some(capture_piece)) {
                    let hand_turn = PhysicalMove::turn_over();
                    p_moves.push(hand_turn);
                }

                // hand-rotate
                let hand_rotate = PhysicalMove::rotate();
                p_moves.push(hand_rotate);

                // hand-on
                let up = piece_to_piece_type(capture_piece);
                let hand_on = PhysicalMove::create_by_address(Address::create_by_hand(Some(position.get_phase()), up));
                p_moves.push(hand_on);
            } else {
                comm.println(&format!("[{}] 駒は取らない", ply));
            }

            // board-off
            let board_off = PhysicalMove::create_by_address(Address::create_by_cell(
                cmove.source_file,
                cmove.source_rank,
                position.get_board_size()
            ));
            p_moves.push(board_off);

            // board-turn-over
            if cmove.promotion {
                let board_turn = PhysicalMove::turn_over();
                p_moves.push(board_turn);
            }

            // board-on
            let board_on = PhysicalMove::create_by_address(destination_address);
            p_moves.push(board_on);
        };

        // change-phase
        let change_phase = PhysicalMove::change_phase();
        p_moves.push(change_phase);

        p_moves
    }

    /// 変換には、初期局面が必要。
    pub fn convert_record(
        comm:&Communication,
        position:&mut Position,
        c_record:&CsaRecord,
        physical_record:&mut PhysicalRecord) {

        // TODO とりあえず平手初期局面だけ対応。
        position.reset_startpos();
        CommonOperation::bo(comm, physical_record, position);

        let mut ply = 1;
        for cmove in &c_record.items {
            let p_moves = CsaConverter::convert_move(
                comm,
                cmove,
                position,
                ply);

            for pmove in p_moves {
                CommonOperation::go(comm, physical_record, &pmove, position);
                CommonOperation::bo(comm, physical_record, position);
            }

            ply += 1;
        }
    }
}