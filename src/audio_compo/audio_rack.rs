use audio_compo::cassette_deck::*;
use media::cassette_tape::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use studio::application::Application;
use studio::board_size::BoardSize;
use studio::common::caret::Awareness;
use studio::common::caret::*;
use studio::common::closed_interval::ClosedInterval;

pub struct AudioRack {
    // カセット・デッキ。
    deck: CassetteDeck,
}
impl AudioRack {
    // ###############
    // # Constructor #
    // ###############

    /// 新規作成☆（＾～＾）
    pub fn new(app: &Application) -> Self {
        Self {
            deck: CassetteDeck::new(&app),
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
        self.deck
            .add_tapes_from_file(box_file_name, slot, board_size, &app);
    }

    pub fn add_tape_to_tape_box(&mut self, slot: Slot, tape: CassetteTape, app: &Application) {
        self.deck.add_tape_to_tape_box(slot, tape, &app);
    }

    // #####
    // # C #
    // #####

    pub fn clear_of_tapes(&mut self, slot: Slot, app: &Application) {
        self.deck.clear_of_tapes(slot, &app)
    }
    pub fn clear_tape_body(&mut self, slot: Slot, app: &Application) {
        self.deck.clear_tape_body(slot, &app)
    }

    // #####
    // # D #
    // #####

    pub fn delete_1note(&mut self, slot: Slot, app: &Application) -> Option<ShogiNote> {
        self.deck.delete_1note(slot, &app)
    }

    // #####
    // # G #
    // #####

    pub fn get_mut_deck(&mut self) -> &mut CassetteDeck {
        &mut self.deck
    }

    pub fn get_ply(&self, slot: Slot) -> i16 {
        self.deck.get_ply(slot)
    }

    pub fn get_sign_of_current_tape(&self, slot: Slot, board_size: BoardSize) -> (String, String) {
        self.deck.get_sign_of_current_tape(slot, board_size)
    }

    pub fn get_file_name_of_tape_box(&self, slot: Slot) -> String {
        self.deck.get_file_name_of_tape_box(slot)
    }

    pub fn get_tape_index(&self, slot: Slot) -> Option<usize> {
        self.deck.get_tape_index(slot)
    }

    /// -と+方向の長さがある☆（＾～＾）
    pub fn get_current_tape_span(&self, slot: Slot) -> ClosedInterval {
        self.deck.get_current_tape_span(slot)
    }

    // #####
    // # I #
    // #####

    pub fn insert_note(
        &mut self,
        slot: Slot,
        note: ShogiNote,
        board_size: BoardSize,
        app: &Application,
    ) {
        self.deck.insert_note(slot, note, board_size, &app);
    }

    pub fn is_facing_left_of_current_tape(&self, slot: Slot, app: &Application) -> bool {
        self.deck.is_facing_left_of_current_tape(slot, &app)
    }

    pub fn is_none_current_tape(&self, slot: Slot) -> bool {
        self.deck.is_none_current_tape(slot)
    }

    // #####
    // # L #
    // #####

    /// 指定のスロットの テープボックスの中の、現在のテープの、キャレットの向きを反対にします。
    pub fn look_back_caret(&mut self, slot: Slot, app: &Application) {
        self.deck.look_back_caret(slot, &app);
    }
    pub fn turn_caret_towards_positive_infinity(&mut self, slot: Slot, app: &Application) {
        self.deck.turn_caret_towards_positive_infinity(slot, &app);
    }
    pub fn turn_caret_towards_negative_infinity(&mut self, slot: Slot, app: &Application) {
        self.deck.turn_caret_towards_negative_infinity(slot, &app);
    }

    // #####
    // # P #
    // #####

    /// 正の方のテープの末端にノートを追加。
    pub fn push_note(&mut self, slot: Slot, note: ShogiNote) {
        self.deck.push_note(slot, note);
    }
    pub fn pop_note(&mut self, slot: Slot) -> Option<ShogiNote> {
        self.deck.pop_note(slot)
    }

    // #####
    // # S #
    // #####

    pub fn set_file_name_without_extension_of_tape_box(
        &mut self,
        slot: Slot,
        tape_box_file_name_without_extension: &str,
    ) {
        self.deck.set_file_name_without_extension_of_tape_box(
            slot,
            tape_box_file_name_without_extension,
        );
    }

    /// 次のテープを利用するぜ☆（＾～＾）
    /// 次のテープが無ければ、おわり☆（＾ｑ＾）
    ///
    /// # Returns
    ///
    /// (成功)
    pub fn seek_of_next_tape(&mut self, slot: Slot, app: &Application) -> bool {
        self.deck.seek_of_next_tape(slot, &app)
    }

    pub fn step_in_of_tape(&self, slot: Slot) -> i16 {
        self.deck.step_in_of_tape(slot)
    }

    /// # Returns
    ///
    /// (taken overflow, move, フェーズ・チェンジを含み、オーバーフローを含まない１手の範囲)
    pub fn skip_a_move(&mut self, slot: Slot, app: &Application) -> (bool, ShogiMove) {
        self.deck.skip_a_move(slot, &app)
    }

    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note(
        &mut self,
        slot: Slot,
        app: &Application,
    ) -> (bool, Awareness, Option<ShogiNote>) {
        self.deck.seek_a_note(slot, &app)
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
        self.deck.seek_a_note_with_othre_caret(slot, caret, &app)
    }

    // #####
    // # T #
    // #####

    pub fn to_human_presentable_of_current_tape_of_training_box(
        &self,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        self.deck
            .to_human_presentable_of_current_tape_of_training_box(board_size, &app)
    }
    pub fn to_human_presentable_of_caret(&self, slot: Slot, app: &Application) -> String {
        self.deck.to_human_presentable_of_caret(slot, &app)
    }
    pub fn to_human_presentable_of_tape_box(&self, slot: Slot) -> String {
        self.deck.to_human_presentable_of_tape_box(slot)
    }

    // #####
    // # W #
    // #####

    /// テープ・フラグメント単位で書き込めるぜ☆（*＾～＾*）スロットは ラーニング限定☆（＾～＾）
    pub fn write_leaning_tapes_fragment(&mut self, board_size: BoardSize, app: &Application) {
        self.deck.write_leaning_tapes_fragment(board_size, &app);
    }

    pub fn write_tape_box(&mut self, board_size: BoardSize, app: &Application) {
        self.deck.write_tape_box(board_size, &app);
    }

    pub fn to_human_presentable(&self) -> String {
        self.deck.to_human_presentable()
    }
}
