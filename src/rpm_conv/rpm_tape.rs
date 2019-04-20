use board_size::*;
use rpm_conv::thread::rpm_note::*;
use std::*;

const NONE_VALUE: i8 = -1;

/// Reversible physical move.
/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
#[derive(Default)]
pub struct RpmTape {
    pub positive_notes: Vec<RpmNote>,
    pub negative_notes: Vec<RpmNote>,
}
impl fmt::Display for RpmTape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "+Len: {}, -Len: {}.",
            self.positive_notes.len(),
            self.negative_notes.len()
        )
    }
}
impl RpmTape {
    pub fn default() -> Self {
        RpmTape {
            positive_notes: Vec::new(),
            negative_notes: Vec::new(),
        }
    }

    /*
    pub fn from_tape(source: RpmTape) -> Self {
        RpmTape {
            positive_notes: source.positive_notes,
            negative_notes: source.negative_notes,
        }
    }
    */

    pub fn from_vector(positive_v: Vec<RpmNote>, negative_v: Vec<RpmNote>) -> Self {
        RpmTape {
            positive_notes: positive_v,
            negative_notes: negative_v,
        }
    }

    /// # Returns
    ///
    /// (is_positive, index)
    pub fn caret_to_index(caret: i16) -> (bool, usize) {
        if caret > -1 {
            (true, caret as usize)
        } else {
            (false, -caret as usize)
        }
    }

    /// Human presentable large log.
    pub fn to_dump(&self, board_size: BoardSize) -> String {
        let mut dump = "".to_string();

        {
            dump = format!("{}, Negative Len: {}, ", dump, self.negative_notes.len());
        }
        for note in &self.negative_notes {
            dump = format!(
                "{} ({}'{}')",
                dump,
                if let Some(pid) = note.get_id() {
                    pid.get_number().to_string()
                } else {
                    NONE_VALUE.to_string()
                },
                note.get_ope().to_sign(board_size),
            );
        }

        {
            dump = format!("{}, Positive Len: {}, ", dump, self.positive_notes.len());
        }
        for note in &self.positive_notes {
            dump = format!(
                "{} ({}'{}')",
                dump,
                if let Some(pid) = note.get_id() {
                    pid.get_number().to_string()
                } else {
                    NONE_VALUE.to_string()
                },
                note.get_ope().to_sign(board_size),
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
    /*
    /// 原点は必ず含む or 何もない。
    pub fn get_end(&self) -> i16 {
        self.len_positive()
    }
    */

    /*
    pub fn clear(&mut self) {
        self.positive_notes.clear();
        self.negative_notes.clear();
    }
     */

    /// 範囲外を指定しないでください。
    pub fn get_note_by_caret(&self, caret: i16) -> RpmNote {
        let (is_positive, index) = RpmTape::caret_to_index(caret);
        if is_positive {
            self.positive_notes[index as usize]
        } else {
            self.negative_notes[index as usize]
        }
    }

    /// start <= end.
    pub fn slice(&self, start: i16, end: i16) -> Vec<RpmNote> {
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
    pub fn overwrite_note(&self, caret: i16, note: RpmNote) -> RpmTape {
        let (is_positive, index) = RpmTape::caret_to_index(caret);

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

        RpmTape::from_vector(posi_v, nega_v)
    }

    /// 正の大きな先端から、原点に向かって削除するぜ☆（＾～＾）
    ///
    /// 先端への　削除　も、中ほどからの　切り落とし　もこれで。
    /// 空テープなど、削除できない場合は None を返す。
    ///
    /// # Returns
    ///
    /// (RpmTape, Removed note)
    pub fn delete_back_note(&self, caret: i16) -> (RpmTape, Option<RpmNote>) {
        let (is_positive, index) = RpmTape::caret_to_index(caret);

        let mut posi_v = Vec::new();
        let mut nega_v = Vec::new();

        let removed_note_opt = if is_positive {
            // 正のテープ。
            // こりゃカンタンだ☆（＾～＾）
            // 負の部分はそのまま残す☆（＾～＾）
            nega_v.extend_from_slice(&self.negative_notes[..]);
            // 正の大きな部分は、切り落とし☆（＾～＾）
            posi_v.extend_from_slice(&self.slice(0, index as i16));

            if index < self.positive_notes.len() {
                Some(self.positive_notes[index])
            } else {
                None
            }
        } else {
            panic!("原点を含まない 負のテープ は存在しないぜ☆（＾～＾）");
            /*
            // 原点を含まない 負のテープ は存在しないので、全部消すぜ☆（＾～＾）
            if index < self.negative_notes.len() {
                Some(self.negative_notes[index])
            } else {
                None
            }
            */
        };

        (RpmTape::from_vector(posi_v, nega_v), removed_note_opt)
    }

    /// 負の大きな先端から、原点に向かって削除するぜ☆（＾～＾）
    ///
    /// 先端への　削除　も、中ほどからの　切り落とし　もこれで。
    /// 空テープなど、削除できない場合は None を返す。
    ///
    /// # Returns
    ///
    /// (RpmTape, Removed note)
    pub fn delete_next_note(&self, caret: i16) -> (RpmTape, Option<RpmNote>) {
        let (is_positive, index) = RpmTape::caret_to_index(caret);

        let mut posi_v = Vec::new();
        let mut nega_v = Vec::new();

        let removed_note_opt = if is_positive {
            panic!("原点を含まない 正のテープ は存在しないぜ☆（＾～＾）");
        /*
        // 原点を含まない 正のテープ は存在しないので、全部消すぜ☆（＾～＾）
        if index < self.positive_notes.len() {
            Some(self.positive_notes[index])
        } else {
            None
        }
        */
        } else {
            // 負のテープだけ。
            // 例えば 負のテープに
            // [-1, -2, -3, -4, -5]
            // というデータが入っているとき、start: 2 なら -3 を差し替えることを意味します。

            // Endは含めず、Startは含めます。
            nega_v.extend_from_slice(&self.slice(0, index as i16));
            // 負の大きな部分は、切り落とし☆（＾～＾）
            // 正の部分はそのまま残す☆（＾～＾）
            posi_v.extend_from_slice(&self.positive_notes[..]);

            if index < self.negative_notes.len() {
                Some(self.negative_notes[index])
            } else {
                None
            }
        };

        (RpmTape::from_vector(posi_v, nega_v), removed_note_opt)
    }

    /// 連結。
    pub fn append_next_tape(&mut self, tape_to_empty: &mut RpmTape) {
        self.positive_notes
            .append(&mut tape_to_empty.negative_notes);
        self.positive_notes
            .append(&mut tape_to_empty.positive_notes);
    }
    pub fn append_back_tape(&mut self, tape_to_empty: &mut RpmTape) {
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
    /// 駒の背番号, 操作。
    pub fn to_json(&self, board_size: BoardSize) -> (String, String) {
        let mut numbers = "".to_string();
        let mut operations = "".to_string();

        for i in 0..2 {
            let mut notes = if i == 0 {
                &self.negative_notes
            } else {
                &self.positive_notes
            };

            // 最初はカンマなし。
            if !notes.is_empty() {
                let note = notes.iter().next().unwrap();
                numbers = format!(
                    "{} {}",
                    numbers,
                    if let Some(pid) = note.get_id() {
                        pid.get_number().to_string()
                    } else {
                        NONE_VALUE.to_string()
                    }
                );
                operations = format!("{} \"{}\"", operations, note.get_ope().to_sign(board_size));
            }

            for _index in 1..notes.len() {
                let note = notes.iter().next().unwrap();
                numbers = format!(
                    "{}, {}",
                    numbers,
                    if let Some(pid) = note.get_id() {
                        pid.get_number().to_string()
                    } else {
                        NONE_VALUE.to_string()
                    }
                );
                operations = format!("{}, \"{}\"", operations, note.get_ope().to_sign(board_size));
            }
        }

        (
            numbers.trim_start().to_string(),
            operations.trim_start().to_string(),
        )
    }
}
