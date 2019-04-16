/// Reversible physical move.
use board_size::*;
use rpm_conv::thread::rpm_move::*;
use rpm_conv::thread::rpm_note::*;
use std::fmt;

const ORIGIN_CARET_POSITION:i16 = 0;
const NONE_VALUE:i8 = -1;

/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
#[derive(Default)]
pub struct RpmTape {
    pub caret: i16,
    pub positive_notes: Vec<RpmNote>,
    pub negative_notes: Vec<RpmNote>,
}
impl fmt::Display for RpmTape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caret: {}, +Len: {}, -Len: {}.", self.caret, self.positive_notes.len(), self.negative_notes.len())
    }
}
impl RpmTape {
    pub fn default() -> Self {
        RpmTape {
            caret: ORIGIN_CARET_POSITION,
            positive_notes: Vec::new(),
            negative_notes: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.caret = ORIGIN_CARET_POSITION;
        self.positive_notes.clear();
        self.negative_notes.clear();
    }

    pub fn go_to_origin(&mut self) {
        self.caret = ORIGIN_CARET_POSITION;
    }

    /// 連結。
    pub fn append_next_tape(&mut self, tape:&mut RpmTape) {
        self.positive_notes.append(&mut tape.negative_notes);
        self.positive_notes.append(&mut tape.positive_notes);
    }
    pub fn append_back_tape(&mut self, tape:&mut RpmTape) {
        self.negative_notes.append(&mut tape.negative_notes);
        self.negative_notes.append(&mut tape.positive_notes);
    }

    pub fn record_next_note(&mut self, note:RpmNote) {
        if self.caret < 0 {
            // 負のテープの場合、この処理は失敗。
            panic!("Record next fail in negative tape.");
        }

        // 最後尾かどうか判断。
        if self.caret as usize == self.positive_notes.len() {
            // 最後尾に達していれば、追加。
            self.positive_notes.push(note);
            self.caret += 1;
        } else {
            // 最後尾でなければ、上書き。
            self.positive_notes[self.caret as usize] = note;
            self.caret += 1;

            // 仮のおわり を更新。
            self.positive_notes.truncate(self.caret as usize);
        }

        /*
        if note.get_ope().is_phase_change() {
            *ply += 1;
        }
         */
    }
    pub fn record_back_note(&mut self, note:RpmNote) {
        if self.caret > 0 {
            // 正のテープの場合、この処理は失敗。
            panic!("Record back fail in positive tape.");
        }

        // 最後尾かどうか判断。
        if (-self.caret - 1) as usize == self.negative_notes.len() {
            // 最後尾に達していれば、追加。
            self.negative_notes.push(note);
            self.caret -= 1;
        } else {
            // 最後尾でなければ、上書き。
            self.negative_notes[(-self.caret - 1) as usize] = note;
            self.caret -= 1;

            // 仮のおわり を更新。
            self.negative_notes.truncate((-self.caret - 1) as usize);
        }

        /*
        if note.get_ope().is_phase_change() {
            *ply -= 1;
        }
        */
    }

    /// 現在の要素を返してから、カーソルを進める。
    pub fn next_note(&mut self) -> Option<RpmNote> {
        if self.caret < 0 {
            // 負のテープの場合、この処理は失敗。
            panic!("Next fail in negative tape.");
        }

        // 最後尾かどうか判断。
        if self.caret as usize == self.positive_notes.len() {
            // 最後尾に達していれば、終端を示す。
            print!("GafE<{}>", self);
            None
        } else {
            print!("Gaf<{}>", self);
            let note = self.positive_notes[self.caret as usize];
            self.caret += 1;
            Some(note)
        }
    }
    /// 現在の要素を返してから、カーソルを戻す。
    pub fn back_note(&mut self) -> Option<RpmNote> {
        if self.caret > 0 {
            // 正のテープの場合、この処理は失敗。
            panic!("Back fail in positive tape.");
        }

        // 最後尾かどうか判断。
        if (-self.caret - 1) as usize == self.negative_notes.len() {
            // 最後尾に達していれば、終端を示す。
            None
        } else {
            let note = self.negative_notes[(-self.caret - 1) as usize];
            self.caret -= 1;
            Some(note)
        }
    }

    pub fn record_next_move(&mut self, rmove:&RpmMove, ply:&mut i16) {
        for note in rmove.notes.iter() {
            self.record_next_note(*note);
            if note.get_ope().is_phase_change() {
                *ply += 1;
            }
        }
    }
    pub fn record_back_move(&mut self, rmove:&RpmMove, ply:&mut i16) {
        for note in rmove.notes.iter() {
            self.record_back_note(*note);
            if note.get_ope().is_phase_change() {
                *ply -= 1;
            }
        }
    }

    pub fn delete_back(&mut self, ply:&mut i16) -> Option<RpmNote> {
        if self.caret < 0 {
            // 負のテープの場合、この処理は失敗。
            panic!("Delete back fail in negative tape.");
        }

        // 後ろの要素がある場合は、削除する。
        if (self.caret + 1) < self.positive_notes.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.caret, self.positive_notes.len());
            self.positive_notes.truncate((self.caret + 1) as usize)
        };

        if let Some(deleted_note) = self.positive_notes.pop() {
            self.caret -= 1;

            if deleted_note.get_ope().is_phase_change() {
                *ply -= 1;
            }

            Some(deleted_note)
        } else {
            // Empty.
            None
        }
    }
    pub fn delete_next(&mut self, ply:&mut i16) -> Option<RpmNote> {
        if self.caret > 0 {
            // 正のテープの場合、この処理は失敗。
            panic!("Delete next fail in positive tape.");
        }

        // 後ろの要素がある場合は、削除する。
        if (-self.caret - 1) < self.negative_notes.len() as i16 {
            println!("後ろの要素を削除。 {}, {}.", self.caret, self.negative_notes.len());
            self.negative_notes.truncate((-self.caret - 1 + 1) as usize)
        };

        if let Some(deleted_note) = self.negative_notes.pop() {
            self.caret += 1;

            if deleted_note.get_ope().is_phase_change() {
                *ply += 1;
            }

            Some(deleted_note)
        } else {
            // Empty.
            None
        }
    }

    /// コマンドライン入力形式。
    /// 
    /// # Returns
    /// 
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size:BoardSize, ply:&mut i16) -> (String, String) {
        let mut numbers = "".to_string();
        let mut operations = "".to_string();

        for note in &self.negative_notes {
            numbers = format!("{} {}", numbers, if let Some(pid) = note.get_id() {pid.get_number().to_string()} else {NONE_VALUE.to_string()});
            operations = format!("{} {}", operations, note.get_ope().to_sign(board_size, ply));
        }

        for note in &self.positive_notes {
            numbers = format!("{} {}", numbers, if let Some(pid) = note.get_id() {pid.get_number().to_string()} else {NONE_VALUE.to_string()});
            operations = format!("{} {}", operations, note.get_ope().to_sign(board_size, ply));
        }

        (numbers, operations)
    }

    /// JSONファイル保存形式。
    /// 
    /// # Returns
    /// 
    /// 駒の背番号, 操作。
    pub fn to_json(&self, board_size:BoardSize, ply:&mut i16) -> (String, String) {
        let mut numbers = "".to_string();
        let mut operations = "".to_string();
        
        for i in 0..2 {
            let mut notes = if i==0 {
                &self.negative_notes
            } else {
                &self.positive_notes
            };

            // 最初はカンマなし。
            if !notes.is_empty() {
                let note = notes.iter().next().unwrap();
                numbers = format!("{} {}", numbers, if let Some(pid) = note.get_id() {pid.get_number().to_string()} else {NONE_VALUE.to_string()});
                operations = format!("{} \"{}\"", operations, note.get_ope().to_sign(board_size, ply));
            }

            for _index in 1..notes.len() {
                let note = notes.iter().next().unwrap();
                numbers = format!("{}, {}", numbers, if let Some(pid) = note.get_id() {pid.get_number().to_string()} else {NONE_VALUE.to_string()});
                operations = format!("{}, \"{}\"", operations, note.get_ope().to_sign(board_size, ply));
            }
        }
        
        (numbers.trim_start().to_string(), operations.trim_start().to_string())
    }
}