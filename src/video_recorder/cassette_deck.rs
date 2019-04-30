use human::human_interface::*;
use instrument::position::*;
use sound::shogi_note::ShogiNote;
use studio::application::Application;
use studio::board_size::BoardSize;
use studio::common::caret::get_index_from_caret_numbers;
use studio::common::closed_interval::ClosedInterval;
use video_recorder::cassette_tape_box::*;

#[derive(Clone, Copy, Debug)]
pub enum Slot {
    /// トレーニング。
    Training,
    /// ラーニング。
    Learning,
}

pub struct CassetteSlot {
    /// このスロットの役割。デバッグ表示用。
    slot_as_role: Slot,

    /// 何も指していない状態で 1。
    /// TODO 本将棋の大橋流の最初の玉は Ply=-39 にしたい。
    /// トレーニング・テープの 手目。
    pub ply: i16,

    /// テープ・ボックス。
    /// 選んでいるテープを示すインデックスを持つ。
    pub tape_box: Option<CassetteTapeBox>,
}
impl CassetteSlot {
    pub fn new_t() -> Self {
        CassetteSlot {
            slot_as_role: Slot::Training,
            ply: 1,
            tape_box: None,
        }
    }

    pub fn new_as_learning(app: &Application) -> Self {
        CassetteSlot {
            slot_as_role: Slot::Learning,
            ply: 1,
            tape_box: Some(CassetteTapeBox::new_empty(Slot::Learning, &app)),
        }
    }

    pub fn to_human_presentable(&self) -> String {
        format!(
            "[Slot: {:?}, Ply: {}, Exists: {}]",
            self.slot_as_role,
            self.ply,
            if let Some(ref _tape_box) = self.tape_box {
                "Exists"
            } else {
                "None"
            }
            .to_string()
        )
    }
}

/// カセット・デッキ。
pub struct CassetteDeck {
    // カセットのスロット。トレーニング、ラーニングの順。
    pub slots: [CassetteSlot; 2],
}
impl CassetteDeck {
    /// 新規作成と、ラーニング・テープの交換☆（＾～＾）
    pub fn new_change(
        training_tape_box_opt: Option<CassetteTapeBox>,
        board_size: BoardSize,
        app: &Application,
    ) -> Self {
        let mut brandnew = CassetteDeck {
            slots: [CassetteSlot::new_t(), CassetteSlot::new_as_learning(&app)],
        };

        brandnew.change(training_tape_box_opt, board_size, &app);

        brandnew
    }

    // JSONファイルを元に トレーニング・テープをオブジェクト化して、デッキに差し込むぜ☆（＾～＾）
    pub fn change_with_tape_box_file(
        &mut self,
        file_name: &str,
        board_size: BoardSize,
        app: &Application,
    ) {
        let training_tape_box = CassetteTapeBox::from_training_file(&file_name, board_size, &app);
        self.change(Some(training_tape_box), board_size, &app);
    }

    /// トレーニング・テープを交換するぜ☆（＾～＾）
    ///
    /// 棋譜読取の際は、テープ・ボックスの中にテープ・インデックスが指定されてある☆（＾～＾）
    pub fn change(
        &mut self,
        training_tape_box_opt: Option<CassetteTapeBox>,
        board_size: BoardSize,
        app: &Application,
    ) {
        // トレーニング・テープは読み取り専用なんで、べつに保存とかしない☆（＾～＾）
        self.slots[Slot::Training as usize].tape_box = training_tape_box_opt;
        self.slots[Slot::Training as usize].ply = 1;

        let mut full = false;
        if let Some(ref learning_box) = self.slots[Slot::Learning as usize].tape_box {
            // ラーニング・テープの、テープ・ボックスを外部ファイルに保存する（今までのラーニング・テープは、このテープ・ボックスに入っている）。
            let file_name = learning_box.get_file_name();
            learning_box.to_rpm(board_size).write(&file_name, &app.comm);

            if 499 < learning_box.len_tapes() {
                // TODO 満杯になったら次のボックスを新しく作りたい☆（＾～＾）
                full = true;
            }
        } else {
            panic!("Get l_box none.")
        }

        if full {
            // TODO 満杯になったら次のボックスを新しく作りたい☆（＾～＾）
            self.slots[Slot::Learning as usize].tape_box =
                Some(CassetteTapeBox::new_empty(Slot::Learning, &app));
        }

        // 新しいラーニング・テープに差し替える。
        if let Some(ref mut learning_box) = self.slots[Slot::Learning as usize].tape_box {
            learning_box.change_brandnew(&app);
            self.slots[Slot::Learning as usize].ply = 1;
        } else {
            panic!("Get l_box none.")
        }
    }

    /// 次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、おわり☆（＾ｑ＾）
    ///
    /// # Returns
    ///
    /// (成功)
    pub fn change_next_if_training_tape_exists(&mut self, app: &Application) -> bool {
        if let Some(ref mut training_tape_box) = &mut self.slots[Slot::Training as usize].tape_box {
            training_tape_box.change_next_if_it_exists(&app)
        } else {
            false
        }
    }

    /// テープ・フラグメント単位で書き込めるぜ☆（*＾～＾*）スロットは ラーニング限定☆（＾～＾）
    pub fn write_tape_fragment(&mut self, board_size: BoardSize, app: &Application) {
        if let Some(ref tape_box) = self.slots[Slot::Learning as usize].tape_box {
            tape_box.write_tape_fragment_of_current_tape(board_size, &app);
        } else {
            panic!("tape box none.");
        }
    }

    pub fn write_tape_box(&mut self, board_size: BoardSize, app: &Application) {
        if let Some(ref tape_box) = self.slots[Slot::Learning as usize].tape_box {
            tape_box.write_tape_box(board_size, &app);
        } else {
            panic!("tape box none.");
        }
    }

    pub fn get_learning_tape_box_file_name(&self) -> String {
        if let Some(tape_box) = &self.slots[Slot::Learning as usize].tape_box {
            tape_box.get_file_name()
        } else {
            panic!("l_tape box none.");
        }
    }

    pub fn get_ply(&self, slot: Slot) -> i16 {
        self.slots[slot as usize].ply
    }

    pub fn get_sign_of_current_tape(&self, slot: Slot, board_size: BoardSize) -> (String, String) {
        if let Some(ref tape_box) = self.slots[slot as usize].tape_box {
            tape_box.get_sign_of_current_tape(board_size)
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in go to next. Slot: {:?}.", slot);
        }
    }

    /// フェーズ・チェンジか、エンド・オブ・テープを拾うまで進める☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (キャレット番地, フェーズ・チェンジを含み、オーバーフローを含まない１手の範囲)
    pub fn go_1move_forcely(&mut self, slot: Slot, app: &Application) -> (i16, ClosedInterval) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            tape_box.go_1move_forcely(&app)
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in go 1move forcely. Slot: {:?}.", slot);
        }
    }

    /// 結果は３つ☆（＾～＾）
    ///
    /// （１）１ノート進んだ。ついでに拾ったノートを返す。
    /// （２）１ノート進んだ。オーバーフローしていてノートは拾えなかった。
    /// （３）スロットにテープがささっていなかったので強制終了。
    pub fn go_to_next(&mut self, slot: Slot, app: &Application) -> (i16, Option<ShogiNote>) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            tape_box.go_to_next(&app)
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in go to next. Slot: {:?}.", slot);
        }
    }

    /// 指定のスロットの テープボックスの中の、現在のテープの、キャレットの向きを反対にします。
    pub fn turn_caret_to_opponent(&mut self, slot: Slot) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            tape_box.turn_caret_to_opponent();
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in turn caret to opponent. Slot: {:?}.", slot);
        }
    }
    pub fn turn_caret_to_positive(&mut self, slot: Slot) {
        let cassette_slot = &mut self.slots[slot as usize];
        if let Some(ref mut tape_box) = cassette_slot.tape_box {
            tape_box.turn_caret_to_positive();
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in turn caret to positive. Slot: {:?}.", slot);
        }
    }
    pub fn turn_caret_to_negative(&mut self, slot: Slot) {
        let cassette_slot = &mut self.slots[slot as usize];
        if let Some(ref mut tape_box) = cassette_slot.tape_box {
            tape_box.turn_caret_to_negative();
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in turn caret to negative. Slot: {:?}.", slot);
        }
    }

    /*
    pub fn put_1move(&mut self, slot: Slot, rmove: &ShogiMove, app: &Application) {
        for note in rmove.notes.iter() {
            self.put_1note(slot, *note, app);
            if let Some(ply) = note.get_ope().get_phase_change() {
                // フェーズ更新。
                self.slots[slot as usize].ply = ply;
            }
        }
    }
    */

    /// キャレット位置に、ノートを上書き、または追加をするぜ☆（＾～＾）
    pub fn put_1note(&mut self, slot: Slot, note: ShogiNote, app: &Application) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            // とりあえず、キャレットを進めようぜ☆（＾～＾）
            let (caret_number, _) = tape_box.go_to_next(&app);
            if -1 < caret_number {
                // ０、または 正のテープ。
                // 次にオーバーフローするか判断。
                if tape_box.is_before_caret_overflow(&app) {
                    // 正の絶対値が大きい方の新しい要素を追加しようとしている。
                    tape_box.push_note_to_positive_of_current_tape(note);
                    tape_box.go_to_next(&app);
                } else {
                    // 先端でなければ、上書き。
                    tape_box.set_note_to_current_tape(caret_number, note);

                    if let (caret_number, Some(_note)) = tape_box.go_to_next(&app) {
                        // 仮のおわり を更新。
                        tape_box.truncate_positive_of_current_tape(get_index_from_caret_numbers(
                            caret_number,
                        ));
                    } else {

                    }
                }
            } else {
                // 負のテープ。
                // 最先端かどうか判断。
                if tape_box.is_before_caret_overflow(&app) {
                    // 負の絶対値が大きい方の新しい要素を追加しようとしている。
                    tape_box.push_note_to_negative_of_current_tape(note);
                    tape_box.go_to_next(&app);
                } else {
                    // 先端でなければ、上書き。
                    tape_box.set_note_to_current_tape(caret_number, note);
                    if let (caret_number, Some(_note)) = tape_box.go_to_next(&app) {
                        // 仮のおわり を更新。
                        tape_box.truncate_negative_of_current_tape(get_index_from_caret_numbers(
                            caret_number,
                        ));
                    } else {

                    }
                }
            }
        } else {
            panic!("Recording tape is none.")
        };
    }

    /// # Returns
    /// TODO ply が変わることがある。
    ///
    /// 削除したノート。
    pub fn delete_1note(&mut self, slot: Slot, app: &Application) -> Option<ShogiNote> {
        let cassette_slot = &mut self.slots[slot as usize];
        if let Some(ref mut tape_box) = cassette_slot.tape_box {
            tape_box.delete_1note(&app)
        } else {
            None
        }
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    /// TODO ply が変わることがある。
    ///
    /// # Returns
    ///
    /// 削除したノート。
    pub fn pop_1note(
        &mut self,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) -> Option<ShogiNote> {
        HumanInterface::show_position(slot, &app.comm, -1, position);

        if let Some(rpm_note) = self.delete_1note(slot, &app) {
            let (_is_legal_touch, _piece_identify_opt) =
                position.try_beautiful_touch_no_log(&rpm_note.get_ope(), &app.comm);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手削除する。
    ///
    /// TODO ply が変わる。
    pub fn pop_1move(&mut self, slot: Slot, position: &mut Position, app: &Application) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = self.pop_1note(slot, position, app) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    pub fn to_human_presentable_of_current_tape_of_training_box(
        &self,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        if let Some(training_tape_box) = &self.slots[Slot::Training as usize].tape_box {
            training_tape_box.to_human_presentable_of_current_tape(board_size, &app)
        } else {
            "None-t-tape-box".to_string()
        }
    }
    pub fn to_human_presentable_of_caret_of_current_tape_of_training_box(
        &self,
        app: &Application,
    ) -> String {
        if let Some(training_tape_box) = &self.slots[Slot::Training as usize].tape_box {
            training_tape_box.to_human_presentable_of_caret_of_current_tape(&app)
        } else {
            "None-t-tape-box".to_string()
        }
    }
    pub fn to_human_presentable_of_tape_box(&self, slot: Slot) -> String {
        if let Some(tape_box) = &self.slots[slot as usize].tape_box {
            tape_box.to_human_presentable()
        } else {
            "None-t-tape-box".to_string()
        }
    }
    pub fn to_human_presentable(&self) -> String {
        let mut text = String::new();

        for slot in &self.slots {
            text = format!("{}{}", text, slot.to_human_presentable())
        }

        text
    }
}
