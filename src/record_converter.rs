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
    pub fn convert_logical_move(logical_move:&LogicalMove, board:&mut Board) -> Vec<PhysicalMove> {
        let result = Vec::new();

        match logical_move.drop
        {
            Some(drop) => {
                // 駒を打つ動きの場合
            },
            None => {
                // 駒を取る動きの場合

                // hand-off
                // logical_move.
            },
        }

        {
            let board_off = PhysicalMove::create_by_address(Address::create_by_cell(
                logical_move.source_file,
                logical_move.source_rank,
                &board.get_board_size()
            ));
            
            if logical_move.promotion {
                let board_turn = PhysicalMove::create_turn();
            }

            let board_on = PhysicalMove::create_by_address(Address::create_by_cell(
                logical_move.destination_file,
                logical_move.destination_rank,
                &board.get_board_size()
            ));
        }

        result
    }

    /// 変換には、初期局面が必要。
    pub fn convert_logical_to_physical(
        board:&mut Board,
        logical_record:&LogicalRecord,
        physical_record:&mut PhysicalRecord) {

        for logical_move in &logical_record.items {
            let physical_moves = RecordConverter::convert_logical_move(logical_move, board);

            for physical_move in physical_moves {
                physical_record.add(&physical_move);
                board.touch(&physical_move);
            }
        }
    }
}