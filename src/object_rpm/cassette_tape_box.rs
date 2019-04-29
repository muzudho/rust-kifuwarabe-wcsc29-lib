extern crate rand;
use application::Application;
use board_size::*;
use common::caret::Caret;
use communication::Communication;
use kifu_rpm::rpm_tape_box::*;
use object_rpm::cassette_tape::*;
use object_rpm::shogi_note::ShogiNote;
use std::*;

/// 保存したいときは RPM棋譜 に変換して、そっちで保存しろだぜ☆（＾～＾）
pub struct CassetteTapeBox {
    file: String,

    /// イテレーターを使いたいので public にしてある。
    pub tapes: Vec<CassetteTape>,

    tape_index: usize,
}
impl CassetteTapeBox {
    /// 他にも JSONファイルを読み込んで、あっちから このオブジェクトを作る方法もある。
    pub fn new_empty(app: &Application) -> Self {
        CassetteTapeBox {
            file: RpmTapeBox::create_file_full_name(&app.kw29_conf),
            tapes: Vec::new(),
            tape_index: 0,
        }
    }

    pub fn from_file(file_name: &str, board_size: BoardSize, app: &Application) -> Self {
        let rpm_tape_box = RpmTapeBox::from_box_file(&file_name, &app);
        rpm_tape_box.to_object(file_name, board_size, &app)
    }

    /// 次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、おわり☆（＾ｑ＾）
    ///
    /// # Returns
    ///
    /// (成功)
    pub fn change_next_if_it_exists(&mut self, app: &Application) -> bool {
        self.tape_index += 1;
        if self.tape_index < self.tapes.len() {
            true
        } else {
            false
        }
    }

    /*
    /// 次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、新品のテープを追加するぜ☆（＾ｑ＾）
    pub fn change_next_create(&mut self, app: &Application) {
        self.tape_index += 1;
        if self.tapes.len() <= self.tape_index {
            self.change_brandnew(&app);
        }
    }
    */

    /// 新品のテープを追加するぜ☆（＾ｑ＾）
    pub fn change_brandnew(&mut self, app: &Application) {
        let brandnew = CassetteTape::new_facing_right(&app);
        self.tapes.push(brandnew);
        self.tape_index = self.tapes.len() - 1;
    }

    /// テープを追加するぜ☆（＾～＾）　トレーニング・テープで使うと思うぜ☆（＾～＾）
    pub fn change_with_training_tape(&mut self, training_tape: CassetteTape) {
        self.tapes.push(training_tape);
        self.tape_index = self.tapes.len() - 1;
    }

    pub fn go_1note_forcely(&mut self, app: &Application) -> Option<ShogiNote> {
        self.tapes[self.tape_index].go_1note_forcely(&app.comm)
    }

    pub fn turn_caret_to_opponent(&mut self) {
        self.tapes[self.tape_index].caret.turn_to_opponent();
    }
    pub fn turn_caret_to_negative(&mut self) {
        self.tapes[self.tape_index].caret.turn_to_negative();
    }
    pub fn turn_caret_to_positive(&mut self) {
        self.tapes[self.tape_index].caret.turn_to_positive();
    }

    /// # Returns
    ///
    /// 削除したノート。
    pub fn delete_1note(&mut self, _app: &Application) -> Option<ShogiNote> {
        let tape = &mut self.tapes[self.tape_index];

        let (new_tape, removed_note_opt) = tape.tracks.new_truncated_tape(&tape.caret);
        tape.tracks = new_tape;

        if let Some(removed_note) = removed_note_opt {
            Some(removed_note)
        } else {
            None
        }
    }

    pub fn push_note_to_positive_of_current_tape(&mut self, note: ShogiNote) {
        self.tapes[self.tape_index].tracks.positive_notes.push(note);
    }
    pub fn push_note_to_negative_of_current_tape(&mut self, note: ShogiNote) {
        self.tapes[self.tape_index].tracks.negative_notes.push(note);
    }

    pub fn set_note_to_positive_of_current_tape(&mut self, index: usize, note: ShogiNote) {
        self.tapes[self.tape_index].tracks.positive_notes[index] = note;
    }
    pub fn set_note_to_negative_of_current_tape(&mut self, index: usize, note: ShogiNote) {
        self.tapes[self.tape_index].tracks.negative_notes[index] = note;
    }

    pub fn truncate_positive_of_current_tape(&mut self, len: usize) {
        self.tapes[self.tape_index]
            .tracks
            .positive_notes
            .truncate(len);
    }
    pub fn truncate_negative_of_current_tape(&mut self, len: usize) {
        self.tapes[self.tape_index]
            .tracks
            .negative_notes
            .truncate(len);
    }

    pub fn is_facing_left_of_current_tape(&self) -> bool {
        self.tapes[self.tape_index].caret.is_facing_left()
    }

    pub fn is_peak_of_current_tape(&self) -> bool {
        if self.is_facing_left_of_current_tape() {
            self.is_negative_peak_of_current_tape()
        } else {
            self.is_positive_peak_of_current_tape()
        }
    }

    pub fn is_positive_peak_of_current_tape(&self) -> bool {
        self.tapes[self.tape_index]
            .caret
            .equals(self.tapes[self.tape_index].get_positive_peak_caret())
    }
    pub fn is_negative_peak_of_current_tape(&self) -> bool {
        self.tapes[self.tape_index]
            .caret
            .equals(self.tapes[self.tape_index].get_negative_peak_caret())
    }

    /// 配列のインデックスに変換します。
    /// 負の配列では 数を 0 側に 1 つ寄せます。
    ///
    /// # Returns
    ///
    /// (is_positive, index, caret_number)
    pub fn get_caret_index_of_current_tape(&self) -> (bool, usize, i16) {
        self.tapes[self.tape_index].caret.to_index()
    }

    pub fn get_sign_of_current_tape(&self, board_size: BoardSize) -> (String, String) {
        self.tapes[self.tape_index].to_sign(board_size)
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    pub fn go_1note_forcely_with_othre_caret(
        &self,
        caret: &mut Caret,
        comm: &Communication,
    ) -> Option<ShogiNote> {
        self.tapes[self.tape_index].go_1note_forcely_with_othre_caret(caret, &comm)
    }

    pub fn go_caret_to_next(&mut self, app: &Application) {
        self.tapes[self.tape_index].caret.go_next(&app.comm);
    }

    pub fn get_file_name(&self) -> String {
        self.file.to_string()
    }

    /// トレーニング、ラーニングに関わらず、テープを追加するぜ☆（＾～＾）
    pub fn push_tape(&mut self, tape: CassetteTape) {
        self.tapes.push(tape);
        self.tape_index = self.tapes.len() - 1;
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
    pub fn write_tape_fragment_of_current_tape(&self, board_size: BoardSize, app: &Application) {
        self.tapes[self.tape_index].write_tape_fragment(board_size, &app.comm)
    }

    /// このテープ・ボックスを書きだすぜ☆（＾～＾）
    pub fn write_tape_box(&self, board_size: BoardSize, app: &Application) {
        let rpm_tape_box = self.to_rpm(board_size);
        rpm_tape_box.write(&self.file, &app.comm);
    }

    /// デバッグ調査用の表示。
    pub fn to_human_presentable(&self) -> String {
        format!(
            "TpIx{}:Cu{}",
            self.tape_index,
            self.tapes[self.tape_index].caret.to_human_presentable()
        )
        .to_string()
    }
}
