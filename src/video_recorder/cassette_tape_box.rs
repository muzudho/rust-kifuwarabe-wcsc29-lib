extern crate rand;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sound::shogi_note::ShogiNote;
use std::*;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::Caret;
use studio::communication::Communication;
use video_recorder::cassette_tape::*;

/// 保存したいときは RPM棋譜 に変換して、そっちで保存しろだぜ☆（＾～＾）
pub struct CassetteTapeBox {
    file: String,

    /// イテレーターを使いたいので public にしてある。
    pub tapes: Vec<CassetteTape>,

    /// インデックス。現在聴いているテープを指す。聴いていないときは None。最初のテープは 0番。全部聞き終わると、最後のテープの次の存在しない番号を指す。
    listening_tape_index: Option<usize>,
}
impl CassetteTapeBox {
    /// 他にも JSONファイルを読み込んで、あっちから このオブジェクトを作る方法もある。
    pub fn new_empty(app: &Application) -> Self {
        CassetteTapeBox {
            file: RpmTapeBox::create_file_full_name(&app.kw29_conf),
            tapes: Vec::new(),
            listening_tape_index: None,
        }
    }

    pub fn from_file(file_name: &str, board_size: BoardSize, app: &Application) -> Self {
        let rpm_tape_box = RpmTapeBox::from_box_file(&file_name, &app);
        rpm_tape_box.to_object(file_name, board_size, &app)
    }

    /// スロットに差し込んでいるカセット・テープを抜くぜ☆（＾～＾）
    pub fn eject(&mut self) {
        self.listening_tape_index = None;
    }

    /// 次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、おわり☆（＾ｑ＾）
    ///
    /// # Returns
    ///
    /// (成功)
    pub fn change_next_if_it_exists(&mut self, _app: &Application) -> bool {
        if let Some(tape_index) = self.listening_tape_index {
            self.listening_tape_index = Some(tape_index + 1);
            tape_index + 1 < self.tapes.len()
        } else if self.tapes.is_empty() {
            false
        } else {
            self.listening_tape_index = Some(0);
            true
        }
    }

    /// ピークに、新品の空テープを追加してそれを聴くぜ☆（＾ｑ＾）
    pub fn change_brandnew(&mut self, app: &Application) {
        let brandnew = CassetteTape::new_facing_right(&app);
        self.listening_tape_index = Some(self.tapes.len());
        self.tapes.push(brandnew);
    }

    /// ピークに、指定のテープを追加してそれを聴くぜ☆（＾～＾）
    /// トレーニング・テープを聴くときと、ラーニング・テープをJSONを出力するときに使う☆（＾～＾）
    pub fn change_with_tape(&mut self, tape: CassetteTape) {
        self.listening_tape_index = Some(self.tapes.len());
        self.tapes.push(tape);
    }

    /// # Returns
    ///
    /// (キャレット番地, 1ノート)
    pub fn go_1note_forcely(&mut self, app: &Application) -> (i16, Option<ShogiNote>) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].go_1note_forcely(&app.comm)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn turn_caret_to_opponent(&mut self) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.turn_to_opponent();
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn turn_caret_to_negative(&mut self) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.turn_to_negative();
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn turn_caret_to_positive(&mut self) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.turn_to_positive();
        } else {
            panic!("Please choice listening tape.");
        }
    }

    /// # Returns
    ///
    /// 削除したノート。
    pub fn delete_1note(&mut self, _app: &Application) -> Option<ShogiNote> {
        if let Some(tape_index) = self.listening_tape_index {
            let tape = &mut self.tapes[tape_index];

            let (new_tape, removed_note_opt) = tape.tracks.new_truncated_tape(&tape.caret);
            tape.tracks = new_tape;

            if let Some(removed_note) = removed_note_opt {
                Some(removed_note)
            } else {
                None
            }
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn push_note_to_positive_of_current_tape(&mut self, note: ShogiNote) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].tracks.positive_notes.push(note);
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn push_note_to_negative_of_current_tape(&mut self, note: ShogiNote) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].tracks.negative_notes.push(note);
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn set_note_to_positive_of_current_tape(&mut self, index: usize, note: ShogiNote) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].tracks.positive_notes[index] = note;
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn set_note_to_negative_of_current_tape(&mut self, index: usize, note: ShogiNote) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].tracks.negative_notes[index] = note;
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn truncate_positive_of_current_tape(&mut self, len: usize) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].tracks.positive_notes.truncate(len);
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn truncate_negative_of_current_tape(&mut self, len: usize) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].tracks.negative_notes.truncate(len);
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn is_facing_left_of_current_tape(&self) -> bool {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.is_facing_left()
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn is_peak_of_current_tape(&self) -> bool {
        if self.is_facing_left_of_current_tape() {
            self.is_negative_peak_of_current_tape()
        } else {
            self.is_positive_peak_of_current_tape()
        }
    }

    pub fn is_positive_peak_of_current_tape(&self) -> bool {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index]
                .caret
                .equals(self.tapes[tape_index].get_positive_peak_caret())
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn is_negative_peak_of_current_tape(&self) -> bool {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index]
                .caret
                .equals(self.tapes[tape_index].get_negative_peak_caret())
        } else {
            panic!("Please choice listening tape.");
        }
    }

    /// TODO このメソッドは廃止したい。
    ///
    /// 配列のインデックスに変換します。
    /// 負の配列では 数を 0 側に 1 つ寄せます。
    ///
    /// # Returns
    ///
    /// (is_positive, index, caret_number)
    pub fn get_caret_index_of_current_tape_obsoluted(&self) -> (bool, usize, i16) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.to_index_obsoluted()
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn get_sign_of_current_tape(&self, board_size: BoardSize) -> (String, String) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].to_sign(board_size)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    pub fn go_1note_forcely_with_othre_caret(
        &self,
        caret: &mut Caret,
        comm: &Communication,
    ) -> (i16, Option<ShogiNote>) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].go_1note_forcely_with_othre_caret(caret, &comm)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn go_caret_to_next(&mut self, app: &Application) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.go_next(&app.comm);
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn get_file_name(&self) -> String {
        self.file.to_string()
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
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].write_tape_fragment(board_size, &app.comm)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    /// このテープ・ボックスを書きだすぜ☆（＾～＾）
    pub fn write_tape_box(&self, board_size: BoardSize, app: &Application) {
        let rpm_tape_box = self.to_rpm(board_size);
        rpm_tape_box.write(&self.file, &app.comm);
    }

    /// このテープ・ボックスのデバッグ情報表示。人間向け。
    pub fn to_human_presentable(&self) -> String {
        if let Some(tape_index) = self.listening_tape_index {
            format!(
                "File: '{}', Tapes: {}, Tape index: {}.",
                self.file,
                self.tapes.len(),
                tape_index
            )
            .to_string()
        } else {
            format!("File: '{}', I have not selected a tape.", self.file).to_string()
        }
    }

    /// 現在聴いているテープのデバッグ情報表示。人間向け。
    pub fn to_human_presentable_of_current_tape(&self, board_size: BoardSize) -> String {
        if let Some(tape_index) = self.listening_tape_index {
            format!(
                "File: '{}', Tape index: {}, Tape: {}.",
                self.file,
                tape_index,
                self.tapes[tape_index].to_human_presentable(board_size)
            )
            .to_string()
        } else {
            format!("File: '{}', I have not selected a tape.", self.file).to_string()
        }
    }
}
