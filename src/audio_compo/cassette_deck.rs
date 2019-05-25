use media::cassette_tape::*;
use media::cassette_tape_box::*;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use studio::application::Application;
use studio::board_size::BoardSize;
use studio::common::caret::Awareness;
use studio::common::caret::*;
use studio::common::closed_interval::ClosedInterval;

#[derive(Clone, Copy, Debug)]
pub enum Slot {
    /// トレーニング。
    Training,
    /// ラーニング。
    Learning,
}

/// カセット・デッキ。
pub struct CassetteDeck {
    // カセットのスロット。トレーニング、ラーニングの順。
    pub slots: [CassetteTapeBox; 2],
}
impl CassetteDeck {
    // ###############
    // # Constructor #
    // ###############

    /// 新規作成☆（＾～＾）
    pub fn new(app: &Application) -> Self {
        if app.is_debug() {
            app.comm.println("[#Deck:new]");
        }

        let mut training_tape_box = CassetteTapeBox::new_empty_tape_box(Slot::Training, &app);
        training_tape_box.set_file_name_without_extension(
            &RpmTapeBox::create_file_full_name_without_extension(&app.kw29_conf, &app),
        );

        let mut learning_tape_box = CassetteTapeBox::new_empty_tape_box(Slot::Learning, &app);
        learning_tape_box.set_file_name_without_extension(
            &RpmTapeBox::create_file_full_name_without_extension(&app.kw29_conf, &app),
        );

        CassetteDeck {
            slots: [training_tape_box, learning_tape_box],
        }
    }

    // #####
    // # A #
    // #####

    /// ◆JSONファイルを読み込んで、テープを詰め込みます。
    pub fn add_tapes_from_file(
        &mut self,
        box_file_name: &str,
        slot: Slot,
        board_size: BoardSize,
        app: &Application,
    ) {
        let tape_box = &mut self.slots[slot as usize];
        let rpm_tape_box = RpmTapeBox::from_box_file(&box_file_name, &app);

        // 仮のテープ・ボックス・ファイル名。
        tape_box.set_file_name_without_extension(
            &RpmTapeBox::create_file_full_name_without_extension(&app.kw29_conf, &app),
        );

        for tape_j in &rpm_tape_box.tape_box {
            // テープを追加中。テープを追加しても、キャレットは進まない☆（*＾～＾*）
            let tape = tape_j.to_object(board_size, &app);
            tape_box.add_tape(tape, &app);
        }
    }

    pub fn add_tape_to_tape_box(&mut self, slot: Slot, tape: CassetteTape, app: &Application) {
        self.slots[slot as usize].add_tape(tape, &app);
    }

    /*
    /// トレーニング・テープを交換するぜ☆（＾～＾）
    ///
    /// 棋譜読取の際は、テープ・ボックスの中にテープ・インデックスが指定されてある☆（＾～＾）
    pub fn change_training_tape(&mut self, board_size: BoardSize, app: &Application) {
        if app.is_debug() {
            app.comm.println("[#Change learning tape box]");
        }

        // トレーニング・テープは読み取り専用なんで、べつに保存とかしない☆（＾～＾）
        // self.set_tape_box(Slot::Training, training_tape_box_opt, &app);
        self.slots[Slot::Training as usize].ply = 1;

        let mut full = false;
        let learning_box = &self.slots[Slot::Learning as usize];
        // ラーニング・テープの、テープ・ボックスを外部ファイルに保存する（今までのラーニング・テープは、このテープ・ボックスに入っている）。
        let file_name = learning_box.get_file_name();
        learning_box.to_rpm(board_size).write(&file_name, &app);

        if 499 < learning_box.len_tapes() {
            // TODO 満杯になったら次のボックスを新しく作りたい☆（＾～＾）
            full = true;
        }

        if full {
            // TODO 満杯になったら次のボックスを新しく作りたい☆（＾～＾）
            if app.is_debug() {
                app.comm.println("Change learning tape box.");
            }

            self.clear_of_tapes(Slot::Learning, &app);
            self.set_file_name_without_extension_of_tape_box(
                Slot::Learning,
                &RpmTapeBox::create_file_full_name_without_extension(&app.kw29_conf, &app),
            );
        }

        // 新しいラーニング・テープに差し替える。
        if app.is_debug() {
            app.comm.println("Change learning tape.");
        }

        let brandnew = CassetteTape::new_facing_right(&app);
        self.slots[Slot::Learning as usize].add_tape(brandnew, &app);
        self.slots[Slot::Learning as usize].ply = 1;
    }
    */

    // #####
    // # C #
    // #####

    pub fn clear_of_tapes(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].clear_tape_box(&app)
    }
    pub fn clear_tape_body(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].clear_tape_body(&app)
    }

    // #####
    // # D #
    // #####

    /// # Returns
    /// TODO ply が変わることがある。
    ///
    /// 削除したノート。
    pub fn delete_1note(&mut self, slot: Slot, app: &Application) -> Option<ShogiNote> {
        self.slots[slot as usize].delete_1note(&app)
    }

    // #####
    // # G #
    // #####

    pub fn get_ply(&self, slot: Slot) -> i16 {
        self.slots[slot as usize].ply
    }

    pub fn get_sign_of_current_tape(&self, slot: Slot, board_size: BoardSize) -> (String, String) {
        self.slots[slot as usize].get_sign_of_current_tape(board_size)
    }

    pub fn get_file_name_of_tape_box(&self, slot: Slot) -> String {
        self.slots[slot as usize].get_file_name()
    }

    pub fn get_tape_index(&self, slot: Slot) -> Option<usize> {
        self.slots[slot as usize].get_tape_index()
    }

    /// -と+方向の長さがある☆（＾～＾）
    pub fn get_current_tape_span(&self, slot: Slot) -> ClosedInterval {
        self.slots[slot as usize].get_current_tape_len()
    }

    // #####
    // # I #
    // #####

    /// キャレット位置に、ノートを挿入するぜ☆（＾～＾）
    pub fn insert_note(
        &mut self,
        slot: Slot,
        note: ShogiNote,
        board_size: BoardSize,
        app: &Application,
    ) {
        self.slots[slot as usize].insert_note(note, board_size, &app);
    }

    pub fn is_facing_left_of_current_tape(&self, slot: Slot, _app: &Application) -> bool {
        self.slots[slot as usize].is_facing_left_of_current_tape()
    }

    pub fn is_none_current_tape(&self, slot: Slot) -> bool {
        self.slots[slot as usize].is_none_current_tape()
    }

    // #####
    // # L #
    // #####

    /// 指定のスロットの テープボックスの中の、現在のテープの、キャレットの向きを反対にします。
    pub fn look_back_caret(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].look_back_caret(&app);
    }
    pub fn turn_caret_towards_positive_infinity(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].turn_caret_towards_positive_infinity(&app);
    }
    pub fn turn_caret_towards_negative_infinity(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].turn_caret_towards_negative_infinity(&app);
    }

    // #####
    // # P #
    // #####

    /// 正の方のテープの末端にノートを追加。
    pub fn push_note(&mut self, slot: Slot, note: ShogiNote) {
        self.slots[slot as usize].push_note(note);
    }
    pub fn pop_note(&mut self, slot: Slot) -> Option<ShogiNote> {
        self.slots[slot as usize].pop_note()
    }

    // #####
    // # S #
    // #####

    pub fn set_file_name_without_extension_of_tape_box(
        &mut self,
        slot: Slot,
        tape_box_file_name_without_extension: &str,
    ) {
        self.slots[slot as usize]
            .set_file_name_without_extension(tape_box_file_name_without_extension);
    }

    pub fn set_source_file_of_tape_label(&mut self, slot: Slot, tape_label: String) {
        self.slots[slot as usize].set_source_file_of_tape_label(tape_label);
    }

    /// 次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、おわり☆（＾ｑ＾）
    ///
    /// # Returns
    ///
    /// (成功)
    pub fn seek_of_next_tape(&mut self, slot: Slot, app: &Application) -> bool {
        self.slots[slot as usize].seek_of_tapes(&app)
    }

    pub fn step_in_of_tape(&self, slot: Slot) -> i16 {
        self.slots[slot as usize].step_in_of_tape()
    }

    /// # Returns
    ///
    /// (taken overflow, move, フェーズ・チェンジを含み、オーバーフローを含まない１手の範囲)
    pub fn skip_a_move(&mut self, slot: Slot, app: &Application) -> (bool, ShogiMove) {
        self.slots[slot as usize].skip_a_move(&app)
    }

    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note(
        &mut self,
        slot: Slot,
        app: &Application,
    ) -> (bool, Awareness, Option<ShogiNote>) {
        self.slots[slot as usize].seek_a_note(&app)
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    ///
    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note_with_othre_caret(
        &mut self,
        slot: Slot,
        caret: &mut Caret,
        app: &Application,
    ) -> (bool, Awareness, Option<ShogiNote>) {
        self.slots[slot as usize].seek_a_note_with_othre_caret(caret, &app)
    }

    // #####
    // # T #
    // #####

    pub fn to_human_presentable_of_current_tape_of_training_box(
        &self,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        self.slots[Slot::Training as usize].to_human_presentable_of_current_tape(board_size, &app)
    }
    pub fn to_human_presentable_of_caret(&self, slot: Slot, app: &Application) -> String {
        self.slots[slot as usize].to_human_presentable_of_caret(&app)
    }
    pub fn to_human_presentable_of_tape_box(&self, slot: Slot) -> String {
        self.slots[slot as usize].to_human_presentable()
    }

    // #####
    // # W #
    // #####

    /// テープ・フラグメント単位で書き込めるぜ☆（*＾～＾*）スロットは ラーニング限定☆（＾～＾）
    pub fn write_leaning_tapes_fragment(&mut self, board_size: BoardSize, app: &Application) {
        self.slots[Slot::Learning as usize].write_current_tapes_fragment(board_size, &app);
    }

    pub fn write_tape_box(&mut self, board_size: BoardSize, app: &Application) {
        self.slots[Slot::Learning as usize].write_tape_box(board_size, &app);
    }

    pub fn to_human_presentable(&self) -> String {
        let mut text = "---------- Deck-info ----------\n".to_string();
        text = format!("{}Slot len: {}.\n", text, self.slots.len());

        let mut i = 0;
        for slot in &self.slots {
            text = format!("{}  Slot[{}]: {}\n", text, i, slot.to_human_presentable());
            /*
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
            */

            i += 1;
        }

        text
    }
}
