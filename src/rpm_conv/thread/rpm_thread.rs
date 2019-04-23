use board_size::BoardSize;
use rpm_conv::thread::rpm_move::*;

/// シーケンスな手筋１個分。
#[derive(Default)]
pub struct RpmThread {
    pub moves: Vec<RpmMove>,
}
impl RpmThread {
    pub fn new() -> Self {
        RpmThread { moves: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    pub fn push_move(&mut self, rmove: RpmMove) {
        self.moves.push(rmove);
    }

    /// Human presentable.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        let mut text = String::new();

        for rmove in &self.moves {
            text = format!("{} {}", text, rmove.to_human_presentable(board_size))
        }

        format!("{}", text)
    }
}
