use board_size::*;
use communication::*;
use piece_etc::*;

const NONE_VALUE:i8 = -1;

/// Reversible physical move - Identify track.
#[derive(Default)]
pub struct RpmITrack {
    // piece number.
    pub items: Vec<Option<PieceIdentify>>,
}
impl RpmITrack {
    pub fn default() -> RpmITrack {
        RpmITrack {
            items: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
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

    pub fn add_identify(&mut self, pid_opt:Option<PieceIdentify>, cursor:&mut i16) {
        // 追加しようとしたとき、すでに後ろの要素がある場合は、後ろの要素を削除する。
        if (*cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", *cursor, self.items.len());
            self.items.truncate((*cursor + 1) as usize)
        };

        if self.items.len() == (*cursor + 1) as usize {
            // 追加。
            self.items.push(pid_opt);

            *cursor += 1;
        } else {
            panic!("Unexpected add: cursor: {}, len: {}.", *cursor, self.items.len());
        }
    }

    pub fn pop_current(&mut self, cursor:&mut i16) -> Option<PieceIdentify> {
        // 後ろの要素がある場合は、削除する。
        if (*cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", *cursor, self.items.len());
            self.items.truncate((*cursor + 1) as usize)
        };

        if self.items.is_empty() {
            None
        } else {
            let last = self.items.len()-1;
            let deleted_identify = self.items[last];
            self.items.remove(last);
            *cursor -= 1;
            deleted_identify
        }
    }

    /// カーソルが指している要素を返す。
    pub fn get_current(&self, cursor:i16) -> Option<PieceIdentify> {
        if cursor == -1 {
            None
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
        for pid_opt in &self.items {
            sign = format!("{} {}", sign, if let Some(pid) = pid_opt {pid.get_number().to_string()} else {NONE_VALUE.to_string()});
        }
        sign
    }

    /// JSONファイル保存形式。
    pub fn to_json(&self, _board_size:BoardSize) -> String {
        let mut text = "".to_string();
        let mut iter = self.items.iter();

        // 最初はカンマなし。
        if !self.items.is_empty() {
            let token = if let Some(pid) = iter.next().unwrap() {pid.get_number().to_string()} else {NONE_VALUE.to_string()};
            text = format!("{} {}", text, token);
        }

        for _index in 1..self.items.len() {
            let token = if let Some(pid) = iter.next().unwrap() {pid.get_number().to_string()} else {NONE_VALUE.to_string()};
            text = format!("{}, {}", text, token);
        }
        
        text.trim_start().to_string()
    }
}