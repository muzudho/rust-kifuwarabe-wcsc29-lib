use board_size::*;
use communication::*;

const NONE_IDENTIFY:i8 = -1;

/// Reversible physical move - Identify track.
#[derive(Default)]
pub struct RpmITrack {
    // piece number.
    pub items: Vec<i8>,
}
impl RpmITrack {
    pub fn default() -> RpmITrack {
        RpmITrack {
            items: Vec::new(),
        }
    }

    /// 連結。
    pub fn append_track(&mut self, track:&mut RpmITrack) {
        self.items.append(&mut track.items);
    }

    /// TODO 保存。
    pub fn save(&self, _comm:&Communication, _board_size:BoardSize) {
        // let book = Book::new();
        // book.save_rpm_i_track(&comm, board_size, &self);
    }

    pub fn add_identify(&mut self, identify:i8, cursor:&mut i16) {
        // 追加しようとしたとき、すでに後ろの要素がある場合は、後ろの要素を削除する。
        if (*cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", *cursor, self.items.len());
            self.items.truncate((*cursor + 1) as usize)
        };

        if self.items.len() == (*cursor + 1) as usize {
            // 追加。
            self.items.push(identify);

            *cursor += 1;
        } else {
            panic!("Unexpected add: cursor: {}, len: {}.", *cursor, self.items.len());
        }
    }

    pub fn pop_current(&mut self, cursor:&mut i16) -> i8 {
        // 後ろの要素がある場合は、削除する。
        if (*cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", *cursor, self.items.len());
            self.items.truncate((*cursor + 1) as usize)
        };

        if self.items.is_empty() {
            NONE_IDENTIFY
        } else {
            let last = self.items.len()-1;
            let deleted_identify = self.items[last];
            self.items.remove(last);
            *cursor -= 1;
            deleted_identify
        }
    }

    /// カーソルが指している要素を返す。
    pub fn get_current(&self, cursor:i16) -> i8 {
        if cursor == -1 {
            NONE_IDENTIFY
        } else {
            self.items[cursor as usize]
        }
    }

    /// カーソルだけ進める。
    pub fn forward(&mut self, cursor:&mut i16) -> bool {
        if self.items.len() as i16 <= (*cursor + 1) {
            // 進めない。
            false
        } else {
            *cursor += 1;
            true
        }
    }

    /// カーソルだけ戻す。
    pub fn back(&mut self, cursor:&mut i16) {
        if *cursor < 0 {
            // 戻れない。
            return
        };

        *cursor -= 1;
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