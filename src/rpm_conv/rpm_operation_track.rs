use communication::*;
use position::*;
use rpm_conv::rpm_operation_note::*;
use rpm_conv::*;

/// Reversible physical move - Operation track.
#[derive(Default)]
pub struct RpmOTrack {
    items: Vec<RpmNote>,
    cursor: i16,
    ply: i16,
}
impl RpmOTrack {
    pub fn default() -> RpmOTrack {
        RpmOTrack {
            items: Vec::new(),
            cursor: -1,
            // 開始時点で、1手目進行中 として扱います。
            ply: 1,
        }
    }

    fn up_count(&mut self, rpm_note:&RpmNote) {
        self.cursor += 1;
        if rpm_note.is_phase_change() {
            self.ply += 1;
        }
    }

    fn up_count_retry(&mut self) {
        self.cursor += 1;
        let rpm_note = self.items[self.cursor as usize];
        if rpm_note.is_phase_change() {
            self.ply += 1;
        }
    }

    fn down_count(&mut self, rpm_note:&RpmNote) {
        self.cursor -= 1;
        if rpm_note.is_phase_change() {
            self.ply -= 1;
        }
    }

    fn down_count_retry(&mut self) {
        // フェーズ切り替えがあったら、手目を１つ減らす。
        let rpm_note = self.items[self.cursor as usize];
        if rpm_note.is_phase_change() {
            self.ply -= 1;
        }

        self.cursor -= 1;
        if self.cursor < 0 {
            // 何も記録していない内部状態に相当。
            return;
        }
    }

    pub fn add(&mut self, rpm_note:&RpmNote) {
        // 追加しようとしたとき、すでに後ろの要素がある場合は、後ろの要素を削除する。
        if (self.cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.cursor, self.items.len());
            self.items.truncate((self.cursor + 1) as usize)
        };

        if self.items.len() == (self.cursor + 1) as usize {
            // 追加。
            self.items.push(*rpm_note);
            self.up_count(rpm_note);
        } else {
            panic!("Unexpected add: cursor: {}, len: {}.", self.cursor, self.items.len());
        }
    }

    pub fn pop_current(&mut self) -> Option<RpmNote> {
        // 後ろの要素がある場合は、削除する。
        if (self.cursor + 1) < self.items.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.cursor, self.items.len());
            self.items.truncate((self.cursor + 1) as usize)
        };

        if self.items.is_empty() {
            None
        } else {
            let last = self.items.len()-1;
            let deleted_rpm_note = self.items[last];
            self.items.remove(last);
            self.down_count(&deleted_rpm_note);
            Some(deleted_rpm_note)
        }
    }

    pub fn get_cursor(&self) -> i16 {
        self.cursor
    }

    pub fn get_ply(&self) -> i16 {
        self.ply
    }

    /// カーソルが指している要素を返す。
    pub fn get_current(&self) -> Option<RpmNote> {
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
            false
        } else {
            self.up_count_retry();
            true
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

    /// 定跡ファイルの保存形式でもある。
    pub fn to_sign(&self, board_size:BoardSize, ply:&mut i16) -> String {
        let mut sign = "".to_string();
        for rpm_note in &self.items {
            sign = format!("{} {}", sign, rpm_note.to_sign(board_size, ply));
        }
        sign
    }
}