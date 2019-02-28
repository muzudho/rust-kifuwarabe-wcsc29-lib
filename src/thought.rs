use position::*;
use record::*;

pub struct Thought {

}
impl Thought {
    pub fn new() -> Thought {
        Thought {

        }
    }

    pub fn get_best_move(self, position:&mut Position) -> Move {
        use position::Piece::*;

        // position.show_board();
        // println!("info Current player: `{}`.", player_to_sign(&position.record.get_current_player()));

        // 盤上の自分の駒を１つ選ぶ。
        let mut piece = None;
        let mut srcFile = 0;
        let mut srcRank = 0;
        'search: for rank in 1..=9 {
            // println!("info Rank: `{}`.", rank);
            for file in 1..=9 {
                piece = position.get_piece(file, rank);
                let player = piece_to_player(&piece);
                if player.is_some() {
                    if player.unwrap() == position.record.get_current_player() {
                        // println!("info Find: {}-{} {}.{}.", file, rank, player_to_sign(player), piece_to_sign(&piece));
                        // TODO 自分の駒に限り。
                        srcFile = file;
                        srcRank = rank;
                        break 'search;
                    }
                }
            }
        }
        // println!("info Src: {}-{} {}.{}", srcFile, srcRank, player_to_sign(&piece_to_player(&piece)), piece_to_sign(&piece));

        // その駒の動き方から、行き先の升。
        let dstRank = if 1 < srcRank {
            srcRank - 1
        } else {
            srcRank
        };

        Move {
            sourceFile: srcFile,
            sourceRank: srcRank,
            destinationFile: srcFile,
            destinationRank: dstRank,
            promotion: false,
            drop: PieceType::Empty,
        }
    }
}