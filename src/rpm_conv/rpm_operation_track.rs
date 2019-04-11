use position::*;
use rpm_conv::rpm_operation_note::*;

/// Reversible physical move - Operation track.
#[derive(Default)]
pub struct RpmOTrack {
    pub items: Vec<RpmOpeNote>,
    ply: i16,
}
impl RpmOTrack {
    pub fn default() -> RpmOTrack {
        RpmOTrack {
            items: Vec::new(),
            // 開始時点で、1手目進行中 として扱います。
            ply: 1,
        }
    }

    /// 連結。
    pub fn append_track(&mut self, track:&mut RpmOTrack) {
        self.items.append(&mut track.items);
    }

    fn up_count(&mut self, rpm_note:&RpmOpeNote) {
        if rpm_note.is_phase_change() {
            self.ply += 1;
        }
    }

    fn up_count_retry(&mut self, cursor:i16) {
        let rpm_note = self.items[cursor as usize];
        if rpm_note.is_phase_change() {
            self.ply += 1;
        }
    }

    fn down_count(&mut self, rpm_note:&RpmOpeNote) {
        if rpm_note.is_phase_change() {
            self.ply -= 1;
        }
    }

    fn down_count_retry(&mut self, cursor:&mut i16) {
        // フェーズ切り替えがあったら、手目を１つ減らす。
        let rpm_note = self.items[*cursor as usize];
        if rpm_note.is_phase_change() {
            self.ply -= 1;
        }

        *cursor -= 1;
        if *cursor < 0 {
            // 何も記録していない内部状態に相当。
            return;
        }
    }

    pub fn add_element(&mut self, rpm_note:&RpmOpeNote, cursor:&mut i16) {
        // 追加しようとしたとき、すでに後ろの要素がある場合は、後ろの要素を削除する。
        if (*cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", *cursor, self.items.len());
            self.items.truncate((*cursor + 1) as usize)
        };

        if self.items.len() == (*cursor + 1) as usize {
            // 追加。
            self.items.push(*rpm_note);

            *cursor += 1;
            self.up_count(rpm_note);
        } else {
            panic!("Unexpected add: cursor: {}, len: {}.", *cursor, self.items.len());
        }
    }

    pub fn pop_current(&mut self, cursor:&mut i16) -> Option<RpmOpeNote> {
        // 後ろの要素がある場合は、削除する。
        if (*cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", *cursor, self.items.len());
            self.items.truncate((*cursor + 1) as usize)
        };

        if self.items.is_empty() {
            None
        } else {
            let last = self.items.len()-1;
            let deleted_rpm_note = self.items[last];
            self.items.remove(last);
            *cursor -= 1;
            self.down_count(&deleted_rpm_note);
            Some(deleted_rpm_note)
        }
    }

    pub fn get_ply(&self) -> i16 {
        self.ply
    }

    /// カーソルが指している要素を返す。
    pub fn get_current(&self, cursor:i16) -> Option<RpmOpeNote> {
        if cursor == -1 { // self.items.is_empty()
            None
        } else {
            Some(self.items[cursor as usize])
        }
    }

    /// カーソルだけ進める。
    pub fn forward(&mut self, cursor:&mut i16) -> bool {
        if self.items.len() as i16 <= (*cursor + 1) {
            // 進めない。
            false
        } else {
            *cursor += 1;
            self.up_count_retry(*cursor);
            true
        }
    }

    /// カーソルだけ戻す。
    pub fn back(&mut self, cursor:&mut i16) {
        if *cursor < 0 {
            // 戻れない。
            return
        };

        self.down_count_retry(cursor);
    }

    /// コマンドライン入力形式。
    pub fn to_sign(&self, board_size:BoardSize, ply:&mut i16) -> String {
        let mut sign = "".to_string();
        for rpm_note in &self.items {
            sign = format!("{} {}", sign, rpm_note.to_sign(board_size, ply));
        }
        sign
    }

    /// JSONファイル保存形式。
    pub fn to_json(&self, board_size:BoardSize, ply:&mut i16) -> String {
        let mut text = "".to_string();
        let mut iter = self.items.iter();

        if !self.items.is_empty() {
            text = format!("{} \"{}\"", text, iter.next().unwrap().to_sign(board_size, ply));
        }

        for _index in 1..self.items.len() {
            text = format!("{}, \"{}\"", text, iter.next().unwrap().to_sign(board_size, ply));
        }

        text.trim_start().to_string()
    }
}