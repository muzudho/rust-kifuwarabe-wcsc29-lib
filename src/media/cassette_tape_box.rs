extern crate rand;
use audio_compo::cassette_deck::Slot;
use media::cassette_tape::*;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sheet_music_format::tape_label::TapeLabel;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use std::*;
use studio::application::Application;
use studio::board_size::*;
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
    awareness_of_tapes: Awareness,

    /// 何も指していない状態で 1。
    /// TODO 本将棋の大橋流の最初の玉は Ply=-39 にしたい。
    /// トレーニング・テープの 手目。
    pub ply: i16,
}
impl CassetteTapeBox {
    // ###############
    // # Constructor #
    // ###############

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
            awareness_of_tapes: Awareness::new(),
            ply: 1,
        }
    }

    // #####
    // # A #
    // #####

    /// ◆テープを追加するぜ☆（＾～＾）
    pub fn add_tape(&mut self, tape: CassetteTape, _app: &Application) {
        self.tapes.push(tape);
    }

    // #####
    // # C #
    // #####

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
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].clear_tape_body(&app)
        } else {
            panic!(
                "#Clear tape body: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    // #####
    // # D #
    // #####

    /// # Returns
    ///
    /// 削除したノート。
    pub fn delete_1note(&mut self, app: &Application) -> Option<ShogiNote> {
        if let Some(index) = self.awareness_of_tapes.index {
            let tape = &mut self.tapes[index];

            let (new_tape, removed_note_opt) = tape.tracks.new_truncated_tape(&tape.caret, &app);
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

    // #####
    // # G #
    // #####

    pub fn get_file_name(&self) -> String {
        self.file_name.to_string()
    }

    /// -と+方向の長さがある☆（＾～＾）
    pub fn get_current_tape_len(&self) -> ClosedInterval {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].get_span_caret_facing_outward()
        } else {
            panic!(
                "#get_current_tape_len: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn get_sign_of_current_tape(&self, board_size: BoardSize) -> (String, String) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].to_sign(board_size)
        } else {
            panic!(
                "#get_sign_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn get_tape_index(&self) -> Option<usize> {
        if let Some(index) = self.awareness_of_tapes.index {
            Some(index)
        } else {
            None
        }
    }

    // #####
    // # I #
    // #####

    pub fn insert_note(&mut self, note: ShogiNote, board_size: BoardSize, app: &Application) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].insert_note(note, board_size, &app);
        } else {
            panic!(
                "#push_note_to_positive_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn is_facing_left_of_current_tape(&self) -> bool {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].caret.is_facing_left()
        } else {
            panic!(
                "#is_facing_left_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    // キャレットがピークを指しているか☆（＾～＾）
    pub fn is_peak_of_current_tape(&self) -> bool {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].is_peak()
        } else {
            panic!(
                "#is_peak_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }
    // キャレットが次、オーバーフローするか☆（＾～＾）
    pub fn is_before_caret_overflow_of_tape(&self) -> bool {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].is_before_caret_overflow()
        } else {
            panic!(
                "#is_before_caret_overflow_of_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn is_empty_tapes(&self) -> bool {
        self.tapes.is_empty()
    }

    pub fn is_none_current_tape(&self) -> bool {
        None == self.awareness_of_tapes.index
    }

    // #####
    // # L #
    // #####

    pub fn look_back_caret(&mut self, app: &Application) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].caret.look_back(&app);
        } else {
            panic!(
                "#look_back_caret_to_opponent: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn turn_caret_towards_negative_infinity(&mut self, app: &Application) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].caret.turn_towards_negative_infinity(&app);
        } else {
            panic!(
                "#turn_caret_towards_negative_infinity: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn turn_caret_towards_positive_infinity(&mut self, app: &Application) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].caret.turn_towards_positive_infinity(&app);
        } else {
            panic!(
                "#turn_caret_towards_positive_infinity: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn len_tapes(&self) -> usize {
        self.tapes.len()
    }

    // #####
    // # P #
    // #####

    /// 正の方のテープの末端にノートを追加。
    pub fn push_note(&mut self, note: ShogiNote) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].push_note(note)
        } else {
            panic!(
                "#Box.Push note: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// 正の方のテープの末端にノートを追加。
    pub fn pop_note(&mut self) -> Option<ShogiNote> {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].pop_note()
        } else {
            panic!(
                "#Box.Pop note: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /*
    pub fn push_note_to_positive_of_current_tape(&mut self, note: ShogiNote) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].tracks.positive_notes.push(note);
        } else {
            panic!(
                "#push_note_to_positive_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }
    pub fn push_note_to_negative_of_current_tape(&mut self, note: ShogiNote) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].tracks.negative_notes.push(note);
        } else {
            panic!(
                "#push_note_to_negative_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }
    */

    // #####
    // # S #
    // #####

    pub fn set_file_name_without_extension(&mut self, file_name_without_extension: &str) {
        self.file_name = format!("{}-tape-box.json", file_name_without_extension).to_string()
    }

    /// テープのラベルを書く。
    pub fn set_label_of_tape(&mut self, label: &TapeLabel) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].set_label(label);
        } else {
            panic!(
                "#set_label_of_tape: Please seek tapes. It is none. Slot: '{:?}'.",
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
        if app.is_debug() {
            app.comm.println("[#Box.seek_of_tapes: 開始]");
        }

        if self.tapes.is_empty() {
            // テープが無いなら。
            if app.is_debug() {
                app.comm.println("[#Seek of tapes:テープが無い]");
            }
            return false;
        }

        if self.tapes.len() as i16 <= self.caret_of_tapes.step_in() {
            // 今回はテープの終わりなら。
            if app.is_debug() {
                app.comm.println("[#Seek of tapes:今回はテープの終わり]");
            }
            return false;
        }

        self.awareness_of_tapes = self.caret_of_tapes.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Seek of tapes: {}]",
                self.awareness_of_tapes.to_human_presentable()
            ));
        }

        if app.is_debug() {
            app.comm.println("[#Box.seek_of_tapes: 終了]");
        }

        true
    }

    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note(&mut self, app: &Application) -> (bool, Awareness, Option<ShogiNote>) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].seek_a_note(&app)
        } else {
            panic!(
                "#seek_to_next: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// # Returns
    ///
    /// (taken overflow, move)
    pub fn skip_a_move(&mut self, app: &Application) -> (bool, ShogiMove) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].skip_a_move(&app)
        } else {
            panic!(
                "#skip_a_move: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    ///
    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note_with_othre_caret(
        &self,
        caret: &mut Caret,
        app: &Application,
    ) -> (bool, Awareness, Option<ShogiNote>) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].seek_a_note_with_othre_caret(caret, &app)
        } else {
            panic!(
                "#seek_next_note_with_othre_caret: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    pub fn step_in_of_tape(&self) -> i16 {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].caret.step_in()
        } else {
            panic!(
                "#step_in_of_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }

    /*
    pub fn set_note_to_current_tape(&mut self, caret_number: i16, note: ShogiNote) {
        if let Some(index) = self.awareness_of_tapes.index {
            if -1 < caret_number {
                self.tapes[index].tracks.positive_notes[caret_number as usize] = note;
            } else {
                self.tapes[index].tracks.negative_notes
                    [get_index_from_caret_numbers(caret_number)] = note;
            }
        } else {
            panic!(
                "#set_note_to_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }
    */

    // #####
    // # T #
    // #####

    /*
    /// 正の数では、（キャレット番号＋１）と、（要素の個数）は等しい。
    pub fn truncate_positive_of_current_tape(&mut self, len: usize) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].tracks.positive_notes.truncate(len);
        } else {
            panic!(
                "#truncate_positive_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }
    /// 負の数では、（キャレット番号の絶対値）と、（要素の個数）は等しい。
    pub fn truncate_negative_of_current_tape(&mut self, len: usize) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].tracks.negative_notes.truncate(len);
        } else {
            panic!(
                "#truncate_negative_of_current_tape: Please seek tapes. It is none. Slot: '{:?}'.",
                self.role_as_slot
            );
        }
    }
    */

    pub fn to_rpm(&self, board_size: BoardSize) -> RpmTapeBox {
        let mut tape_box = RpmTapeBox::new();

        for tape in &self.tapes {
            tape_box.push(tape.to_rpm(board_size));
        }

        tape_box
    }

    /// このテープ・ボックスのデバッグ情報表示。人間向け。
    pub fn to_human_presentable(&self) -> String {
        if let Some(index) = self.awareness_of_tapes.index {
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
                index
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
        if let Some(index) = self.awareness_of_tapes.index {
            use audio_compo::cassette_deck::Slot::*;
            format!(
                "[{}-Box: Tape index: {}, Tape: {}]",
                match self.role_as_slot {
                    Training => "T",
                    Learning => "Learnig",
                }
                .to_string(),
                index,
                self.tapes[index].to_human_presentable(board_size, &app)
            )
            .to_string()
        } else {
            "[Box: I have not seek a tape]".to_string()
        }
    }

    /// 現在聴いているテープのキャレットのデバッグ情報表示。人間向け。
    pub fn to_human_presentable_of_caret(&self, app: &Application) -> String {
        if let Some(index) = self.awareness_of_tapes.index {
            format!(
                "{:?}-box {}",
                self.role_as_slot,
                self.tapes[index].caret.to_human_presentable(&app)
            )
            .to_string()
        } else {
            "[#Tape-box: I have not seek a tape]".to_string()
        }
    }

    // #####
    // # W #
    // #####

    /// このテープを、テープ・フラグメント書式で書きだすぜ☆（＾～＾）
    pub fn write_current_tapes_fragment(&self, board_size: BoardSize, app: &Application) {
        if let Some(index) = self.awareness_of_tapes.index {
            self.tapes[index].write_tape_fragment(board_size, &app)
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
}
