use human::human_interface::*;
use instrument::position::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use studio::application::Application;
use studio::board_size::BoardSize;
use studio::common::caret::get_index_from_caret_numbers;
use studio::common::closed_interval::ClosedInterval;
use video_tape_model::cassette_tape_box::*;

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
            learning_box.to_rpm(board_size).write(&file_name, &app);

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

    pub fn get_file_name_of_tape_box(&self, slot: Slot) -> String {
        if let Some(ref tape_box) = self.slots[slot as usize].tape_box {
            tape_box.get_file_name()
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box file name fail. Slot: {:?}.", slot);
        }
    }

    pub fn get_tape_index(&self, slot: Slot) -> Option<usize> {
        if let Some(ref tape_box) = self.slots[slot as usize].tape_box {
            tape_box.get_tape_index()
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("Get tape index fail. Slot: {:?}.", slot);
        }
    }

    /// -と+方向の長さがある☆（＾～＾）
    pub fn get_current_tape_span(&self, slot: Slot) -> ClosedInterval {
        if let Some(ref tape_box) = self.slots[slot as usize].tape_box {
            tape_box.get_current_tape_len()
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("Get tape len fail. Slot: {:?}.", slot);
        }
    }

    pub fn step_in(&self, slot: Slot, app: &Application) -> i16 {
        if let Some(tape_box) = &self.slots[slot as usize].tape_box {
            tape_box.step_in(&app)
        } else {
            panic!("Step in fail.");
        }
    }

    /// フェーズ・チェンジか、エンド・オブ・テープを拾うまで進める☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (taken overflow, move, フェーズ・チェンジを含み、オーバーフローを含まない１手の範囲)
    pub fn seek_1move(&mut self, slot: Slot, app: &Application) -> (bool, ShogiMove) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            tape_box.seek_1move(&app)
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in go 1move forcely. Slot: {:?}.", slot);
        }
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
    pub fn seek_to_next(
        &mut self,
        slot: Slot,
        app: &Application,
    ) -> (bool, ShogiMove, Option<ShogiNote>) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            tape_box.seek_to_next(&app)
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in go to next. Slot: {:?}.", slot);
        }
    }

    /// 指定のスロットの テープボックスの中の、現在のテープの、キャレットの向きを反対にします。
    pub fn look_back_caret_to_opponent(&mut self, slot: Slot, app: &Application) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            tape_box.look_back_caret_to_opponent(&app);
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in turn caret to opponent. Slot: {:?}.", slot);
        }
    }
    pub fn look_back_caret_to_positive(&mut self, slot: Slot, app: &Application) {
        let cassette_slot = &mut self.slots[slot as usize];
        if let Some(ref mut tape_box) = cassette_slot.tape_box {
            tape_box.look_back_caret_to_positive(&app);
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in turn caret to positive. Slot: {:?}.", slot);
        }
    }
    pub fn look_back_caret_to_negative(&mut self, slot: Slot, app: &Application) {
        let cassette_slot = &mut self.slots[slot as usize];
        if let Some(ref mut tape_box) = cassette_slot.tape_box {
            tape_box.look_back_caret_to_negative(&app);
        } else {
            // 指定のスロットの テープボックスの中の、現在のテープ が無いエラー。
            panic!("tape box none in turn caret to negative. Slot: {:?}.", slot);
        }
    }

    /// キャレット位置に、ノートを上書き、または追加をするぜ☆（＾～＾）
    pub fn put_1note(&mut self, slot: Slot, note: ShogiNote, app: &Application) {
        if let Some(ref mut tape_box) = &mut self.slots[slot as usize].tape_box {
            // とりあえず、キャレットを進めようぜ☆（＾～＾）
            let (_taken_overflow, rmove, _) = tape_box.seek_to_next(&app);

            if -1 < rmove.get_end() {
                // ０、または 正のテープ。
                // 次にオーバーフローするか判断。
                if tape_box.is_before_caret_overflow(&app) {
                    // 正の絶対値が大きい方の新しい要素を追加しようとしている。
                    tape_box.push_note_to_positive_of_current_tape(note);
                    tape_box.seek_to_next(&app);
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
                if tape_box.is_before_caret_overflow(&app) {
                    // 負の絶対値が大きい方の新しい要素を追加しようとしている。
                    tape_box.push_note_to_negative_of_current_tape(note);
                    tape_box.seek_to_next(&app);
                } else {
                    // 先端でなければ、上書き。
                    tape_box.set_note_to_current_tape(rmove.get_end(), note);
                    // 仮のおわり を更新。
                    tape_box.truncate_negative_of_current_tape(get_index_from_caret_numbers(
                        rmove.get_end(),
                    ));
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
        HumanInterface::show_position(position, &app);

        if let Some(rpm_note) = self.delete_1note(slot, &app) {
            let (_is_legal_touch, _piece_identify_opt) =
                position.try_beautiful_touch_no_log(&rpm_note.get_ope(), &app);
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
            match self.seek_to_next(slot, &app) {
                (_taken_overflow, note_move, Some(rnote)) => {
                    // タッチに成功するか、しないかに関わらず、このノートはムーブに含める☆（＾～＾）
                    // このムーブの長さが、進めたノートの数と等しいぜ☆（＾～＾）
                    // あとで、ルック・バックする範囲☆（＾～＾）
                    rmove
                        .caret_closed_interval
                        .intersect_closed_interval(note_move.caret_closed_interval);

                    if position.try_beautiful_touch(&rnote, &app) {
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
        while let (_taken_overflow, _note_move, Some(rnote)) = self.seek_to_next(slot, &app) {
            /*
            app.comm.println(&format!(
                "[LOOP read_tape_for_1move_forcely:{}:{}:{}]",
                self.to_human_presentable_of_caret_of_current_tape_of_training_box(&app),
                self.to_human_presentable_of_tape_box(slot),
                rnote.to_human_presentable(position.get_board_size())
            ));
            */
            is_legal_touch = position.try_beautiful_touch(&rnote, &app);
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
            if let (_taken_overflow, _note_move, Some(rnote)) = self.seek_to_next(slot, &app) {
                // 指し手を拾えたのなら、指せだぜ☆（＾～＾）
                /*
                app.comm.println(&format!(
                    "<Go-force:{}/{} {}>",
                    i,
                    repeat,
                    rnote.to_human_presentable(position.get_board_size())
                ));
                */
                if !position.try_beautiful_touch(&rnote, &app) {
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
    pub fn to_human_presentable_of_caret_of_current_tape(
        &self,
        slot: Slot,
        app: &Application,
    ) -> String {
        if let Some(tape_box) = &self.slots[slot as usize].tape_box {
            tape_box.to_human_presentable_of_caret_of_current_tape(&app)
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
