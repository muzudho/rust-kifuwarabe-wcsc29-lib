use sheet_music_format::kifu_rpm::rpm_tape_tracks::*;
use sound::shogi_move::*;
use sound::shogi_note::*;
use std::*;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::*;
use studio::common::closed_interval::ClosedInterval;

const NONE_VALUE: i8 = -1;

/// Reversible physical move.
/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
#[derive(Default)]
pub struct TwoHeadsVec {
    positive_notes: Vec<ShogiNote>,
    negative_notes: Vec<ShogiNote>,
}
impl fmt::Display for TwoHeadsVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "+Len: {}, -Len: {}.",
            self.positive_notes.len(),
            self.negative_notes.len()
        )
    }
}
impl TwoHeadsVec {
    // ###############
    // # Constructor #
    // ###############

    pub fn new() -> Self {
        Self {
            positive_notes: Vec::new(),
            negative_notes: Vec::new(),
        }
    }

    pub fn from_vector(positive_v: Vec<ShogiNote>, negative_v: Vec<ShogiNote>) -> Self {
        Self {
            positive_notes: positive_v,
            negative_notes: negative_v,
        }
    }

    /// キャレット位置にノートを挿入した新しいオブジェクトを返します。
    pub fn new_vector_with_inserted_note(
        &self,
        caret: &mut Caret,
        note: ShogiNote,
        _board_size: BoardSize,
        app: &Application,
    ) -> Self {
        // ピークを取得。
        let awareness = caret.get_peak();
        /*
        if app.is_debug() {
            app.comm.println(&format!(
                "[#2H vec.insert: 挿入位置 {} caret, Len-{}+{}]",
                awareness.expected_caret,
                self.positive_notes.len(),
                self.negative_notes.len(),
            ));
        }
        */

        let mut posi_v = Vec::new();
        let mut nega_v = Vec::new();

        if awareness.negative {
            // 負のテープへ挿入。

            // 正のテープは丸コピー。
            let v0 = &self.positive_notes[..];
            posi_v.extend_from_slice(v0);

            // 要素その１。
            let v1 = self.slice_by_caret(0, awareness.expected_caret, &app);
            nega_v.extend_from_slice(&v1);

            // オーバー位置から挿入。
            nega_v.push(note);

            // 余りがあれば挿入。
            //let mut v2 = Vec::new();
            if let Some(index) = awareness.index {
                if index < self.negative_notes.len() {
                    let v2 = self.slice_by_caret(
                        index as i16 + 1,
                        self.negative_notes.len() as i16,
                        &app,
                    );
                    nega_v.extend_from_slice(&v2);
                }
            }

        /*
        if app.is_debug() {
            app.comm.println(&format!(
            "[#2H vec.insert: 負のテープへ挿入。 v0:[{}], v1:[{}]], note:[{}], v2:[{}], Nega:[{}], Posi:[{}]",
            ShogiNote::to_human_presentable_vec(v0.to_vec(), board_size, &app),
            ShogiNote::to_human_presentable_vec(v1.to_vec(), board_size, &app),
            note.to_human_presentable(board_size, &app),
            ShogiNote::to_human_presentable_vec(v2.to_vec(), board_size, &app),
            ShogiNote::to_human_presentable_vec(nega_v.to_vec(), board_size, &app),
            ShogiNote::to_human_presentable_vec(posi_v.to_vec(), board_size, &app)
        ));
        }
        */
        } else {
            // 正のテープへ挿入。

            // 負のテープは丸コピー。
            let v0 = &self.negative_notes[..];
            nega_v.extend_from_slice(v0);

            // 要素その１。
            let v1 = self.slice_by_caret(0, awareness.expected_caret, &app);
            posi_v.extend_from_slice(&v1);

            // オーバー位置から挿入。
            posi_v.push(note);

            // 余りがあれば挿入。
            //let mut v2 = Vec::new();
            if let Some(index) = awareness.index {
                if index < self.positive_notes.len() {
                    let v2 = self.slice_by_caret(
                        index as i16 + 1,
                        self.positive_notes.len() as i16,
                        &app,
                    );
                    posi_v.extend_from_slice(&v2);
                }
            }

            /*
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#2H vec.insert: 正のテープへ挿入。 v0:[{}], v1:[{}]], note:[{}], v2:[{}], Nega:[{}], Posi:[{}]",
                    ShogiNote::to_human_presentable_vec(v0.to_vec(), board_size, &app),
                    ShogiNote::to_human_presentable_vec(v1.to_vec(), board_size, &app),
                    note.to_human_presentable(board_size, &app),
                    ShogiNote::to_human_presentable_vec(v2.to_vec(), board_size, &app),
                    ShogiNote::to_human_presentable_vec(nega_v.to_vec(), board_size, &app),
                    ShogiNote::to_human_presentable_vec(posi_v.to_vec(), board_size, &app)
                ));
            }
            */
        }

        Self::from_vector(posi_v, nega_v)
    }

    // #####
    // # A #
    // #####

    /// 連結。
    pub fn append_tape_to_right(&mut self, tape_to_empty: &mut Self) {
        self.positive_notes
            .append(&mut tape_to_empty.negative_notes);
        self.positive_notes
            .append(&mut tape_to_empty.positive_notes);
    }
    pub fn append_tape_to_left(&mut self, tape_to_empty: &mut Self) {
        self.negative_notes
            .append(&mut tape_to_empty.positive_notes);
        self.negative_notes
            .append(&mut tape_to_empty.negative_notes);
    }

    // #####
    // # C #
    // #####

    pub fn clear(&mut self) {
        self.positive_notes.clear();
        self.negative_notes.clear();
    }

    // #####
    // # G #
    // #####

    /// 範囲はキャレット番地で示す☆（＾～＾）
    /// ０に背を向けた２つのキャレットがあると仮定し、両端はピークを指すキャレット☆（＾～＾）
    /// 向きは取れない☆（＾～＾）
    pub fn get_span_caret_facing_outward(&self) -> ClosedInterval {
        ClosedInterval::from_all(
            self.get_negative_peak_caret_facing_outward(),
            self.get_positive_peak_caret_facing_outward(),
            true,
        )
    }

    pub fn get_negative_peak_caret_facing_outward(&self) -> i16 {
        self.negative_notes.len() as i16 + MINUS_ZERO_LEN as i16 + 1
    }

    pub fn get_positive_peak_caret_facing_outward(&self) -> i16 {
        self.positive_notes.len() as i16 - 1
    }

    // #####
    // # N #
    // #####

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
    pub fn new_truncated_tape(
        &self,
        caret: &Caret,
        app: &Application,
    ) -> (Self, Option<ShogiNote>) {
        let mut posi_v = Vec::new();
        let mut nega_v = Vec::new();

        let awareness = caret.get_peak();

        if awareness.expected_caret == 0 {
            // キャレットが０番地のときに切り落とすと、正の方、負の方の全部なくなる☆（＾～＾）
            (Self::from_vector(posi_v, nega_v), None)
        } else {
            let index = awareness
                .index
                .unwrap_or_else(|| panic!(app.comm.panic("Index fail.")));

            let removed_note_opt = if !awareness.negative {
                // 正のテープ側で切り落とし。
                // 負の部分はそのまま残して、正の絶対値の大きな方を切り落とす☆（＾～＾）
                nega_v.extend_from_slice(&self.negative_notes[..]);
                posi_v.extend_from_slice(&self.slice_by_caret(0, index as i16, &app));

                if index < self.positive_notes.len() {
                    Some(self.positive_notes[index])
                } else {
                    None
                }
            } else {
                // 負のテープ側で切り落とし。
                // 正の部分はそのまま残して、負の絶対値の大きな方を切り落とす☆（＾～＾）
                posi_v.extend_from_slice(&self.positive_notes[..]);
                nega_v.extend_from_slice(&self.slice_by_caret(0, index as i16, &app));

                if index < self.negative_notes.len() {
                    Some(self.negative_notes[index])
                } else {
                    None
                }
            };

            (Self::from_vector(posi_v, nega_v), removed_note_opt)
        }
    }

    // #####
    // # S #
    // #####

    /// キャレットは必ず１つ進みます。
    /// 0 は、正の数とします。（マイナスゼロは無いです）
    /// Noneを返したら、オーバーフローしています。
    ///
    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note(
        &self,
        caret: &mut Caret,
        app: &Application,
    ) -> (bool, Awareness, Option<ShogiNote>) {
        // とりあえず、キャレットを１つ進める。
        let awareness = caret.seek_a_note(&app);
        /*
        if app.is_debug() {
            app.comm
                .print(&format!("[#INVec.Seek a note: Awareness:{:?}]", awareness));
        }
        */

        if caret.is_facing_left() {
            // 負の無限大 <---- 顔の向き。
            if awareness.negative {
                // 負の方。
                if Some(self.negative_notes.len()) <= awareness.index {
                    // 配列の範囲外。
                    /*
                    if app.is_debug() {
                        app.comm
                            .println("[#Seek next note: <-- 負の方、配列の範囲外]");
                    }
                    */
                    (true, awareness, None)
                } else {
                    // 配列の範囲内。
                    /*
                    if app.is_debug() {
                        app.comm.println("[#Seek next note: <-- 負の方]");
                    }
                    */

                    let note = if let Some(index) = awareness.index {
                        Some(self.negative_notes[index])
                    } else {
                        None
                    };

                    (false, awareness, note)
                }
            } else {
                // 正の方。
                if Some(self.positive_notes.len()) <= awareness.index {
                    // 配列の範囲外。
                    /*
                    if app.is_debug() {
                        app.comm
                            .println("[#Seek next note: <-- 正の方、配列の範囲外]");
                    }
                    */

                    (true, awareness, None)
                } else {
                    // 配列の範囲内。
                    /*
                    if app.is_debug() {
                        app.comm.println("[#Seek next note: <-- 正の方]");
                    }
                    */

                    let note = if let Some(index) = awareness.index {
                        Some(self.positive_notes[index])
                    } else {
                        None
                    };

                    (false, awareness, note)
                }
            }
        } else {
            // 顔の向き ----> 正の無限大。
            if !awareness.negative {
                // 正の方。
                if Some(self.positive_notes.len()) <= awareness.index {
                    // 配列の範囲外。
                    /*
                    if app.is_debug() {
                        app.comm
                            .println("[#Seek next note: --> 正の方、配列の範囲外]");
                    }
                    */

                    (true, awareness, None)
                } else {
                    // 配列の範囲内。
                    /*
                    if app.is_debug() {
                        app.comm.println("[#Seek next note: --> 正の方]");
                    }
                    */

                    let note = if let Some(index) = awareness.index {
                        Some(self.positive_notes[index])
                    } else {
                        None
                    };

                    (false, awareness, note)
                }
            } else {
                // 負の方。
                if Some(self.negative_notes.len()) <= awareness.index {
                    // 配列の範囲外。
                    /*
                    if app.is_debug() {
                        app.comm
                            .println("[#Seek next note: --> 負の方、配列の範囲外]");
                    }
                    */

                    (true, awareness, None)
                } else {
                    // 配列の範囲内。
                    /*
                    if app.is_debug() {
                        app.comm.println("[#Seek next note: --> 負の方]");
                    }
                    */

                    let note = if let Some(index) = awareness.index {
                        Some(self.negative_notes[index])
                    } else {
                        None
                    };

                    (false, awareness, note)
                }
            }
        }
    }

    /// 現在の指し手をスキップするぜ☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (taken overflow, move)
    pub fn skip_a_move(&self, caret: &mut Caret, app: &Application) -> (bool, ShogiMove) {
        // 指し手（実際のところ、テープ上の範囲を示したもの）。
        let mut rmove = ShogiMove::new_facing_right_move();

        loop {
            // とりあえずキャレットを１つ進める。
            match self.seek_a_note(caret, &app) {
                (taken_overflow, awareness, Some(note)) => {
                    if note.is_phase_change() {
                        rmove
                            .caret_closed_interval
                            .intersect_2_values(awareness.passed_caret, awareness.expected_caret);
                        return (taken_overflow, rmove);
                    }
                    // ループを続行。
                }
                (taken_overflow, awareness, None) => {
                    // テープの終わり☆（＾～＾）
                    rmove
                        .caret_closed_interval
                        .intersect_2_values(awareness.passed_caret, awareness.expected_caret);
                    return (taken_overflow, rmove);
                }
            }
        }
    }

    /// 2つのキャレットで、欲しい範囲を挟んでください。
    /// start <= end.
    pub fn slice_by_caret(
        &self,
        start_caret: i16,
        end_caret: i16,
        _app: &Application,
    ) -> Vec<ShogiNote> {
        /*
        if app.is_debug() {
            app.comm.println(&format!(
                "[#2H vec.Slice: Span {}:{} caret]",
                start_caret, end_caret
            ));
        }
        */
        let mut v = Vec::new();

        if start_caret < 0 {
            // 負のテープを含む。
            if end_caret <= 0 {
                // 負のテープだけで範囲が収まります。
                let start_ix = (-end_caret + 1) as usize;
                let end_ix = (-start_caret + 1) as usize;
                /*
                if app.is_debug() {
                    app.comm.println(&format!(
                        "[#2H vec.Slice: 負のテープだけで範囲が収まります。 [Index {}:{}]]",
                        start_ix, end_ix
                    ));
                }
                */
                let s = &self.negative_notes[start_ix..end_ix];
                v.extend_from_slice(s);
            } else {
                /*
                if app.is_debug() {
                    app.comm.println(&format!(
                        "[#2H vec.Slice: 負のテープ全てと、正のテープの途中まで。 [Posi end {}]]",
                        end_caret
                    ));
                }
                */
                // ひとまず、負のテープすべて。
                let s1 = &self.negative_notes[..];
                v.extend_from_slice(s1);

                // 正のテープの 0 から End まで。
                let s2 = &self.positive_notes[..end_caret as usize];
                v.extend_from_slice(s2);
            }
        } else {
            // 正のテープだけ。
            let start_ix = start_caret as usize;
            let end_ix = end_caret as usize;
            /*
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#2H vec.Slice: 正のテープだけで範囲が収まります。 [Index {}:{}]]",
                    start_ix, end_ix
                ));
            }
            */
            // こりゃカンタンだ☆（＾～＾）
            let s = &self.positive_notes[start_ix..end_ix];
            v.extend_from_slice(s);
        }

        v
    }

    // #####
    // # T #
    // #####

    /// コマンドライン入力形式の棋譜。
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
    /// 駒の背番号（ダブルクォーテーション含まず）, 操作（ダブルクォーテーション含まず）。
    pub fn to_rpm_tracks(&self, board_size: BoardSize) -> RpmTapeTracks {
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
                    // 最初。
                    numbers = if let Some(pid) = note.get_id() {
                        pid.get_number().to_string()
                    } else {
                        NONE_VALUE.to_string()
                    };
                } else {
                    // ２つ目からスペース区切り。
                    numbers = format!(
                        "{} {}",
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
                    // 最初。
                    operations = note.get_ope().to_sign(board_size).to_string();
                } else {
                    // ２つ目からスペース区切り。
                    operations = format!("{} {}", operations, note.get_ope().to_sign(board_size));
                }

                is_first = false;
            }
        }

        RpmTapeTracks {
            // 駒の背番号は、半角スペース１個区切り。
            id: numbers.trim_start().to_string(),
            // 操作は、半角スペース１個区切り。
            ope: operations.trim_start().to_string(),
        }
    }

    /// Human presentable large log.
    pub fn to_human_presentable(&self, board_size: BoardSize, app: &Application) -> String {
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
                note.get_ope().to_human_presentable(board_size, &app),
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
                note.get_ope().to_human_presentable(board_size, &app),
            );
        }

        dump
    }
}
