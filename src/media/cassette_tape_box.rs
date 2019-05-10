extern crate rand;
use audio_compo::cassette_deck::Slot;
use media::cassette_tape::*;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use std::*;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::get_index_from_caret_numbers;
use studio::common::caret::Awareness;
use studio::common::caret::Caret;
use studio::common::closed_interval::ClosedInterval;

/// 保存したいときは RPM棋譜 に変換して、そっちで保存しろだぜ☆（＾～＾）
pub struct CassetteTapeBox {
    // このテープボックスの役割。
    role_as_slot: Slot,

    file_name: String,

    // イテレーターを使いたいので public にしてある。
    pub tapes: Vec<CassetteTape>,

    // テープ間のキャレット。
    caret_of_tapes: Caret,
    awareness_of_tapes: Option<Awareness>,

    /// 何も指していない状態で 1。
    /// TODO 本将棋の大橋流の最初の玉は Ply=-39 にしたい。
    /// トレーニング・テープの 手目。
    pub ply: i16,
}
impl CassetteTapeBox {
    // トレーニング・テープ・ボックスと、ラーニング・テープ・ボックスの２つしか作られないはず。
    pub fn new_empty_tape_box(slot: Slot, app: &Application) -> Self {
        if app.is_debug() {
            app.comm
                .println(&format!("[#New tape box: Slot:{:?}]", slot))
        }

        CassetteTapeBox {
            role_as_slot: slot,
            file_name: "".to_string(),
            tapes: Vec::new(),
            caret_of_tapes: Caret::new_facing_right_caret(),
            awareness_of_tapes: None,
            ply: 1,
        }
    }

    /// ◆テープを追加するぜ☆（＾～＾）
    pub fn add_tape(&mut self, tape: CassetteTape, _app: &Application) {
        self.tapes.push(tape);
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.to_string()
    }
    pub fn set_file_name_without_extension(&mut self, file_name_without_extension: &str) {
        self.file_name = format!("{}.rtape", file_name_without_extension).to_string()
    }

    /// ◆スロットに差し込んでいるカセット・テープを抜くぜ☆（＾～＾）
    pub fn clear_tape_box(&mut self, app: &Application) {
        if app.is_debug() {
            app.comm
                .println(&format!("[#Clear tape box: {:?}]", self.role_as_slot));
        }
        self.tapes.clear();
        self.caret_of_tapes.clear_facing_right();
    }
    pub fn clear_tape_body(&mut self, app: &Application) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].clear_tape_body(&app)
        } else {
            panic!(
                "#seek_to_next: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// ◆次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、おわり☆（＾ｑ＾）
    ///
    /// # Returns
    ///
    /// (成功)
    pub fn seek_of_tapes(&mut self, app: &Application) -> bool {
        if self.tapes.is_empty() {
            // テープが無いなら。
            if app.is_debug() {
                app.comm.println("[#seek tape:テープが無い]");
            }
            return false;
        }

        if self.tapes.len() as i16 <= self.caret_of_tapes.step_in() {
            // 今回はテープの終わりなら。
            if app.is_debug() {
                app.comm.println("[#seek tape:今回はテープの終わり]");
            }
            return false;
        }

        self.awareness_of_tapes = Some(self.caret_of_tapes.go_to_next(&app));
        true
    }

    /// キャレットは必ず１つ進みます。
    /// 0 は、正の数とします。（マイナスゼロは無いです）
    /// Noneを返したら、オーバーフローしています。
    ///
    /// # Returns
    ///
    /// (taken overflow, move, note)
    pub fn seek_to_next_note(&mut self, app: &Application) -> (bool, ShogiMove, Option<ShogiNote>) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].seek_to_next(&app)
        } else {
            panic!(
                "#seek_to_next: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// フェーズ・チェンジか、エンド・オブ・テープを拾うまで進める☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (taken overflow, move)
    pub fn seek_1move(&mut self, app: &Application) -> (bool, ShogiMove) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].seek_1move(&app)
        } else {
            panic!(
                "#seek_1move: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    ///
    /// # Returns
    ///
    /// (taken overflow, move, note)
    pub fn seek_to_next_with_othre_caret(
        &self,
        caret: &mut Caret,
        app: &Application,
    ) -> (bool, ShogiMove, Option<ShogiNote>) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].seek_to_next_with_othre_caret(caret, &app)
        } else {
            panic!(
                "#seek_to_next_with_othre_caret: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn step_in_of_tape(&self) -> i16 {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].caret.step_in()
        } else {
            panic!(
                "#step_in_of_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// -と+方向の長さがある☆（＾～＾）
    pub fn get_current_tape_len(&self) -> ClosedInterval {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].get_span_caret_facing_outward()
        } else {
            panic!(
                "#get_current_tape_len: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn look_back_caret_to_opponent(&mut self, app: &Application) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index]
                .caret
                .look_back_to_opponent(&app);
        } else {
            panic!(
                "#look_back_caret_to_opponent: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn look_back_caret_to_negative(&mut self, app: &Application) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index]
                .caret
                .look_back_to_negative(&app);
        } else {
            panic!(
                "#look_back_caret_to_negative: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn look_back_caret_to_positive(&mut self, app: &Application) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index]
                .caret
                .look_back_to_positive(&app);
        } else {
            panic!(
                "#look_back_caret_to_positive: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// # Returns
    ///
    /// 削除したノート。
    pub fn delete_1note(&mut self, _app: &Application) -> Option<ShogiNote> {
        if let Some(ref awareness) = self.awareness_of_tapes {
            let tape = &mut self.tapes[awareness.index];

            let (new_tape, removed_note_opt) = tape.tracks.new_truncated_tape(&tape.caret);
            tape.tracks = new_tape;

            if let Some(removed_note) = removed_note_opt {
                Some(removed_note)
            } else {
                None
            }
        } else {
            panic!(
                "#delete_1note: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn push_note_to_positive_of_current_tape(&mut self, note: ShogiNote) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].tracks.positive_notes.push(note);
        } else {
            panic!(
                "#push_note_to_positive_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn push_note_to_negative_of_current_tape(&mut self, note: ShogiNote) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].tracks.negative_notes.push(note);
        } else {
            panic!(
                "#push_note_to_negative_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn set_note_to_current_tape(&mut self, caret_number: i16, note: ShogiNote) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            if -1 < caret_number {
                self.tapes[awareness.index].tracks.positive_notes[caret_number as usize] = note;
            } else {
                self.tapes[awareness.index].tracks.negative_notes
                    [get_index_from_caret_numbers(caret_number)] = note;
            }
        } else {
            panic!(
                "#set_note_to_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// 正の数では、（キャレット番号＋１）と、（要素の個数）は等しい。
    pub fn truncate_positive_of_current_tape(&mut self, len: usize) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index]
                .tracks
                .positive_notes
                .truncate(len);
        } else {
            panic!(
                "#truncate_positive_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// 負の数では、（キャレット番号の絶対値）と、（要素の個数）は等しい。
    pub fn truncate_negative_of_current_tape(&mut self, len: usize) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index]
                .tracks
                .negative_notes
                .truncate(len);
        } else {
            panic!(
                "#truncate_negative_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn is_facing_left_of_current_tape(&self) -> bool {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].caret.is_facing_left()
        } else {
            panic!(
                "#is_facing_left_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    // キャレットがピークを指しているか☆（＾～＾）
    pub fn is_peak_of_current_tape(&self) -> bool {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].is_peak()
        } else {
            panic!(
                "#is_peak_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }
    // キャレットが次、オーバーフローするか☆（＾～＾）
    pub fn is_before_caret_overflow_of_tape(&self) -> bool {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].is_before_caret_overflow()
        } else {
            panic!(
                "#is_before_caret_overflow_of_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn get_sign_of_current_tape(&self, board_size: BoardSize) -> (String, String) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].to_sign(board_size)
        } else {
            panic!(
                "#get_sign_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn get_tape_index(&self) -> Option<usize> {
        if let Some(ref awareness) = self.awareness_of_tapes {
            Some(awareness.index)
        } else {
            None
        }
    }

    pub fn to_rpm(&self, board_size: BoardSize) -> RpmTapeBox {
        let mut rbox = RpmTapeBox::new();

        for tape in &self.tapes {
            rbox.push(tape.to_rpm(board_size));
        }

        rbox
    }

    pub fn len_tapes(&self) -> usize {
        self.tapes.len()
    }

    pub fn is_empty_tapes(&self) -> bool {
        self.tapes.is_empty()
    }

    /// このテープを、テープ・フラグメント書式で書きだすぜ☆（＾～＾）
    pub fn write_current_tapes_fragment(&self, board_size: BoardSize, app: &Application) {
        if let Some(ref awareness) = self.awareness_of_tapes {
            self.tapes[awareness.index].write_tape_fragment(board_size, &app)
        } else {
            panic!(
                "#write_current_tapes_fragment: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// このテープ・ボックスを書きだすぜ☆（＾～＾）
    pub fn write_tape_box(&self, board_size: BoardSize, app: &Application) {
        let rpm_tape_box = self.to_rpm(board_size);
        rpm_tape_box.write(&self.file_name, &app);
    }

    /// このテープ・ボックスのデバッグ情報表示。人間向け。
    pub fn to_human_presentable(&self) -> String {
        if let Some(ref awareness) = self.awareness_of_tapes {
            use audio_compo::cassette_deck::Slot::*;
            format!(
                "[{}-Box: File: '{}', Tapes: {}, Tape index: {}]",
                match self.role_as_slot {
                    Training => "T",
                    Learning => "Learnig",
                }
                .to_string(),
                self.file_name,
                self.tapes.len(),
                awareness.index
            )
            .to_string()
        } else {
            format!("[Box: File: '{}', I have not seek a tape]", self.file_name).to_string()
        }
    }

    /// 現在聴いているテープのデバッグ情報表示。人間向け。
    pub fn to_human_presentable_of_current_tape(
        &self,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        if let Some(ref awareness) = self.awareness_of_tapes {
            use audio_compo::cassette_deck::Slot::*;
            format!(
                "[{}-Box: Tape index: {}, Tape: {}]",
                match self.role_as_slot {
                    Training => "T",
                    Learning => "Learnig",
                }
                .to_string(),
                awareness.index,
                self.tapes[awareness.index].to_human_presentable(board_size, &app)
            )
            .to_string()
        } else {
            "[Box: I have not seek a tape]".to_string()
        }
    }

    /// 現在聴いているテープのキャレットのデバッグ情報表示。人間向け。
    pub fn to_human_presentable_of_caret_of_current_tape(&self, app: &Application) -> String {
        if let Some(ref awareness) = self.awareness_of_tapes {
            format!(
                "[Box: Caret: {}]",
                self.tapes[awareness.index].caret.to_human_presentable(&app)
            )
            .to_string()
        } else {
            "[Box: I have not seek a tape]".to_string()
        }
    }
}
