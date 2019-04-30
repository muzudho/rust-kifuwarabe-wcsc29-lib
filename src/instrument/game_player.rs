use human::human_interface::*;
use instrument::position::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note_operation::*;
use std::*;
use studio::application::Application;
use studio::common::caret::*;
use video_recorder::cassette_deck::CassetteDeck;
use video_recorder::cassette_deck::*;
use video_recorder::cassette_tape_box::CassetteTapeBox;

pub struct GamePlayer {}
impl GamePlayer {
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
    /// (is_end_of_tape, 指した１手分)
    pub fn try_read_tape_for_1move(
        deck: &mut CassetteDeck,
        slot: Slot,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) -> (bool, Option<ShogiMove>) {
        // 指し手（実際のところ、テープ上の範囲を示したもの）。
        let mut rmove = ShogiMove::new_facing_right_move();

        let mut is_rollback = false;
        let mut is_phase_change = false;

        // とりあえず、フェーズ切り替えするまで、キャレットを１ノートずつ進めてみるぜ☆（*＾～＾*）
        while let (caret_number, Some(rnote)) = deck.go_to_next(slot, &app) {
            // 範囲も広げるぜ☆（＾～＾）このムーブの長さが、進めたノートの数と等しいぜ☆（＾～＾）

            app.comm.println(&format!(
                "[{} note advanced! Note:{}, Move:{}]",
                rmove.len(),
                rnote.to_human_presentable(position.get_board_size()),
                rmove.to_human_presentable(deck, slot, position.get_board_size(), &app)
            ));

            if !position.try_beautiful_touch(&rnote, ply, &app) {
                // 未着手なタッチならループを抜けて、今回進めた分を全部逆戻りさせるループへ進むぜ☆（＾～＾）
                is_rollback = true;
                break;
            }

            // ここに来たら、着手は成立☆（*＾～＾*）

            rmove
                .caret_closed_interval
                .intersect_caret_number(caret_number);

            if 1 < rmove.len() && rnote.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                print!("[Phase-change-break try_read_1move:{}]", rnote);
                is_phase_change = true;
                break;
            }
        }

        // ここに来た時、ムーブの長さ＋１　分だけキャレットは進んでいる☆（＾～＾）

        if rmove.is_empty() {
            // ループに入らなかったのなら トラックの終わり。キャレットを１戻す。
            deck.turn_caret_to_opponent(slot);
            deck.go_to_next(slot, &app);
            deck.turn_caret_to_opponent(slot);
            return (true, None);
        }

        if is_rollback {
            // 非合法タッチを自動で戻す。
            app.comm.println("[Try_read_1move: Rollback!]");
            deck.turn_caret_to_opponent(slot);
            GamePlayer::read_tape_for_n_notes_forcely(
                deck,
                slot,
                // ムーブの長さの１つ多めに戻すぜ☆（＾～＾）
                rmove.len() as u16 + 1,
                position,
                ply,
                &app,
            );
            deck.turn_caret_to_opponent(slot);

            return (false, None);
        }

        if is_phase_change {
            // 1手分。
            (false, Some(rmove))
        } else {
            // 最後の1手分。
            (true, Some(rmove))
        }
    }

    pub fn read_tape_for_n_moves_forcely(
        tape_box: &mut CassetteTapeBox,
        repeats: i16,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) {
        for _i in 0..repeats {
            GamePlayer::read_tape_for_1move_forcely(tape_box, position, ply, &app);
        }
    }

    /// 必ず1手進める。（非合法タッチがあれば強制終了）
    pub fn read_tape_for_1move_forcely(
        tape_box: &mut CassetteTapeBox,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) {
        app.comm.println(&format!(
            "[read_tape_for_1move_forcely:{}]",
            tape_box.to_human_presentable(),
        ));

        let mut is_legal_touch = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら終了。
        while let (_caret_number, Some(rnote)) = tape_box.go_to_next(&app) {
            app.comm.println(&format!(
                "<LOOP read_tape_for_1move_forcely:{}:{}>",
                tape_box.to_human_presentable(),
                rnote.to_human_presentable(position.get_board_size())
            ));
            is_legal_touch = position.try_beautiful_touch(&rnote, ply, &app);
            forwarding_count += 1;

            if !is_legal_touch {
                break;
            }

            if forwarding_count != 1 && rnote.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                print!("<NXm1End{} {}>", forwarding_count, rnote);
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

    /// 非合法手はない前提で、強制的にテープを進めます。
    pub fn read_tape_for_n_notes_forcely(
        deck: &mut CassetteDeck,
        slot: Slot,
        repeat: u16,
        position: &mut Position,
        ply: i16,
        app: &Application,
    ) {
        for i in 0..repeat {
            if let (_caret_number, Some(rnote)) = deck.go_to_next(slot, &app) {
                app.comm
                    .println(&format!("<Go-force:{}/{} {}>", i, repeat, rnote));
                if !position.try_beautiful_touch(&rnote, ply, &app) {
                    panic!("Touch fail forcely.");
                }
            } else {
                panic!("<Go forcely fail:{}/{} None>", i, repeat);
            }
        }
    }

    /// Operation トラック文字列読取。
    pub fn read_ope_track(
        line: &str,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        let mut caret = Caret::new_facing_right_caret();

        loop {
            if caret.is_greater_than_or_equal_to(line.len() as i16) {
                return;
            }

            let tuple =
                ShogiNoteOpe::parse_1ope(&line, &mut caret, position.get_board_size(), &app);

            if let (_last_used_caret, Some(rnote_ope)) = tuple {
                app.comm
                    .println("rpm_cassette_tape_editor.rs:read_ope_track: touch_1note_ope");
                position.touch_1note_ope(&rnote_ope, deck, &app);

                HumanInterface::bo(deck, Slot::Learning, &position, &app);
            }
        }
    }
}
