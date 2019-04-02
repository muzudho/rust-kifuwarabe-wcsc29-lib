use address::*;
use communication::*;
use common_operation::*;
use csa_conv::csa_move::*;
use csa_conv::csa_record::*;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_operation_note::*;
use rpm_conv::rpm_operation_track::*;

pub struct CsaConverter {

}
impl CsaConverter {
    /// 変換には、現局面が必要。
    pub fn convert_move(
        comm:&Communication,
        cmove:&CsaMove,
        position:&Position,
        ply:i16) -> Vec<RpmNote> {
        let mut p_moves = Vec::new();

        let destination_address = Address::create_by_cell(
            cmove.destination_file,
            cmove.destination_rank,
            position.get_board_size()
        );
        
        if let Some(drop) = cmove.get_drop() {
            // 駒を打つ動きの場合

            // hand-off
            let hand_off = RpmNote::create_by_address(Address::create_by_hand(Some(position.get_phase()), drop));
            p_moves.push(hand_off);

            // hand-on
            let hand_on = RpmNote::create_by_address(destination_address);
            p_moves.push(hand_on);
        } else {
            // 駒を進める動きの場合
            if let Some(capture_id_piece) = position.get_id_piece_by_address(destination_address.get_index()) {
                // 駒を取る動きが入る場合
                comm.println(&format!("[{}] 駒を取る動きが入る場合 {}", ply, capture_id_piece.to_extended_usi_text()));

                // hand-off
                let hand_off = RpmNote::create_by_address(destination_address);
                p_moves.push(hand_off);

                // hand-turn
                if capture_id_piece.is_promoted() {
                    let hand_turn = RpmNote::turn_over();
                    p_moves.push(hand_turn);
                }

                // hand-rotate
                let hand_rotate = RpmNote::rotate();
                p_moves.push(hand_rotate);

                // hand-on
                let up = capture_id_piece.get_type();
                let hand_on = RpmNote::create_by_address(Address::create_by_hand(Some(position.get_phase()), up));
                p_moves.push(hand_on);
            } else {
                comm.println(&format!("[{}] 駒は取らない", ply));
            }

            // board-off
            let board_off = RpmNote::create_by_address(Address::create_by_cell(
                cmove.source_file,
                cmove.source_rank,
                position.get_board_size()
            ));
            p_moves.push(board_off);

            // board-turn-over
            // 盤上にある駒が不成で、指し手の駒種類が成り駒なら、今、成った。
            let pre_promoted = if let Some(id_piece) = position.get_id_piece(cmove.source_file, cmove.source_rank) {
                id_piece.is_promoted()
            } else {
                false
            };
            let cur_promoted = is_promoted_piece_type(cmove.koma);
            if !pre_promoted && cur_promoted {
                let board_turn = RpmNote::turn_over();
                p_moves.push(board_turn);
            }

            // board-on
            let board_on = RpmNote::create_by_address(destination_address);
            p_moves.push(board_on);
        };

        // change-phase
        let change_phase = RpmNote::change_phase();
        p_moves.push(change_phase);

        p_moves
    }

    /// 変換には、初期局面が必要。
    pub fn convert_record(
        comm:&Communication,
        position:&mut Position,
        c_record:&CsaRecord,
        rpm_o_track:&mut RpmOTrack) {

        // TODO とりあえず平手初期局面だけ対応。
        position.reset_startpos();
        CommonOperation::bo(comm, rpm_o_track, position);

        let mut ply = 1;
        for cmove in &c_record.items {
            let p_moves = CsaConverter::convert_move(
                comm,
                cmove,
                position,
                ply);

            for rpm_note in p_moves {
                CommonOperation::go(comm, rpm_o_track, &rpm_note, position);
                CommonOperation::bo(comm, rpm_o_track, position);
            }

            ply += 1;
        }
    }
}