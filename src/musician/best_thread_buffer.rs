use musician::best_move::*;
use musician::best_thread::BestThread;
use studio::application::Application;
use studio::board_size::BoardSize;

/// 手筋１個分。読み筋。
#[derive(Default)]
pub struct BestThreadBuffer {
    pub moves: Vec<BestMove>,
}
impl BestThreadBuffer {
    pub fn new() -> Self {
        BestThreadBuffer { moves: Vec::new() }
    }

    /// 現在の内容を破棄し、空っぽにするぜ☆（＾～＾）
    pub fn clear(&mut self) {
        self.moves.clear();
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// 指し手を追加。
    pub fn push_move(&mut self, bmove: BestMove) {
        self.moves.push(bmove);
    }

    pub fn to_object(&self) -> BestThread {
        // ベクターってこれでコピーできるの☆（＾～＾）？
        BestThread::from_buffer(self.moves.to_vec())
    }

    pub fn to_human_presentable(&self, board_size: BoardSize, app: &Application) -> String {
        let mut text = String::new();

        for bmove in &self.moves {
            text = format!("{} {}", text, bmove.to_human_presentable(board_size, &app))
        }

        text.to_string()
    }
}
