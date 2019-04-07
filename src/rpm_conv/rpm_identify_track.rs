use communication::*;
use position::*;
// use rpm_conv::*;

const NONE_IDENTIFY:i16 = -1;

/// Reversible physical move - Identify track.
#[derive(Default)]
pub struct RpmITrack {
    items: Vec<i16>,
    cursor: i16,
}
impl RpmITrack {
    pub fn default() -> RpmITrack {
        RpmITrack {
            items: Vec::new(),
            cursor: -1,
        }
    }

    /// TODO 保存。
    pub fn save(&self, _comm:&Communication, _board_size:BoardSize) {
        // let book = Book::new();
        // book.save_rpm_i_track(&comm, board_size, &self);
    }

    fn up_count(&mut self) {
        self.cursor += 1;
    }

    fn down_count(&mut self) {
        self.cursor -= 1;
    }

    pub fn add_identify(&mut self, identify:i16) {
        // 追加しようとしたとき、すでに後ろの要素がある場合は、後ろの要素を削除する。
        if (self.cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.cursor, self.items.len());
            self.items.truncate((self.cursor + 1) as usize)
        };

        if self.items.len() == (self.cursor + 1) as usize {
            // 追加。
            self.items.push(identify);
            self.up_count();
        } else {
            panic!("Unexpected add: cursor: {}, len: {}.", self.cursor, self.items.len());
        }
    }

    pub fn pop_current(&mut self) -> i16 {
        // 後ろの要素がある場合は、削除する。
        if (self.cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.cursor, self.items.len());
            self.items.truncate((self.cursor + 1) as usize)
        };

        if self.items.is_empty() {
            NONE_IDENTIFY
        } else {
            let last = self.items.len()-1;
            let deleted_identify = self.items[last];
            self.items.remove(last);
            self.down_count();
            deleted_identify
        }
    }

    pub fn get_cursor(&self) -> i16 {
        self.cursor
    }

    /// カーソルが指している要素を返す。
    pub fn get_current(&self) -> i16 {
        if self.cursor == -1 {
            NONE_IDENTIFY
        } else {
            self.items[self.cursor as usize]
        }
    }

    /// カーソルだけ進める。
    pub fn forward(&mut self) -> bool {
        if self.items.len() as i16 <= (self.cursor + 1) {
            // 進めない。
            false
        } else {
            self.up_count();
            true
        }
    }

    /// カーソルだけ戻す。
    pub fn back(&mut self) {
        if self.cursor < 0 {
            // 戻れない。
            return
        };

        self.down_count();
    }

    /// コマンドライン入力形式。
    pub fn to_sign(&self, _board_size:BoardSize) -> String {
        let mut sign = "".to_string();
        for identify in &self.items {
            sign = format!("{} {}", sign, identify);
        }
        sign
    }

    /// JSONファイル保存形式。
    pub fn to_json(&self, _board_size:BoardSize) -> String {
        let mut text = "".to_string();
        let mut iter = self.items.iter();

        if !self.items.is_empty() {
            text = format!("{} {}", text, iter.next().unwrap());
        }

        for _index in 1..self.items.len() {
            text = format!("{}, {}", text, iter.next().unwrap());
        }
        
        text.trim_start().to_string()
    }
}