extern crate rand;
use audio_compo::cassette_deck::Slot;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use std::*;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::get_index_from_caret_numbers;
use studio::common::caret::Caret;
use studio::common::closed_interval::ClosedInterval;
use video_tape_model::cassette_tape::*;

/// 保存したいときは RPM棋譜 に変換して、そっちで保存しろだぜ☆（＾～＾）
pub struct CassetteTapeBox {
    // このテープボックスの役割。
    role_as_slot: Slot,

    file: String,

    /// イテレーターを使いたいので public にしてある。
    pub tapes: Vec<CassetteTape>,

    /// インデックス。現在聴いているテープを指す。聴いていないときは None。最初のテープは 0番。全部聞き終わると、最後のテープの次の存在しない番号を指す。
    listening_tape_index: Option<usize>,
}
impl CassetteTapeBox {
    /// 他にも JSONファイルを読み込んで、あっちから このオブジェクトを作る方法もある。
    pub fn new_empty_tape_box(slot: Slot, app: &Application) -> Self {
        CassetteTapeBox {
            role_as_slot: slot,
            file: RpmTapeBox::create_file_full_name(&app.kw29_conf, &app),
            tapes: Vec::new(),
            listening_tape_index: None,
        }
    }

    /// ファイル名付きで、テープ・ボックスを作成します。
    pub fn new_with_tape_box_file_name_without_extension(
        slot: Slot,
        file_name_without_extension: &str,
        app: &Application,
    ) -> Self {
        if app.is_debug() {
            app.comm
                .println("[#cassette_tape_box.new_with_tape_box_file_name]");
        }

        let mut tapes_vec = Vec::new();
        tapes_vec.push(CassetteTape::new_facing_right_with_file(
            format!("{}.tapesfrag", file_name_without_extension).to_string(),
        ));

        CassetteTapeBox {
            role_as_slot: slot,
            file: format!("{}.rtape", file_name_without_extension).to_string(),
            tapes: tapes_vec,
            listening_tape_index: Some(0),
        }
    }

    /// ファイルを読み込んで、テープ・ボックスを作成します。
    pub fn from_tape_box_file(file_name: &str, board_size: BoardSize, app: &Application) -> Self {
        if app.is_debug() {
            app.comm.println("[#cassette_tape_box.from_tape_box_file]");
        }

        let rpm_tape_box = RpmTapeBox::from_box_file(&file_name, &app);
        rpm_tape_box.to_training_object(board_size, &app)
    }

    fn set_tape_index(&mut self, index_opt: Option<usize>, app: &Application) {
        if app.is_debug() {
            app.comm.println(&format!(
                "[#set_tape_index: {}, Slot: '{:?}']",
                if let Some(index) = index_opt {
                    index.to_string()
                } else {
                    "None".to_string()
                },
                self.role_as_slot
            ));
        }

        self.listening_tape_index = index_opt;
    }

    pub fn back_to_first_tape(&mut self, app: &Application) {
        if self.tapes.is_empty() {
            if app.is_debug() {
                app.comm.println("[#back_to_first_tape: Tapes is empty.]");
            }
            self.set_tape_index(None, &app);
        } else {
            if app.is_debug() {
                app.comm.println("[#back_to_first_tape: Set 0.]");
            }
            self.set_tape_index(Some(0), &app);
        }
    }

    /// スロットに差し込んでいるカセット・テープを抜くぜ☆（＾～＾）
    pub fn eject(&mut self, app: &Application) {
        if app.is_debug() {
            app.comm
                .println(&format!("[#Eject: {:?}]", self.role_as_slot));
        }
        self.set_tape_index(None, &app);
    }

    /// 次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、おわり☆（＾ｑ＾）
    ///
    /// # Returns
    ///
    /// (成功)
    pub fn change_tape_if_next_exists(&mut self, app: &Application) -> bool {
        if let Some(tape_index) = self.listening_tape_index {
            self.set_tape_index(Some(tape_index + 1), &app);
            tape_index + 1 < self.tapes.len()
        } else if self.tapes.is_empty() {
            false
        } else {
            self.set_tape_index(Some(0), &app);
            true
        }
    }

    /// ピークに、新品の空テープを追加してそれを聴くぜ☆（＾ｑ＾）
    pub fn change_tape_brandnew(&mut self, app: &Application) {
        let brandnew = CassetteTape::new_facing_right(&app);
        let index = self.tapes.len();
        self.set_tape_index(Some(index), &app);
        self.tapes.push(brandnew);
    }

    /// ピークに、指定のテープを追加してそれを聴くぜ☆（＾～＾）
    /// トレーニング・テープを聴くときと、ラーニング・テープをJSONを出力するときに使う☆（＾～＾）
    pub fn change_tape_as_name(&mut self, tape: CassetteTape, app: &Application) {
        let index = self.tapes.len();
        self.set_tape_index(Some(index), &app);
        self.tapes.push(tape);
    }

    /// フェーズ・チェンジか、エンド・オブ・テープを拾うまで進める☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (taken overflow, move)
    pub fn seek_1move(&mut self, app: &Application) -> (bool, ShogiMove) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].seek_1move(&app)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    /// キャレットは必ず１つ進みます。
    /// 0 は、正の数とします。（マイナスゼロは無いです）
    /// Noneを返したら、オーバーフローしています。
    ///
    /// # Returns
    ///
    /// (taken overflow, move, note)
    pub fn seek_to_next(&mut self, app: &Application) -> (bool, ShogiMove, Option<ShogiNote>) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].seek_to_next(&app)
        } else {
            panic!(
                "Please change tape. It is none. Slot: '{:?}'.",
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
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].seek_to_next_with_othre_caret(caret, &app)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn step_in(&self, app: &Application) -> i16 {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.step_in(&app.comm)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    /// -と+方向の長さがある☆（＾～＾）
    pub fn get_current_tape_len(&self) -> ClosedInterval {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].get_span_caret_facing_outward()
        } else {
            panic!("Please choice listening tape.");
        }
    }

    pub fn look_back_caret_to_opponent(&mut self, app: &Application) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.look_back_to_opponent(&app);
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn look_back_caret_to_negative(&mut self, app: &Application) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.look_back_to_negative(&app);
        } else {
            panic!("Please choice listening tape.");
        }
    }
    pub fn look_back_caret_to_positive(&mut self, app: &Application) {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].caret.look_back_to_positive(&app);
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

    pub fn set_note_to_current_tape(&mut self, caret_number: i16, note: ShogiNote) {
        if let Some(tape_index) = self.listening_tape_index {
            if -1 < caret_number {
                self.tapes[tape_index].tracks.positive_notes[caret_number as usize] = note;
            } else {
                self.tapes[tape_index].tracks.negative_notes
                    [get_index_from_caret_numbers(caret_number)] = note;
            }
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

    // キャレットがピークを指しているか☆（＾～＾）
    pub fn is_peak(&self, app: &Application) -> bool {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].is_peak(&app)
        } else {
            panic!("Please choice listening tape.");
        }
    }
    // キャレットが次、オーバーフローするか☆（＾～＾）
    pub fn is_before_caret_overflow(&self, app: &Application) -> bool {
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].is_before_caret_overflow(&app)
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

    pub fn get_file_name(&self) -> String {
        self.file.to_string()
    }

    pub fn get_tape_index(&self) -> Option<usize> {
        self.listening_tape_index
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
        if let Some(tape_index) = self.listening_tape_index {
            self.tapes[tape_index].write_tape_fragment(board_size, &app)
        } else {
            panic!("Please choice listening tape.");
        }
    }

    /// このテープ・ボックスを書きだすぜ☆（＾～＾）
    pub fn write_tape_box(&self, board_size: BoardSize, app: &Application) {
        let rpm_tape_box = self.to_rpm(board_size);
        rpm_tape_box.write(&self.file, &app);
    }

    /// このテープ・ボックスのデバッグ情報表示。人間向け。
    pub fn to_human_presentable(&self) -> String {
        if let Some(tape_index) = self.listening_tape_index {
            use audio_compo::cassette_deck::Slot::*;
            format!(
                "[{}-Box: File: '{}', Tapes: {}, Tape index: {}]",
                match self.role_as_slot {
                    Training => "T",
                    Learning => "Learnig",
                }
                .to_string(),
                self.file,
                self.tapes.len(),
                tape_index
            )
            .to_string()
        } else {
            format!("[Box: File: '{}', I have not selected a tape]", self.file).to_string()
        }
    }

    /// 現在聴いているテープのデバッグ情報表示。人間向け。
    pub fn to_human_presentable_of_current_tape(
        &self,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        if let Some(tape_index) = self.listening_tape_index {
            use audio_compo::cassette_deck::Slot::*;
            format!(
                "[{}-Box: Tape index: {}, Tape: {}]",
                match self.role_as_slot {
                    Training => "T",
                    Learning => "Learnig",
                }
                .to_string(),
                tape_index,
                self.tapes[tape_index].to_human_presentable(board_size, &app)
            )
            .to_string()
        } else {
            "[Box: I have not selected a tape]".to_string()
        }
    }

    /// 現在聴いているテープのキャレットのデバッグ情報表示。人間向け。
    pub fn to_human_presentable_of_caret_of_current_tape(&self, app: &Application) -> String {
        if let Some(tape_index) = self.listening_tape_index {
            format!(
                "[Box: Caret: {}]",
                self.tapes[tape_index].caret.to_human_presentable(&app)
            )
            .to_string()
        } else {
            "[Box: I have not selected a tape]".to_string()
        }
    }
}
