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


        /*
        // 後手の動き。飛車先の歩。
        let piece = position.get_piece(2, 7);
        println!("info Piece: `{}`.", piece_to_sign(&piece));
        if piece == P1 {
            "bestmove 2g2f".to_string()
        } else {
            // 後手の動き。飛車先の歩。
            let piece = position.get_piece(2, 6);
            println!("info Piece: `{}`.", piece_to_sign(&piece));
            if piece == P1 {
                "bestmove 2f2e".to_string()
            } else {
        */
                // 盤上の自分の駒を１つ選ぶ。
                let mut piece = Empty;
                let mut srcFile = 0;
                let mut srcRank = 0;
                'search: for rank in 1..=9 {
                    println!("info Rank: `{}`.", rank);
                    for file in 1..=9 {
                        println!("info File: `{}`.", file);
                        piece = position.get_piece(file, rank);
                        println!("info Piece: `{}`.", piece_to_sign(&piece));
                        println!("info Piece to player: `{}`.", player_to_sign(&piece_to_player(&piece)));
                        if piece != Empty {
                            // TODO 自分の駒に限り。
                            srcFile = file;
                            srcRank = rank;
                            break 'search;
                        }
                    }
                }
                println!("info SrcFile: `{}`.", srcFile);
                println!("info SrcRank: `{}`.", srcRank);
                println!("info Piece: `{}`.", piece_to_sign(&piece));
                println!("info Piece to player: `{}`.", player_to_sign(&piece_to_player(&piece)));

                let dstRank = if 1 < srcRank {
                    srcRank - 1
                } else {
                    srcRank
                };

                let mov = Move {
                    sourceFile: srcFile,
                    sourceRank: srcRank,
                    destinationFile: srcFile,
                    destinationRank: dstRank,
                    promotion: false,
                    drop: PieceType::Empty,
                };

                format!("bestmove {}", mov.to_sign())
        /*
            }
        }
        */
    }
}