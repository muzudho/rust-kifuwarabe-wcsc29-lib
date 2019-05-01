use musician::best_move::*;
use studio::application::Application;
use studio::board_size::BoardSize;

/// シーケンスな手筋１個分。読み筋。
#[derive(Default)]
pub struct BestThread {
    pub moves: Vec<BestMove>,
}
impl BestThread {
    pub fn new() -> Self {
        BestThread { moves: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    pub fn push_move(&mut self, bmove: BestMove) {
        self.moves.push(bmove);
    }

    /// Human presentable.
    pub fn to_human_presentable(&self, board_size: BoardSize, app: &Application) -> String {
        let mut text = String::new();

        for bmove in &self.moves {
            text = format!("{} {}", text, bmove.to_human_presentable(board_size, &app))
        }

        text.to_string()
    }
}
