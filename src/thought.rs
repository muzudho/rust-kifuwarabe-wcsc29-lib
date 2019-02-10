use position::Position;

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

        if piece == Empty {
            "bestmove 2f2e".to_string()
        } else {
            "bestmove 2g2f".to_string()
        }
    }
}