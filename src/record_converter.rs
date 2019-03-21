use address::*;
use logical_move::*;
use logical_record::*;
use physical_move::*;
use physical_record::*;
use position::*;

pub struct RecordConverter {

}
impl RecordConverter {
    /// 変換には、現局面が必要。
    pub fn ConvertLogicalMove(logical_move:&LogicalMove, position:&mut Position) -> Vec<PhysicalMove> {
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
                &position.board
            ));
            
            if logical_move.promotion {
                let board_turn = PhysicalMove::create_turn();
            }

            let board_on = PhysicalMove::create_by_address(Address::create_by_cell(
                logical_move.destination_file,
                logical_move.destination_rank,
                &position.board
            ));
        }

        result
    }

    /// 変換には、初期局面が必要。
    pub fn ConvertLogicalToPhysical(logical_record:&LogicalRecord, position:&mut Position) -> PhysicalRecord {
        let mut physical_record = PhysicalRecord::new();

        for logical_move in &logical_record.items {
            let physical_moves = RecordConverter::ConvertLogicalMove(logical_move, position);

            for physical_move in physical_moves {
                physical_record.add(&physical_move);
                position.board.touch(&physical_move);
            }
        }

        physical_record
    }
}