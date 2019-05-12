use audio_compo::cassette_deck::*;
use instrument::piece_etc::*;
use instrument::position::*;
use regex::Regex;
use sound::shogi_move::ShogiMove;
use studio::application::Application;
use studio::common::caret::SoughtMoveResult;

pub struct BasePerformer {}
impl BasePerformer {
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
                    if position.try_beautiful_touch(&deck, &rnote, &app) {
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
            deck.synchronized_seek_and_touch_n_notes(rmove.len(), position, &app);
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
}
