use address::*;
use board::*;
use logical_move::*;
use logical_record::*;
use physical_move::*;
use physical_record::*;

pub struct RecordConverter {

}
impl RecordConverter {
    /// 変換には、現局面が必要。
    pub fn convert_logical_move(logical_move:LogicalMove, board:&Board, phase:Phase) -> Vec<PhysicalMove> {
        let mut physical_moves = Vec::new();

        let destination_address = Address::create_by_cell(
            logical_move.destination_file,
            logical_move.destination_rank,
            &board.get_board_size()
        );
        
        match logical_move.drop
        {
            Some(drop) => {
                // 駒を打つ動きの場合

                // hand-off
                let hand_off = PhysicalMove::create_by_address(Address::create_hand(Some(phase), drop));
                physical_moves.push(hand_off);

                // hand-on
                let hand_on = PhysicalMove::create_by_address(destination_address);
                physical_moves.push(hand_on);
            },
            None => {
                match board.get_piece_by_address(destination_address.get_index()) {
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
                        let hand_on = PhysicalMove::create_by_address(Address::create_hand(Some(phase), up));
                        physical_moves.push(hand_on);
                    },
                    None => {
                        // 駒を進める動きの場合

                        // board-off
                        let board_off = PhysicalMove::create_by_address(Address::create_by_cell(
                            logical_move.source_file,
                            logical_move.source_rank,
                            &board.get_board_size()
                        ));
                        physical_moves.push(board_off);

                        // board-turn-over
                        if logical_move.promotion {
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
    pub fn convert_logical_record_to_physical_record(
        board:&mut Board,
        logical_record:&LogicalRecord,
        physical_record:&mut PhysicalRecord) {

        for logical_move in &logical_record.items {
            let physical_moves = RecordConverter::convert_logical_move(*logical_move, board, physical_record.get_phase());

            for physical_move in physical_moves {
                physical_record.add(&physical_move);
                if board.touch(&physical_move) {
                    // Phase change.
                    physical_record.add(&PhysicalMove::phase_change());
                }
            }
        }
    }
}