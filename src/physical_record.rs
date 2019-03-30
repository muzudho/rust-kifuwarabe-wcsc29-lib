use position::*;
use physical_move::*;

#[derive(Default)]
pub struct PhysicalRecord {
    items: Vec<PhysicalMove>,
    cursor: i16,
    ply: i16,
}
impl PhysicalRecord {
    pub fn default() -> PhysicalRecord {
        PhysicalRecord {
            items: Vec::new(),
            cursor: -1,
            // 開始時点で、1手目進行中 として扱います。
            ply: 1,
        }
    }

    fn up_count(&mut self, pmove:&PhysicalMove) {
        self.cursor += 1;
        if pmove.is_phase_change() {
            self.ply += 1;
        }
    }

    fn up_count_retry(&mut self) {
        self.cursor += 1;
        let pmove = self.items[self.cursor as usize];
        if pmove.is_phase_change() {
            self.ply += 1;
        }
    }

    fn down_count(&mut self, pmove:&PhysicalMove) {
        self.cursor -= 1;
        if pmove.is_phase_change() {
            self.ply -= 1;
        }
    }

    fn down_count_retry(&mut self) {
        // フェーズ切り替えがあったら、手目を１つ減らす。
        let pmove = self.items[self.cursor as usize];
        if pmove.is_phase_change() {
            self.ply -= 1;
        }

        self.cursor -= 1;
        if self.cursor < 0 {
            // 何も記録していない内部状態に相当。
            return;
        }
    }

    pub fn add(&mut self, pmove:&PhysicalMove) {
        // 追加しようとしたとき、すでに後ろの要素がある場合は、後ろの要素を削除する。
        if (self.cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.cursor, self.items.len());
            self.items.truncate((self.cursor + 1) as usize)
        };

        if self.items.len() == (self.cursor + 1) as usize {
            // 追加。
            self.items.push(*pmove);
            self.up_count(pmove);
        } else {
            panic!("Unexpected add: cursor: {}, len: {}.", self.cursor, self.items.len());
        }
    }

    pub fn pop(&mut self) -> Option<PhysicalMove> {
        if self.items.is_empty() {
            None
        } else {
            let last = self.items.len()-1;
            let deleted_pmove = self.items[last];
            self.items.remove(last);
            self.down_count(&deleted_pmove);
            Some(deleted_pmove)
        }
    }

    pub fn get_cursor(&self) -> i16 {
        self.cursor
    }

    pub fn get_ply(&self) -> i16 {
        self.ply
    }

    /// カーソルが指している要素を返す。
    pub fn get_current(&self) -> Option<PhysicalMove> {
        if self.cursor == -1 { // self.items.is_empty()
            None
        } else {
            Some(self.items[self.cursor as usize])
        }
    }

    /// カーソルだけ進める。
    pub fn forward(&mut self) -> bool {
        if self.items.len() as i16 <= (self.cursor + 1) {
            // 進めない。
            return false
        } else {
            self.up_count_retry();
            return true
        }
    }

    /// カーソルだけ戻す。
    pub fn back(&mut self) {
        if self.cursor < 0 {
            // 戻れない。
            return
        };

        self.down_count_retry();
    }

    pub fn to_sign(&self, board_size:BoardSize) -> String {
        let mut sign = "".to_string();
        let mut ply = 1;
        for pmove in &self.items {
            sign = format!("{} {}", sign, pmove.to_sign(board_size, &mut ply));
        }
        sign
    }
}