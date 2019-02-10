use position::Position;

pub struct Thought {

}
impl Thought {
    pub fn new() -> Thought {
        Thought {

        }
    }

    pub fn think(self, position:&Position) -> String {
        return "bestmove 2g2f".to_string();
    }
}