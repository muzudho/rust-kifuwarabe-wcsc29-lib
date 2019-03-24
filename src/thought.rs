use position::*;
use usi_conv::usi_move::*;
use physical_record::*;
use piece_etc::*;

#[derive(Default)]
pub struct Thought {

}
impl Thought {
    pub fn new() -> Thought {
        Thought {

        }
    }

    pub fn get_best_move(self, position:&Position, physical_record:&mut PhysicalRecord) -> UsiMove {
        // use position::Piece::*;

        // position.show_position();
        // println!("info Current phase: `{}`.", phase_to_sign(&logical_record.get_current_phase()));

        // 盤上の自分の駒を１つ選ぶ。
        let mut piece = None;
        let mut src_file = 0;
        let mut src_rank = 0;
        'search: for rank in 1..=9 {
            // println!("info Rank: `{}`.", rank);
            for file in 1..=9 {
                piece = position.get_piece(file, rank);
                let phase = piece_to_phase(piece);
                if phase.is_some() && phase.unwrap() == position.get_phase() {
                    // println!("info Find: {}-{} {}.{}.", file, rank, phase_to_sign(phase), piece_to_sign(&piece));
                    // TODO 自分の駒に限り。
                    src_file = file;
                    src_rank = rank;
                    break 'search;
                }
            }
        }
        // println!("info Src: {}-{} {}.{}", src_file, src_rank, phase_to_sign(&piece_to_phase(&piece)), piece_to_sign(&piece));

        // その駒の動き方から、行き先の升。
        let dst_rank = if 1 < src_rank {
            src_rank - 1
        } else {
            src_rank
        };

        UsiMove {
            source_file: src_file,
            source_rank: src_rank,
            destination_file: src_file,
            destination_rank: dst_rank,
            promotion: false,
            drop: None,
        }
    }
}