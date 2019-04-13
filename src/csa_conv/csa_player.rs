use address::*;
use communication::*;
use common_operation::*;
use csa_conv::csa_move::*;
use csa_conv::csa_record::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_operation_note::*;
use rpm_conv::rpm_record::*;

pub struct CsaPlayer {

}
impl CsaPlayer {
    /// 変換には、現局面が必要。
    pub fn convert_move(
        _comm:&Communication,
        cmove:&CsaMove,
        position:&Position,
        _ply:i16) -> Vec<RpmOpeNote> {
        let mut p_moves = Vec::new();

        // 盤上の駒の番地。
        let destination_address = Address::from_cell(
            cmove.destination,
            position.get_board_size()
        );
        
        if let Some(drop) = cmove.get_drop() {
            // 駒を打つ動きの場合

            // hand-off
            let hand_off = RpmOpeNote::create_by_address(Address::create_by_hand(Some(position.get_phase()), drop));
            p_moves.push(hand_off);

            // hand-on
            let hand_on = RpmOpeNote::create_by_address(destination_address);
            p_moves.push(hand_on);
        } else {
            // 駒を進める動きの場合
            if let Some(capture_id_piece) = position.get_id_piece_by_address(destination_address.get_index()) {
                // 駒を取る動きが入る場合

                // hand-off
                let hand_off = RpmOpeNote::create_by_address(destination_address);
                p_moves.push(hand_off);

                // hand-turn
                if capture_id_piece.is_promoted() {
                    let hand_turn = RpmOpeNote::turn_over();
                    p_moves.push(hand_turn);
                }

                // hand-rotate
                let hand_rotate = RpmOpeNote::rotate();
                p_moves.push(hand_rotate);

                // hand-on
                let up = capture_id_piece.get_type();
                let hand_on = RpmOpeNote::create_by_address(Address::create_by_hand(Some(position.get_phase()), up));
                p_moves.push(hand_on);
            }

            // board-off
            // 盤上の駒の番地。
            let board_off = RpmOpeNote::create_by_address(Address::from_cell(
                cmove.source.unwrap(),
                position.get_board_size()
            ));
            p_moves.push(board_off);

            // board-turn-over
            // 盤上にある駒が不成で、指し手の駒種類が成り駒なら、今、成った。
            let pre_promoted = if let Some(id_piece) = position.get_id_piece(cmove.source.unwrap()) {
                id_piece.is_promoted()
            } else {
                false
            };
            let cur_promoted = is_promoted_piece_type(cmove.koma);
            if !pre_promoted && cur_promoted {
                let board_turn = RpmOpeNote::turn_over();
                p_moves.push(board_turn);
            }

            // board-on
            let board_on = RpmOpeNote::create_by_address(destination_address);
            p_moves.push(board_on);
        };

        // change-phase
        let change_phase = RpmOpeNote::change_phase();
        p_moves.push(change_phase);

        p_moves
    }

    /// 変換には、初期局面が必要。
    pub fn play_out_record(
        comm:&Communication,
        position:&mut Position,
        c_record:&CsaRecord,
        rpm_record:&mut RpmRecord) {

        // TODO とりあえず平手初期局面だけ対応。
        position.reset_startpos();

        let mut ply = 1;
        for cmove in &c_record.items {
            let p_moves = CsaPlayer::convert_move(
                comm,
                cmove,
                position,
                ply);

            for rpm_note in p_moves {
                CommonOperation::touch_beautiful_world(comm, rpm_record, &rpm_note, position);
            }

            ply += 1;
        }
    }
}