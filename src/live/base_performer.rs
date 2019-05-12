// シークと、タッチの両方を行うメソッドはここです。
use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::piece_etc::*;
use instrument::position::*;
use regex::Regex;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::ShogiNote;
use sound::shogi_note_operation::*;
use studio::application::Application;
use studio::common::caret::SoughtMoveResult;
use studio::common::caret::*;

pub struct BasePerformer {}
impl BasePerformer {
    // #####
    // # I #
    // #####

    /// Operation トラック文字列読取。
    pub fn improvise_by_line(
        line: &str,
        deck: &mut CassetteDeck,
        is_ok_illegal: bool,
        position: &mut Position,
        app: &Application,
    ) {
        let mut caret = Caret::new_facing_right_caret();

        loop {
            if caret.is_greater_than_or_equal_to(line.len() as i16) {
                return;
            }

            if let (_last_used_caret, Some(rnote_ope)) =
                ShogiNoteOpe::parse_1ope(&line, &mut caret, position.get_board_size(), &app)
            {
                if app.is_debug() {
                    app.comm.println(&format!(
                        "[#toush_by_line: {}]",
                        rnote_ope.to_human_presentable(position.get_board_size(), &app)
                    ));
                }
                BasePerformer::improvise_note_ope_no_log(
                    deck,
                    &rnote_ope,
                    is_ok_illegal,
                    position,
                    &app,
                );

                HumanInterface::bo(deck, &position, &app);
            }
        }
    }

    /// 棋譜を作る☆（＾～＾）
    /// 盤に触れて、
    /// ラーニング・テープに　棋譜も書くぜ☆（＾～＾）
    pub fn improvise_note_ope_no_log(
        deck: &mut CassetteDeck,
        rnote_ope: &ShogiNoteOpe,
        is_ok_illegal: bool,
        position: &mut Position,
        app: &Application,
    ) {
        // 盤を操作する。盤を触ると駒IDが分かる。それも返す。
        let id = match position.try_beautiful_touch_no_log(
            deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
            &rnote_ope,
            &app,
        ) {
            (is_legal_touch, Some(piece_identify)) => {
                if !is_legal_touch && !is_ok_illegal {
                    panic!(
                        "Illegal touch. PID: {}, Rnote: '{}'.",
                        piece_identify.to_human_presentable(),
                        rnote_ope.to_human_presentable(position.get_board_size(), &app)
                    )
                }

                PieceIdentify::from_number(piece_identify.get_id().get_number())
            }
            (is_legal_touch, None) => {
                // フェーズチェンジなどはここ。
                if !is_legal_touch && !is_ok_illegal {
                    panic!(
                        "Illegal touch. Rnote: '{}'.",
                        rnote_ope.to_human_presentable(position.get_board_size(), &app)
                    )
                }

                None
            }
        };

        let rnote = ShogiNote::from_id_ope(
            id,
            *rnote_ope,
            deck.is_facing_left_of_current_tape(Slot::Learning, &app),
        );

        // ラーニング・テープに１ノート挿入します。
        deck.insert_note(Slot::Learning, rnote, position.get_board_size(), app);
        // キャレットを１つ進めます。
        let (taken_overflow, awareness, note_opt) = deck.seek_a_note(Slot::Learning, &app);
        // 動作チェック。
        if let Some(aware_note) = note_opt {
            if aware_note != rnote {
                panic!(
                    "挿入失敗。expected:{}, actual:{}, {}{}.",
                    rnote,
                    aware_note,
                    if taken_overflow {
                        "Overflow, ".to_string()
                    } else {
                        "".to_string()
                    },
                    awareness.to_human_presentable()
                );
            }
        } else {
            panic!(
                "挿入失敗。None, {}{}.",
                if taken_overflow {
                    "Overflow, ".to_string()
                } else {
                    "".to_string()
                },
                awareness.to_human_presentable()
            );
        }
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
    pub fn pop_1note(
        deck: &mut CassetteDeck,
        position: &mut Position,
        app: &Application,
    ) -> Option<ShogiNote> {
        HumanInterface::bo(deck, position, &app);

        if let Some(rpm_note) = deck.delete_1note(Slot::Learning, &app) {
            let (_is_legal_touch, _piece_identify_opt) = position.try_beautiful_touch_no_log(
                deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
                &rpm_note.get_ope(),
                &app,
            );
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手削除する。
    ///
    /// TODO ply が変わる。
    pub fn pop_1move(deck: &mut CassetteDeck, position: &mut Position, app: &Application) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = BasePerformer::pop_1note(deck, position, app) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    // #####
    // # R #
    // #####

    /// テープに沿って1手分進める。（非合法タッチは自動で戻します）
    ///
    /// # Return
    ///
    /// (SoughtMoveResult, move)
    pub fn replay_a_move(
        deck: &mut CassetteDeck,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) -> (SoughtMoveResult, ShogiMove) {
        if app.is_debug() {
            app.comm
                .println(&format!("[#Deck.ReplayM: 開始, Slot:{:?}]", slot));
        }
        // 指し手（実際のところ、テープ上の範囲を示したもの）。
        let mut rmove = ShogiMove::new_facing_right_move();
        if !rmove.is_empty() {
            panic!(app.comm.panic(&format!(
                "[#Deck.ReplayM: Rmoveが最初から長さが0より大きかったら、おかしい☆（＾～＾）Rmove len:{}]",
                rmove.len())));
        }

        let mut is_rollback = false;
        let mut closed = false;

        'caret_loop: loop {
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#Deck.ReplayM {}]",
                    deck.to_human_presentable_of_caret(slot, &app)
                ))
            }

            // とりあえず、キャレットを１ノートずつ進めてみるぜ☆（*＾～＾*）
            match deck.seek_a_note(slot, &app) {
                (_taken_overflow, awareness, Some(rnote)) => {
                    if position.try_beautiful_touch(
                        deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
                        &rnote,
                        &app,
                    ) {
                        // ここに来たら、着手は成立☆（*＾～＾*）
                        /*
                        app.comm.println(&format!(
                            "[{} note advanced! Note:{}, Move:{}]",
                            rmove.len(),
                            rnote.to_human_presentable(position.get_board_size()),
                            rmove.to_human_presentable(self, slot, position.get_board_size(), &app)
                        ));
                        */

                        // タッチを完遂した場合のみ、このノートはムーブに含める☆（＾～＾）
                        // このムーブの長さが、進めたノートの数と等しいぜ☆（＾～＾）
                        // あとで、ルック・バックする範囲☆（＾～＾）
                        rmove
                            .caret_closed_interval
                            .intersect_2_values(awareness.passed_caret, awareness.expected_caret);

                        if 1 == rmove.len() && !rnote.is_phase_change() {
                            // １つ目で、フェーズ切り替えでなかった場合、読み取り位置がおかしい☆（＾～＾）
                            panic!(app.comm.panic(&format!(
                            "[#Deck.ReplayM: １つ目で、フェーズ切り替えでなかった場合、読み取り位置がおかしい☆（＾～＾）Move len:{}, Rnote:{}]",
                            rmove.len(),
                            rnote.to_human_presentable(position.get_board_size(),&app))));
                        } else if rnote.is_phase_change() && 1 < rmove.len() && rmove.len() < 4 {
                            panic!(app.comm.panic(&format!("[#Deck.ReplayM: ２つ目と３つ目に　フェーズ切り替え　が現れた場合、棋譜がおかしい☆（＾～＾）Move len:{}]",rmove.len())));
                        } else if app.is_debug() {
                            app.comm.println("[#Deck.ReplayM: ノート読めてる]");
                        }

                        if rnote.is_phase_change() && 3 < rmove.len() {
                            // ２回目のフェーズ切り替えで終了。
                            // 指し手は　２つ以上のノートを含むので、４つ目以降にあるはず。
                            // print!("[Phase-change-break try_read_1move:{}]", rnote);
                            closed = true;
                            break 'caret_loop;
                        }
                    } else {
                        // タッチは未着手だったので、ポジションは動いてない☆（＾～＾）キャレットは戻すぜ☆（＾～＾）
                        /*
                        if app.is_debug() {
                            app.comm.println(
                                "[#Deck.Seek a move: タッチは未着手だったので、キャレットは戻すぜ☆（＾～＾）]",
                            );
                        }
                        */
                        deck.look_back_caret(slot, &app);
                        deck.seek_a_note(slot, &app);
                        // ポジションは動かしてない☆（＾～＾）
                        deck.look_back_caret(slot, &app);
                        /*
                        if app.is_debug() {
                            app.comm.println(
                                "[#Deck.Seek a move: タッチは未着手だったので、キャレットは戻したぜ☆（＾～＾）]",
                            );
                        }
                        */

                        // 未着手なタッチならループを抜けて、今回進めた分を全部逆戻りさせるループへ進むぜ☆（＾～＾）
                        // app.comm.println("[$Untouched. Back a caret]");
                        is_rollback = true;
                        break 'caret_loop;
                    }
                }
                (_taken_overflow, awareness, None) => {
                    // オーバーフローを、読んだということだぜ☆（＾～＾）
                    if rmove.is_empty() {
                        if app.is_debug() {
                            app.comm.println(&format!("[#Deck.ReplayM: ノート読めない☆ オーバーフローのノートを読んでる☆（＾～＾） Awareness:{:?}]",awareness));
                        }

                        rmove
                            .caret_closed_interval
                            .intersect_2_values(awareness.passed_caret, awareness.expected_caret);
                        return (SoughtMoveResult::Forever, rmove);
                    } else {
                        panic!(app.comm.panic("[#Deck.ReplayM: 指し手を読んでる途中でオーバーフローが現れた場合、指し手が閉じられてない☆（＾～＾）棋譜がおかしい☆（＾～＾）]"));
                    }
                }
            }
        }

        // ここに来た時、ムーブの長さ＋１　分だけキャレットは進んでいる☆（＾～＾）

        if !closed {
            // おかしい☆（＾～＾）
            is_rollback = true;
        }

        if is_rollback {
            // キャレットを使って局面を戻す。
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#Deck.ReplayM: 巻き戻そう☆（＾～＾） Rollback {} note! Slot:{:?}, Move:{}]",
                    rmove.len(),
                    slot,
                    rmove.to_human_presentable(deck, slot, position.get_board_size(), &app)
                ));
            }
            deck.look_back_caret(Slot::Training, &app);
            deck.look_back_caret(Slot::Learning, &app);
            BasePerformer::synchronized_seek_and_touch_n_notes(deck, rmove.len(), position, &app);
            deck.look_back_caret(Slot::Training, &app);
            deck.look_back_caret(Slot::Learning, &app);

            return (SoughtMoveResult::Dream, ShogiMove::new_facing_right_move());
        }

        // 1手分。
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Deck.ReplayM: １手分☆（＾～＾） Move:{}]",
                rmove.to_human_presentable(deck, slot, position.get_board_size(), &app)
            ));
        }
        (SoughtMoveResult::Aware, rmove)
    }

    pub fn read_tape_for_n_moves_forcely(
        deck: &mut CassetteDeck,
        slot: Slot,
        repeats: usize,
        position: &mut Position,
        app: &Application,
    ) {
        for _i in 0..repeats {
            BasePerformer::read_tape_for_1move_forcely(deck, slot, position, &app);
        }
    }

    /// 必ず1手進める。（非合法タッチがあれば強制終了）
    pub fn read_tape_for_1move_forcely(
        deck: &mut CassetteDeck,
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
        while let (_taken_overflow, _awareness, Some(rnote)) = deck.seek_a_note(slot, &app) {
            /*
            app.comm.println(&format!(
                "[LOOP read_tape_for_1move_forcely:{}:{}:{}]",
                self.to_human_presentable_of_caret_of_current_tape_of_training_box(&app),
                self.to_human_presentable_of_tape_box(slot),
                rnote.to_human_presentable(position.get_board_size())
            ));
            */
            is_legal_touch = position.try_beautiful_touch(
                deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
                &rnote,
                &app,
            );
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
            panic!(app.comm.panic("Illegal, go opponent forcely!"));
        }

        // 1つも読まなかったら強制終了。
        if forwarding_count < 1 {
            panic!(app.comm.panic("Illegal, zero foward!"));
        }
    }

    // #####
    // # S #
    // #####

    pub fn scan_pid(
        line: &str,
        deck: &mut CassetteDeck,
        position: &mut Position,
        app: &Application,
    ) {
        let re = Regex::new(r"scan-pid\s+(\d+)")
            .unwrap_or_else(|f| panic!(app.comm.panic(&f.to_string())));
        let matched = re
            .captures(line)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. parse.")));
        let pnum_str = matched.get(1).map_or("", |m| m.as_str());
        let pnum: i8 = pnum_str.parse().unwrap();
        let pid = if let Some(pid) = PieceIdentify::from_number(pnum) {
            pid
        } else {
            app.comm
                .println(&format!("[#Scan pid fail: Pnum: {}]", pnum));
            return;
        };

        // 記録係フェーズなんで、もう１つ先に進めるぜ☆（＾～＾）
        position.seek_a_player(
            deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
            &app,
        );

        if let Some((idp, addr)) = position.scan_pid(position.get_phase().get_state(), pid) {
            app.comm.println(&format!(
                "[#Scan pid: Found pnum:{}, Idp:{}, Addr:{}]",
                pnum,
                idp.to_human_presentable(),
                addr.to_human_presentable(position.get_board_size())
            ));
        } else {
            app.comm
                .println(&format!("[#Scan pid: Not found pnum: {}]", pnum));
        }

        // 進めた分を戻すぜ☆（＾～＾）
        deck.look_back_caret(Slot::Learning, &app);
        position.seek_a_player(
            deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
            &app,
        );
        deck.look_back_caret(Slot::Learning, &app);
    }

    /// トレーニング・テープと、ラーニング・テープを同時に n ノート シークします。
    /// また、盤面へのタッチも行います。
    pub fn synchronized_seek_and_touch_n_notes(
        deck: &mut CassetteDeck,
        repeat: usize,
        position: &mut Position,
        app: &Application,
    ) {
        if app.is_debug() {
            app.comm
                .println("[#synchronized_seek_and_touch_n_notes 開始]");
        }

        for _i in 0..repeat {
            // トレーニング・テープをシークする☆（＾～＾）結果は要らない☆（＾～＾）
            let (taken_overflow_t, awareness_t, note_opt_t) =
                deck.seek_a_note(Slot::Training, &app);
            if let Some(_note) = note_opt_t {

            } else if app.is_debug() {
                app.comm.println(&format!("[#synchronized_seek_and_touch_n_notes: トレーニング・テープが途切れた☆（＾～＾）？ taken_overflow:{}, Awareness:{:?}]",
                        taken_overflow_t,
                        awareness_t),
                    );
            }

            // ラーニング・テープをシークする☆（＾～＾）結果は、盤面を巻き戻すのに使う☆（＾～＾）
            let (taken_overflow_l, awareness_l, note_opt_l) =
                deck.seek_a_note(Slot::Learning, &app);
            if let Some(note) = note_opt_l {
                /*
                app.comm.println(&format!(
                    "<Go-force:{}/{} {}>",
                    i,
                    repeat,
                    rnote.to_human_presentable(position.get_board_size())
                ));
                */
                if !position.try_beautiful_touch(
                    deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
                    &note,
                    &app,
                ) {
                    /*
                    app.comm.println(&format!(
                        "Touch fail, permissive. Note: {}, Caret: {}.",
                        rnote.to_human_presentable(position.get_board_size()),
                        self.to_human_presentable_of_caret_of_current_tape_of_training_box(&app),
                    ));
                    */
                }
            } else if app.is_debug() {
                app.comm.println(&format!("[#synchronized_seek_and_touch_n_notes: ラーニング・テープが途切れた☆（＾～＾）？ taken_overflow:{}, Awareness:{:?}]",
                        taken_overflow_l,
                        awareness_l),
                    );
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

        if app.is_debug() {
            app.comm
                .println("[#synchronized_seek_and_touch_n_notes 終了]");
        }
    }

    /// ラーニング・テープを n ノート シークします。
    /// また、タッチも行います。成立しないタッチをしてしまうことも、おおめに見ます。
    pub fn seek_and_touch_learning_n_notes_permissive(
        deck: &mut CassetteDeck,
        repeat: usize,
        position: &mut Position,
        app: &Application,
    ) {
        if app.is_debug() {
            app.comm
                .println("[#seek_and_touch_learning_n_notes_permissive 開始]");
        }

        for _i in 0..repeat {
            // キャレットは必ず進めろだぜ☆（＾～＾）
            if let (_taken_overflow, _awareness, Some(rnote)) =
                deck.seek_a_note(Slot::Learning, &app)
            {
                // 指し手を拾えたのなら、指せだぜ☆（＾～＾）
                /*
                app.comm.println(&format!(
                    "<Go-force:{}/{} {}>",
                    i,
                    repeat,
                    rnote.to_human_presentable(position.get_board_size())
                ));
                */
                if !position.try_beautiful_touch(
                    deck.slots[Slot::Learning as usize].is_facing_left_of_current_tape(),
                    &rnote,
                    &app,
                ) {
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

        if app.is_debug() {
            app.comm
                .println("[#seek_and_touch_learning_n_notes_permissive 終了]");
        }
    }
}
