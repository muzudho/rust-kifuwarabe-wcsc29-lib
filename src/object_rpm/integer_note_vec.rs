use board_size::*;
use common::caret::*;
use communication::*;
use object_rpm::shogi_move::ShogiMove;
use object_rpm::shogi_note::*;
use std::*;

const NONE_VALUE: i8 = -1;

/// Reversible physical move.
/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
#[derive(Default)]
pub struct IntegerNoteVec {
    pub positive_notes: Vec<ShogiNote>,
    pub negative_notes: Vec<ShogiNote>,
}
impl fmt::Display for IntegerNoteVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "+Len: {}, -Len: {}.",
            self.positive_notes.len(),
            self.negative_notes.len()
        )
    }
}
impl IntegerNoteVec {
    pub fn default() -> Self {
        IntegerNoteVec {
            positive_notes: Vec::new(),
            negative_notes: Vec::new(),
        }
    }

    pub fn from_vector(positive_v: Vec<ShogiNote>, negative_v: Vec<ShogiNote>) -> Self {
        IntegerNoteVec {
            positive_notes: positive_v,
            negative_notes: negative_v,
        }
    }

    pub fn from_1_move(removable_rmove: &ShogiMove) -> Self {
        let mut pnotes = Vec::new();

        // & を頭に付けると元のベクターは残るらしい。付けてないとアクセスできなくなるらしい。
        pnotes.extend(&removable_rmove.notes);

        IntegerNoteVec {
            positive_notes: pnotes,
            negative_notes: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.positive_notes.clear();
        self.negative_notes.clear();
    }

    /// Human presentable large log.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        let mut dump;

        {
            dump = format!("Len-{}:", self.negative_notes.len());
        }
        for note in &self.negative_notes {
            dump = format!(
                "{} ({}'{}')",
                dump,
                if let Some(pid) = note.get_id() {
                    pid.to_human_presentable()
                } else {
                    NONE_VALUE.to_string()
                },
                note.get_ope().to_human_presentable(board_size),
            );
        }

        {
            dump = format!("{} Len+{}:", dump, self.positive_notes.len());
        }
        for note in &self.positive_notes {
            dump = format!(
                "{} ({}'{}')",
                dump,
                if let Some(pid) = note.get_id() {
                    pid.to_human_presentable()
                } else {
                    NONE_VALUE.to_string()
                },
                note.get_ope().to_human_presentable(board_size),
            );
        }

        dump
    }

    pub fn len_positive(&self) -> u16 {
        self.positive_notes.len() as u16
    }
    pub fn len_negative(&self) -> u16 {
        self.negative_notes.len() as u16
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    pub fn go_1note_forcely(
        &self,
        note_caret: &mut Caret,
        comm: &Communication,
    ) -> Option<ShogiNote> {
        let (is_positive, index) = note_caret.to_index();

        if note_caret.is_facing_left() {
            // 負の無限大の方に向いているとき。
            if !is_positive {
                if self.negative_notes.len() <= index {
                    // 負の先端要素を超えたら。
                    None
                } else {
                    // 負。
                    note_caret.go_next(comm, "tape-go_1note_forcely");
                    Some(self.negative_notes[index as usize])
                }
            } else {
                // 正。
                note_caret.go_next(comm, "tape+go_1note_forcely");
                Some(self.positive_notes[index as usize])
            }
        } else {
            // 正の無限大の方に向いているとき。
            if is_positive {
                if self.positive_notes.len() <= index {
                    // 正の先端要素を超えたら。
                    None
                } else {
                    // 正。
                    note_caret.go_next(comm, "tape+go_1note_forcely");
                    Some(self.positive_notes[index as usize])
                }
            } else {
                // 負。
                note_caret.go_next(comm, "tape+go_1note_forcely");
                Some(self.negative_notes[index as usize])
            }
        }
    }

    /// start <= end.
    pub fn slice(&self, start: i16, end: i16) -> Vec<ShogiNote> {
        //let len = end - start;
        let mut v = Vec::new();

        if start < 0 {
            // 負のテープ。正のテープに及ぶこともある。
            if end < 0 {
                // 負のテープだけで収まります。Endは含めず、Startは含めます。
                let s = &self.negative_notes[(-end + 1) as usize..(-start + 1) as usize];
                v.extend_from_slice(s);
            } else {
                // ひとまず、負のテープすべて。
                let s1 = &self.negative_notes[..];
                v.extend_from_slice(s1);

                // 正のテープの 0 から End まで。
                let s2 = &self.positive_notes[..end as usize];
                v.extend_from_slice(s2);
            }
        } else {
            // 正のテープだけ。
            // こりゃカンタンだ☆（＾～＾）
            let s = &self.positive_notes[start as usize..end as usize];
            v.extend_from_slice(s);
        }

        v
    }

    /// 先端への　足し継ぎ　も、中ほどの　リプレース　もこれで。
    pub fn overwrite_note(&self, note_caret: Caret, note: ShogiNote) -> Self {
        let (is_positive, index) = note_caret.to_index();

        let mut posi_v = Vec::new();
        let mut nega_v = Vec::new();

        if is_positive {
            // 正のテープ。
            // こりゃカンタンだ☆（＾～＾）
            nega_v.extend_from_slice(&self.negative_notes[..]);
            posi_v.extend_from_slice(&self.slice(0, index as i16));
            posi_v.push(note);
            if index < self.len_positive() as usize {
                posi_v.extend_from_slice(&self.slice(index as i16 + 1, self.len_positive() as i16));
            }
        } else {
            // 負のテープだけ。
            // 例えば 負のテープに
            // [-1, -2, -3, -4, -5]
            // というデータが入っているとき、start: 2 なら -3 を差し替えることを意味します。

            // Endは含めず、Startは含めます。
            nega_v.extend_from_slice(&self.slice(0, index as i16));
            nega_v.push(note);
            if index < self.len_negative() as usize {
                nega_v.extend_from_slice(&self.slice(index as i16 + 1, self.len_negative() as i16));
            }
            posi_v.extend_from_slice(&self.positive_notes[..]);
        }

        IntegerNoteVec::from_vector(posi_v, nega_v)
    }

    /// 削除はこれ。
    /// キャレットから見て、絶対値の大きな方を切り落とした結果を作るぜ☆（＾～＾）
    /// キャレットは使うが、動かさない☆（＾～＾）
    ///
    /// 切り落とした側の、こちらに一番近い要素を返すぜ☆（＾～＾）
    /// そんな要素がなければ None を返す。
    ///
    /// # Returns
    ///
    /// (RpmTape, Removed note)
    pub fn new_truncated_tape(&self, note_caret: &Caret) -> (Self, Option<ShogiNote>) {
        let mut posi_v = Vec::new();
        let mut nega_v = Vec::new();

        let (is_positive, index) = note_caret.to_index();

        if index == 0 {
            (IntegerNoteVec::from_vector(posi_v, nega_v), None)
        } else {
            let removed_note_opt = if is_positive {
                // 正のテープ側で切り落とし。
                // 負の部分はそのまま残して、正の絶対値の大きな方を切り落とす☆（＾～＾）
                nega_v.extend_from_slice(&self.negative_notes[..]);
                posi_v.extend_from_slice(&self.slice(0, index as i16));

                if index < self.positive_notes.len() {
                    Some(self.positive_notes[index])
                } else {
                    None
                }
            } else {
                // 負のテープ側で切り落とし。
                // 正の部分はそのまま残して、負の絶対値の大きな方を切り落とす☆（＾～＾）
                posi_v.extend_from_slice(&self.positive_notes[..]);
                nega_v.extend_from_slice(&self.slice(0, index as i16));

                if index < self.negative_notes.len() {
                    Some(self.negative_notes[index])
                } else {
                    None
                }
            };

            (
                IntegerNoteVec::from_vector(posi_v, nega_v),
                removed_note_opt,
            )
        }
    }

    /// 連結。
    pub fn append_tape_to_right(&mut self, tape_to_empty: &mut IntegerNoteVec) {
        self.positive_notes
            .append(&mut tape_to_empty.negative_notes);
        self.positive_notes
            .append(&mut tape_to_empty.positive_notes);
    }
    pub fn append_tape_to_left(&mut self, tape_to_empty: &mut IntegerNoteVec) {
        self.negative_notes
            .append(&mut tape_to_empty.positive_notes);
        self.negative_notes
            .append(&mut tape_to_empty.negative_notes);
    }

    /// コマンドライン入力形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size: BoardSize) -> (String, String) {
        let mut numbers = "".to_string();
        let mut operations = "".to_string();

        for note in &self.negative_notes {
            numbers = format!(
                "{} {}",
                numbers,
                if let Some(pid) = note.get_id() {
                    pid.get_number().to_string()
                } else {
                    NONE_VALUE.to_string()
                }
            );
            operations = format!("{} {}", operations, note.get_ope().to_sign(board_size));
        }

        for note in &self.positive_notes {
            numbers = format!(
                "{} {}",
                numbers,
                if let Some(pid) = note.get_id() {
                    pid.get_number().to_string()
                } else {
                    NONE_VALUE.to_string()
                }
            );
            operations = format!("{} {}", operations, note.get_ope().to_sign(board_size));
        }

        (numbers, operations)
    }

    /// JSONファイル保存形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号（ダブルクォーテーション含む）, 操作（ダブルクォーテーション含まず）。
    pub fn to_tracks_json(&self, board_size: BoardSize) -> (String, String) {
        // 背番号トラック。
        let mut numbers = "".to_string();
        for sign in 0..2 {
            let mut notes = if sign == 0 {
                &self.negative_notes
            } else {
                &self.positive_notes
            };

            let mut is_first = true;
            for note in notes {
                if is_first {
                    // 最初はカンマなし。
                    numbers = if let Some(pid) = note.get_id() {
                        pid.get_number().to_string()
                    } else {
                        NONE_VALUE.to_string()
                    };
                } else {
                    // ２つ目からカンマあり。
                    numbers = format!(
                        "{}, {}",
                        numbers,
                        if let Some(pid) = note.get_id() {
                            pid.get_number().to_string()
                        } else {
                            NONE_VALUE.to_string()
                        }
                    );
                }

                is_first = false;
            }
        }

        // 操作トラック。
        let mut operations = "".to_string();
        for sign in 0..2 {
            let mut notes = if sign == 0 {
                &self.negative_notes
            } else {
                &self.positive_notes
            };

            let mut is_first = true;
            for note in notes {
                if is_first {
                    // 最初はスペースなし。
                    operations = note.get_ope().to_sign(board_size).to_string();
                } else {
                    // ２つ目からスペース区切り。
                    operations = format!("{} {}", operations, note.get_ope().to_sign(board_size));
                }

                is_first = false;
            }
        }

        (
            numbers.trim_start().to_string(),
            operations.trim_start().to_string(),
        )
    }
}
