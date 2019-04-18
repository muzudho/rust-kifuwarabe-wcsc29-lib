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

    /// 負のテープにあるときのキャレットを、配列のインデックスに変換。
    pub fn get_index_in_negative(&self) -> usize {
        (-self.caret - 1) as usize
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

    pub fn get_positive_peak_caret(&self) -> i16 {
        self.positive_notes.len() as i16
    }
    pub fn is_positive_peak(&self) -> bool {
        self.caret == self.get_positive_peak_caret()
    }
    pub fn get_negative_peak_caret(&self) -> i16 {
        -(self.negative_notes.len() as i16)
    }
    pub fn is_negative_peak(&self) -> bool {
        -(self.get_index_in_negative() as i16) == self.get_negative_peak_caret()
    }   
    /*
    pub fn is_negative_last(&self) -> bool {
        self.get_index_in_negative() as usize == self.negative_notes.len() - 1
    }
     */

    pub fn record_next_note(&mut self, note:RpmNote) {
        if self.caret >= -1 {
            // 1を足したら根元が0以上の場合、正のテープ。
            // 最後尾かどうか判断。
            if self.is_positive_peak() {
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
        } else {
            // 負のテープの場合、この処理は失敗。
            panic!("Record next fail in negative tape.");
        }

    }
    pub fn overwrite_note_in_negative(&mut self, note:RpmNote) {
        let index = self.get_index_in_negative();
        self.negative_notes[index] = note;
    }
    pub fn update_negative_peak(&mut self){
        let length = self.get_index_in_negative() + 1;
        self.negative_notes.truncate(length);
    }
    pub fn record_back_note(&mut self, note:RpmNote) {
        if self.caret > 0 {
            // 1を引いても羽先が0以上なら、正のテープ。
            // 正のテープの場合、この処理は失敗。
            panic!("Record back fail in positive tape.");
        }

        // 最後尾かどうか判断。
        if self.is_negative_peak() {
            // 最後尾に達していれば、追加。
            self.negative_notes.push(note);
            self.caret -= 1;
        } else {
            // 最後尾でなければ、上書き。
            self.overwrite_note_in_negative(note);
            self.caret -= 1;

            // 仮のおわり を更新。
            self.update_negative_peak();
        }
    }

    /// 現在の要素を返してから、カーソルを進めます。
    pub fn next_note(&mut self) -> Option<RpmNote> {
        if self.caret >= -1 {
            // 1を足したら根元が0以上の場合、正のテープ。
            // 最後尾かどうか判断。
            if self.is_positive_peak() {
                // 最後尾に達していれば、終端を示す。
                print!("GafE<{}>", self);
                None
            } else {
                print!("Gaf<{}>", self);
                let note = self.positive_notes[self.caret as usize];
                self.caret += 1;
                Some(note)
            }
        } else {
            // 負のテープの場合。
            let note = self.negative_notes[self.get_index_in_negative()];
            self.caret += 1;
            Some(note)
        }

    }
    /// カーソルを戻してから、現在の要素を返します。
    pub fn back_note(&mut self) -> Option<RpmNote> {
        if self.caret > 0 {
            // 1を引いても羽先が0以上なら、正のテープ。
            self.caret -= 1;
            println!("caret: {}, +len: {}.", self.caret, self.positive_notes.len());
            let note = self.positive_notes[self.caret as usize];
            Some(note)

            // 負のテープの最後尾の場合。
        } else if self.get_index_in_negative() as i16 - 1 <= self.get_negative_peak_caret() {
            // 1を引いて先端に達していれば、終端を示す。
            None
        } else {
            self.caret -= 1;
            // TODO 長さが 0 なのに、 [0]アクセスすることがある。
            println!("caret: {}, -index: {}, -len: {}.", self.caret, self.get_index_in_negative(), self.negative_notes.len());
            let note = self.negative_notes[self.get_index_in_negative()];
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
        if self.caret >= 0 {
            // 正のテープの場合。

            // キャレットより正の大きい方に要素がある場合は、削除する。
            if self.caret + 1 < self.positive_notes.len() as i16 {
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
        } else {
            // TODO 負の方向へのデリート・バックは未定義。
            panic!("負の方向へのデリート・バックは未定義。");
        }
    }
    pub fn update_positive_peak(&mut self) {
        println!("後ろの要素を削除。 {}, {}.", self.caret, self.negative_notes.len());
        let length = self.get_index_in_negative() + 1;
        self.negative_notes.truncate(length)
    }
    pub fn delete_next(&mut self, ply:&mut i16) -> Option<RpmNote> {
        if self.caret > 0 {
            // TODO 正の方向へのデリート・ネクストは未定義。
            panic!("正の方向へのデリート・ネクストは未定義。");
        } else {
            // キャレットより負の小さい方に要素がある場合は、削除する。
            if self.get_index_in_negative() + 1 < self.negative_notes.len() {
                self.update_positive_peak();
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