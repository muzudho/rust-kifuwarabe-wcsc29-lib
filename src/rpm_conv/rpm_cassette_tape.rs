use board_size::*;
use rpm_conv::rpm_tape::*;
use rpm_conv::thread::rpm_move::*;
use rpm_conv::thread::rpm_note::*;
use std::*;

/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1753122
pub struct RpmCassetteTape {
    pub caret: i16,
    pub tape: RpmTape,
}
impl fmt::Display for RpmCassetteTape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caret: {}, {}", self.caret, self.tape)
    }
}
impl RpmCassetteTape {
    pub fn default() -> Self {
        RpmCassetteTape {
            caret: 0,
            tape: RpmTape::default(),
        }
    }

    pub fn reset_caret(&mut self) {
        self.caret = 0;
    }

    pub fn get_positive_peak_caret(&self) -> i16 {
        self.tape.len_positive() as i16
    }
    pub fn get_negative_peak_caret(&self) -> i16 {
        -(self.tape.len_negative() as i16) - 1
    }

    /*
    /// 負のテープにあるときのキャレットを、配列のインデックスに変換。
    pub fn get_index(&self) -> i16 {
        if self.caret > -1 {
            self.caret
        } else {
            (-self.caret - 1)
        }
    }
     */

    pub fn is_positive_peak(&self) -> bool {
        self.caret == self.get_positive_peak_caret()
    }
    pub fn is_negative_peak(&self) -> bool {
        self.caret == self.get_negative_peak_caret()
    }

    /// 連結。
    pub fn append_next_cassette_tape(&mut self, cassette_tape_to_empty: &mut RpmCassetteTape) {
        self.tape.append_next_tape(&mut cassette_tape_to_empty.tape);
    }
    pub fn append_back_cassette_tape(&mut self, cassette_tape_to_empty: &mut RpmCassetteTape) {
        self.tape.append_back_tape(&mut cassette_tape_to_empty.tape);
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
                let note = self.tape.get_note_by_caret(self.caret);
                self.caret += 1;
                Some(note)
            }
        } else {
            // 負のテープの場合。
            let note = self.tape.get_note_by_caret(self.caret);
            self.caret += 1;
            Some(note)
        }
    }
    /// カーソルを戻してから、現在の要素を返します。
    pub fn back_note(&mut self) -> Option<RpmNote> {
        if self.caret > 0 {
            // 1を引いても羽先が0以上なら、正のテープ。
            self.caret -= 1;
            println!("caret: {}, +len: {}.", self.caret, self.tape.len_positive());
            let note = self.tape.get_note_by_caret(self.caret);
            Some(note)

        // 負のテープの最後尾の場合。
        } else if -self.caret - 1 <= self.get_negative_peak_caret() {
            // 1を引いて先端に達していれば、終端を示す。
            None
        } else {
            self.caret -= 1;
            // TODO 長さが 0 なのに、 [0]アクセスすることがある。
            println!("caret: {}, -len: {}.", self.caret, self.tape.len_negative());
            let note = self.tape.get_note_by_caret(self.caret);
            Some(note)
        }
    }

    pub fn record_next_note(&mut self, note: RpmNote) {
        if self.caret >= -1 {
            // 1を足したら根元が0以上の場合、正のテープ。
            // 最後尾かどうか判断。
            if self.is_positive_peak() {
                // 最後尾に達していれば、追加。
                self.tape.positive_notes.push(note);
                self.caret += 1;
            } else {
                // 最後尾でなければ、上書き。
                self.tape.positive_notes[self.caret as usize] = note;
                self.caret += 1;

                // 仮のおわり を更新。
                self.tape.positive_notes.truncate(self.caret as usize);
            }
        } else {
            // 負のテープの場合、この処理は失敗。
            panic!("Record next fail in negative tape.");
        }
    }

    pub fn overwrite_note_in_negative(&mut self, note: RpmNote) {
        self.tape = self.tape.overwrite_note(self.caret, note);
    }
    /*
    pub fn update_negative_peak(&mut self) {
        let length = self.get_index() + 1;
        self.tape.negative_notes.truncate(length as usize);
    }
    */

    pub fn record_back_note(&mut self, note: RpmNote) {
        if self.caret > 0 {
            // 1を引いても羽先が0以上なら、正のテープ。
            // 正のテープの場合、この処理は失敗。
            panic!("Record back fail in positive tape.");
        }

        // 置換／上書き。新しいテープを作成。
        self.tape = self.tape.overwrite_note(self.caret, note);
        self.caret -= 1;
    }

    pub fn record_next_move(&mut self, rmove: &RpmMove, ply: &mut i16) {
        for note in rmove.notes.iter() {
            self.record_next_note(*note);
            if note.get_ope().is_phase_change() {
                *ply += 1;
            }
        }
    }
    pub fn record_back_move(&mut self, rmove: &RpmMove, ply: &mut i16) {
        for note in rmove.notes.iter() {
            self.record_back_note(*note);
            if note.get_ope().is_phase_change() {
                *ply -= 1;
            }
        }
    }

    pub fn delete_back(&mut self, ply: &mut i16) -> Option<RpmNote> {
        let (new_tape, removed_note_opt) = self.tape.delete_back_note(self.caret);
        self.tape = new_tape;

        self.caret -= 1;

        if let Some(removed_note) = removed_note_opt {
            if removed_note.get_ope().is_phase_change() {
                *ply -= 1;
            }

            Some(removed_note)
        } else {
            None
        }
    }
    pub fn delete_next(&mut self, ply: &mut i16) -> Option<RpmNote> {
        let (new_tape, removed_note_opt) = self.tape.delete_next_note(self.caret);
        self.tape = new_tape;

        self.caret -= 1;

        if let Some(removed_note) = removed_note_opt {
            if removed_note.get_ope().is_phase_change() {
                *ply -= 1;
            }

            Some(removed_note)
        } else {
            None
        }
    }

    /// Human presentable large log.
    pub fn to_dump(&self, board_size: BoardSize) -> String {
        self.tape.to_dump(board_size)
    }

    /// コマンドライン入力形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size: BoardSize, ply: &mut i16) -> (String, String) {
        self.tape.to_sign(board_size, ply)
    }

    /// JSONファイル保存形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_json(&self, board_size: BoardSize, ply: &mut i16) -> (String, String) {
        self.tape.to_json(board_size, ply)
    }
}
