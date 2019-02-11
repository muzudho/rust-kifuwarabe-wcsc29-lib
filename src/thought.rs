use position::*;
use moves::*;

pub struct Thought {

}
impl Thought {
    pub fn new() -> Thought {
        Thought {

        }
    }

    pub fn think(self, position:&mut Position) -> String {
        use position::Piece::*;

        position.show_board();

        let piece = position.get_piece(2, 7);
        println!("info Piece: `{}`.", piece_to_sign(&piece));

        // 後手の動き。
        if piece == Empty {
            "bestmove 2f2e".to_string()
        } else if piece == P1 {
            "bestmove 2g2f".to_string()
        } else {
            // 盤上の自分の駒を１つ選ぶ。
            let mut piece = Empty;
            let mut srcFile = 0;
            let mut srcRank = 0;
            'search: for rank in 1..=9 {
                for file in 1..=9 {
                    piece = position.get_piece(file, rank);
                    if piece != Empty {
                        srcFile = file;
                        srcRank = rank;
                        break 'search;
                    }
                }
            }

            let mov = Move {
                sourceFile: srcFile,
                sourceRank: srcRank,
                destinationFile: srcFile,
                destinationRank: srcRank-1,
                promotion: false,
                drop: PieceType::Empty,
            };
            mov.to_sign()
        }
    }
}