use position::*;
use physical_move::*;

#[derive(Default)]
pub struct PhysicalRecord {
    items: Vec<PhysicalMove>,
    cursor: i16,
}
impl PhysicalRecord {
    pub fn default() -> PhysicalRecord {
        PhysicalRecord {
            items: Vec::new(),
            cursor: -1,
        }
    }

    pub fn add(&mut self, physical_move:&PhysicalMove) {
        // 追加しようとしたとき、すでに後ろの要素がある場合は、後ろの要素を削除する。
        if (self.cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.cursor, self.items.len());
            self.items.truncate((self.cursor + 1) as usize)
        };

        if self.items.len() == (self.cursor + 1) as usize {
            // 追加。
            self.items.push(*physical_move);
            self.cursor += 1;
        } else {
            panic!("Unexpected add: cursor: {}, len: {}.", self.cursor, self.items.len());
        }
    }

    pub fn pop(&mut self) -> Option<PhysicalMove> {
        if self.items.is_empty() {
            None
        } else {
            let last = self.items.len()-1;
            let deleted = self.items[last];
            self.items.remove(last);
            self.cursor -= 1;
            Some(deleted)
        }
    }

    /// カーソルが指している要素を返す。
    pub fn get_current(&self) -> Option<PhysicalMove> {
        if self.items.is_empty() {
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
            self.cursor += 1;
            return true
        }
    }

    /// カーソルだけ戻す。
    pub fn back(&mut self) {
        if self.cursor < 0 {
            // 戻れない。
            return
        };

        self.cursor -= 1;
    }

    pub fn to_sign(&self, board_size:BoardSize) -> String {
        let mut sign = "".to_string();
        let mut ply = 1;
        for physical_move in &self.items {
            sign = format!("{} {}", sign, physical_move.to_sign(board_size, &mut ply));
        }
        sign
    }
}