use application::Application;
use board_size::BoardSize;
use object_rpm::cassette_tape_box::CassetteTapeBox;
use object_rpm::shogi_move::*;

/// シーケンスな手筋１個分。
#[derive(Default)]
pub struct ShogiThread {
    pub moves: Vec<ShogiMove>,
}
impl ShogiThread {
    pub fn new() -> Self {
        ShogiThread { moves: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    pub fn push_move(&mut self, rmove: ShogiMove) {
        self.moves.push(rmove);
    }

    /// Human presentable.
    pub fn to_human_presentable(
        &self,
        tape_box: &CassetteTapeBox,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        let mut text = String::new();

        for rmove in &self.moves {
            text = format!(
                "{} {}",
                text,
                rmove.to_human_presentable(&tape_box, board_size, &app)
            )
        }

        text.to_string()
    }
}
