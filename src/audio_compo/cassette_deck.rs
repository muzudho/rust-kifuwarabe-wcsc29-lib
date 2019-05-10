use human::human_interface::*;
use instrument::position::*;
use media::cassette_tape::*;
use media::cassette_tape_box::*;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use studio::application::Application;
use studio::board_size::BoardSize;
use studio::common::caret::get_index_from_caret_numbers;
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
    pub fn new_empty(app: &Application) -> Self {
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
    pub fn clear_of_tape_body(&mut self, slot: Slot, app: &Application) {
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
    // # L #
    // #####

    /// 指定のスロットの テープボックスの中の、現在のテープの、キャレットの向きを反対にします。
    pub fn look_back_caret_to_opponent(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].look_back_caret_to_opponent(&app);
    }
    pub fn look_back_caret_to_positive(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].look_back_caret_to_positive(&app);
    }
    pub fn look_back_caret_to_negative(&mut self, slot: Slot, app: &Application) {
        self.slots[slot as usize].look_back_caret_to_negative(&app);
    }

    // #####
    // # P #
    // #####

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    /// TODO ply が変わることがある。
    ///
    /// # Returns
    ///
    /// 削除したノート。
    pub fn pop_1note(&mut self, position: &mut Position, app: &Application) -> Option<ShogiNote> {
        HumanInterface::show_position(position, &app);

        if let Some(rpm_note) = self.delete_1note(Slot::Learning, &app) {
            let (_is_legal_touch, _piece_identify_opt) =
                position.try_beautiful_touch_no_log(&self, &rpm_note.get_ope(), &app);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手削除する。
    ///
    /// TODO ply が変わる。
    pub fn pop_1move(&mut self, position: &mut Position, app: &Application) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = self.pop_1note(position, app) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    /// キャレット位置に、ノートを上書き、または追加をするぜ☆（＾～＾）
    pub fn put_1note(&mut self, slot: Slot, note: ShogiNote, app: &Application) {
        let tape_box = &mut self.slots[slot as usize];
        // とりあえず、キャレットを進めようぜ☆（＾～＾）
        let (_taken_overflow, rmove, _) = tape_box.seek_to_next_note(&app);

        if -1 < rmove.get_end() {
            // ０、または 正のテープ。
            // 次にオーバーフローするか判断。
            if tape_box.is_before_caret_overflow_of_tape() {
                // 正の絶対値が大きい方の新しい要素を追加しようとしている。
                tape_box.push_note_to_positive_of_current_tape(note);
                tape_box.seek_to_next_note(&app);
            } else {
                // 先端でなければ、上書き。
                tape_box.set_note_to_current_tape(rmove.get_end(), note);

                // 仮のおわり を更新。
                tape_box.truncate_positive_of_current_tape(get_index_from_caret_numbers(
                    rmove.get_end(),
                ));
            }
        } else {
            // 負のテープ。
            // 最先端かどうか判断。
            if tape_box.is_before_caret_overflow_of_tape() {
                // 負の絶対値が大きい方の新しい要素を追加しようとしている。
                tape_box.push_note_to_negative_of_current_tape(note);
                tape_box.seek_to_next_note(&app);
            } else {
                // 先端でなければ、上書き。
                tape_box.set_note_to_current_tape(rmove.get_end(), note);
                // 仮のおわり を更新。
                tape_box.truncate_negative_of_current_tape(get_index_from_caret_numbers(
                    rmove.get_end(),
                ));
            }
        }
    }

    // #####
    // # R #
    // #####

    pub fn read_tape_for_n_moves_forcely(
        &mut self,
        slot: Slot,
        repeats: usize,
        position: &mut Position,
        app: &Application,
    ) {
        for _i in 0..repeats {
            self.read_tape_for_1move_forcely(slot, position, &app);
        }
    }

    /// 必ず1手進める。（非合法タッチがあれば強制終了）
    pub fn read_tape_for_1move_forcely(
        &mut self,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) {
        /*
        app.comm.println(&format!(
            "[TOP read_tape_for_1move_forcely:{}:{}]",
            self.to_human_presentable_of_tape_box(slot),
            self.to_human_presentable_of_caret_of_current_tape_of_training_box(&app)
        ));
        */

        let mut is_legal_touch = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら終了。
        while let (_taken_overflow, _note_move, Some(rnote)) = self.seek_to_next_note(slot, &app) {
            /*
            app.comm.println(&format!(
                "[LOOP read_tape_for_1move_forcely:{}:{}:{}]",
                self.to_human_presentable_of_caret_of_current_tape_of_training_box(&app),
                self.to_human_presentable_of_tape_box(slot),
                rnote.to_human_presentable(position.get_board_size())
            ));
            */
            is_legal_touch = position.try_beautiful_touch(&self, &rnote, &app);
            forwarding_count += 1;

            if !is_legal_touch {
                break;
            }

            if forwarding_count != 1 && rnote.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                // print!("<NXm1End{} {}>", forwarding_count, rnote);
                break;
            }
        }

        if !is_legal_touch {
            // 非合法タッチは強制終了。
            panic!("Illegal, go opponent forcely!");
        }

        // 1つも読まなかったら強制終了。
        if forwarding_count < 1 {
            panic!("Illegal, zero foward!");
        }
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

    /// フェーズ・チェンジか、エンド・オブ・テープを拾うまで進める☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (taken overflow, move, フェーズ・チェンジを含み、オーバーフローを含まない１手の範囲)
    pub fn seek_1move(&mut self, slot: Slot, app: &Application) -> (bool, ShogiMove) {
        self.slots[slot as usize].seek_1move(&app)
    }

    /// キャレットは必ず１つ進みます。
    /// 0 は、正の数とします。（マイナスゼロは無いです）
    /// Noneを返したら、オーバーフローしています。
    ///
    /// だから、結果は３つ☆（＾～＾）
    ///
    /// （１）１ノート進んだ。ついでに拾ったノートを返す。
    /// （２）１ノート進んだ。オーバーフローしていてノートは拾えなかった。
    /// （３）スロットにテープがささっていなかったので強制終了。
    ///
    /// # Returns
    ///
    /// (taken overflow, move, note)
    pub fn seek_to_next_note(
        &mut self,
        slot: Slot,
        app: &Application,
    ) -> (bool, ShogiMove, Option<ShogiNote>) {
        self.slots[slot as usize].seek_to_next_note(&app)
    }

    /// 成立しないタッチをしてしまうことも、おおめに見ます。
    pub fn seek_n_notes_permissive(
        &mut self,
        slot: Slot,
        repeat: usize,
        position: &mut Position,
        app: &Application,
    ) {
        for _i in 0..repeat {
            // キャレットは必ず進めろだぜ☆（＾～＾）
            // 結果は３つ☆（＾～＾）
            //
            // （１）１ノート進んだ。ついでに拾ったノートを返す。
            // （２）１ノート進んだ。オーバーフローしていてノートは拾えなかった。
            // （３）スロットにテープがささっていなかったので強制終了。
            if let (_taken_overflow, _note_move, Some(rnote)) = self.seek_to_next_note(slot, &app) {
                // 指し手を拾えたのなら、指せだぜ☆（＾～＾）
                /*
                app.comm.println(&format!(
                    "<Go-force:{}/{} {}>",
                    i,
                    repeat,
                    rnote.to_human_presentable(position.get_board_size())
                ));
                */
                if !position.try_beautiful_touch(&self, &rnote, &app) {
                    /*
                    app.comm.println(&format!(
                        "Touch fail, permissive. Note: {}, Caret: {}.",
                        rnote.to_human_presentable(position.get_board_size()),
                        self.to_human_presentable_of_caret_of_current_tape_of_training_box(&app),
                    ));
                    */
                }
            } else {
                // オーバーフローした☆（＾～＾）テープの終了だが、テープを終了したあとにバックすればもう１回終了するし☆（＾～＾）
                // 気にせずループを続行しろだぜ☆（＾～＾）

                /*
                if i + 1 == repeat {
                    // 指示したリピートの数と、テープの終了は一致するはずだぜ☆（＾～＾）
                    break;
                } else {
                    panic!(
                        "テープの長さを超えてリピートしろと指示出してる☆（＾～＾）どっかおかしいのでは☆（＾～＾）？  Caret: {}, i {}, repeat {} notes.",
                        self.to_human_presentable_of_caret_of_current_tape(slot, &app),
                        i,
                        repeat
                    );
                }
                */
            }
        }
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
    pub fn to_human_presentable_of_caret_of_current_tape(
        &self,
        slot: Slot,
        app: &Application,
    ) -> String {
        self.slots[slot as usize].to_human_presentable_of_caret_of_current_tape(&app)
    }
    pub fn to_human_presentable_of_tape_box(&self, slot: Slot) -> String {
        self.slots[slot as usize].to_human_presentable()
    }

    /// 1手分進める。（非合法タッチは自動で戻します）
    ///
    /// 結果は次の４つだぜ☆（＾～＾）
    /// （１）最後の１手分。局面もキャレットも進んでいる。
    /// （２）最後ではない１手分。局面もキャレットも進んでいる。
    /// （３）テープ終わっていた。キャレットを戻す。
    /// （４）実現しない操作だった。局面とキャレットを戻す。
    ///
    /// # Return
    ///
    /// (taken overflow, move)
    pub fn try_seek_1move(
        &mut self,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) -> (bool, ShogiMove) {
        // 指し手（実際のところ、テープ上の範囲を示したもの）。
        let mut rmove = ShogiMove::new_facing_right_move();

        let mut is_rollback = false;
        let mut is_phase_change = false;

        'caret_loop: loop {
            // とりあえず、キャレットを１ノートずつ進めてみるぜ☆（*＾～＾*）
            match self.seek_to_next_note(slot, &app) {
                (_taken_overflow, note_move, Some(rnote)) => {
                    // タッチに成功するか、しないかに関わらず、このノートはムーブに含める☆（＾～＾）
                    // このムーブの長さが、進めたノートの数と等しいぜ☆（＾～＾）
                    // あとで、ルック・バックする範囲☆（＾～＾）
                    rmove
                        .caret_closed_interval
                        .intersect_closed_interval(note_move.caret_closed_interval);

                    if position.try_beautiful_touch(&self, &rnote, &app) {
                        // ここに来たら、着手は成立☆（*＾～＾*）
                        /*
                        app.comm.println(&format!(
                            "[{} note advanced! Note:{}, Move:{}]",
                            rmove.len(),
                            rnote.to_human_presentable(position.get_board_size()),
                            rmove.to_human_presentable(self, slot, position.get_board_size(), &app)
                        ));
                        */

                        if 1 < rmove.len() && rnote.is_phase_change() {
                            // フェーズ切り替えしたら終了。（ただし、初回除く）
                            // print!("[Phase-change-break try_read_1move:{}]", rnote);
                            is_phase_change = true;
                            break 'caret_loop;
                        }
                    } else {
                        // 未着手なタッチならループを抜けて、今回進めた分を全部逆戻りさせるループへ進むぜ☆（＾～＾）
                        // app.comm.println("[$Untouched. Back a caret]");
                        is_rollback = true;
                        break 'caret_loop;
                    }
                }
                (taken_overflow, note_move, None) => {
                    // オーバーフローを、読んだということだぜ☆（＾～＾）
                    if app.is_debug() {
                        app.comm.println(
                            "[#オーバーフローのノートを読んでる☆（＾～＾）]",
                        );
                    }
                    rmove
                        .caret_closed_interval
                        .intersect_closed_interval(note_move.caret_closed_interval);
                    return (taken_overflow, rmove);
                }
            }
        }

        // ここに来た時、ムーブの長さ＋１　分だけキャレットは進んでいる☆（＾～＾）

        if is_rollback {
            // キャレットを使って局面を戻す。
            /*
            app.comm.println(&format!(
                "[Try_read_1move: Rollback {} note!:{}]",
                rmove.len(),
                rmove.to_human_presentable(self, slot, position.get_board_size(), &app)
            ));
            */
            self.look_back_caret_to_opponent(slot, &app);
            self.seek_n_notes_permissive(slot, rmove.len(), position, &app);
            self.look_back_caret_to_opponent(slot, &app);

            return (false, ShogiMove::new_facing_right_move());
        }

        if is_phase_change {
            // 1手分。
            (false, rmove)
        } else {
            // 最後の1手なのでフェーズ・チェンジが無かったと考える。
            (true, rmove)
        }
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
