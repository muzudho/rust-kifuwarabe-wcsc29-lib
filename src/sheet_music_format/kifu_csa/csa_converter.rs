use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::piece_etc::*;
use instrument::position::*;
use live::ohashi_player::*;
use sheet_music_format::kifu_csa::csa_move::*;
use sheet_music_format::kifu_csa::csa_tape::*;
use sound::shogi_note_operation::*;
use studio::address::*;
use studio::application::Application;

pub struct CsaConverter {}
impl CsaConverter {
    /// 変換には、初期局面が必要。
    pub fn play_out_csa_tape(
        ctape: &CsaTape,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        if app.is_debug() {
            app.comm.println("[#play_out_csa_tape:開始]");
        }

        // 大橋流を指すところから☆（*＾～＾*）
        OhashiPlayer::play_ohashi_starting(position, deck, app);
        if app.is_debug() {
            app.comm.println("[#play_out_csa_tape:大橋流終わり]");
            HumanInterface::bo(deck, &position, &app);
        }

        let mut ply = 1;
        for cmove in &ctape.items {
            if app.is_debug() {
                app.comm
                    .println(&format!("[#play_out_csa_tape: Ply: {}]", ply));
            }

            // 盤を動かしていく。
            CsaConverter::convert_csa_move(cmove, position, ply, deck, &app);

            ply += 1;
        }

        if app.is_debug() {
            app.comm.println("[#play_out_csa_tape:終了]");
        }
    }

    /// １ノートずつ盤を動かしながら、ノートを作ります。
    fn convert_csa_move(
        cmove: &CsaMove,
        position: &mut Position,
        ply: i16,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        /*
        if app.is_debug() {
            app.comm.println(&format!(
                "[#convert_csa_move:開始:Phase {:?}]",
                position.get_phase().get_state()
            ));
        }
        */

        let board_size = position.get_board_size();

        // change-phase
        {
            let change_phase = ShogiNoteOpe::change_phase(ply);

            /*
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#convert_csa_move:フェーズ始端:Phase {:?}]",
                    position.get_phase().get_state()
                ));
                HumanInterface::bo(deck, &position, &app);
            }
            */

            position.touch_1note_ope(
                deck,
                &change_phase,
                false,
                board_size,
                &app,
                "convert_csa_move(1)",
            );

            /*
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#convert_csa_move:タッチ直後:Phase {:?}]",
                    position.get_phase().get_state()
                ));
            }
            */

            /*
            if app.is_debug() {
                HumanInterface::bo(deck, &position, &app);
            }
            */
        }

        // 盤上の駒の番地。
        let destination_address = Address::from_cell(cmove.destination, position.get_board_size());

        if let Some(drop) = cmove.get_drop() {
            // 駒を打つ動きの場合

            // hand-off
            {
                let hand_off = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                    position.get_phase().get_state(),
                    drop,
                ));
                position.touch_1note_ope(
                    deck,
                    &hand_off,
                    false,
                    board_size,
                    &app,
                    "convert_csa_move(2)",
                );
                /*
                if app.is_debug() {
                    HumanInterface::bo(deck, &position, &app);
                }
                */
            }

            // hand-on
            {
                let hand_on = ShogiNoteOpe::from_address(destination_address);
                position.touch_1note_ope(
                    deck,
                    &hand_on,
                    false,
                    board_size,
                    &app,
                    "convert_csa_move(3)",
                );
                /*
                if app.is_debug() {
                    HumanInterface::bo(deck, &position, &app);
                }
                */
            }
        } else {
            // 駒を進める動きの場合
            if let Some(capture_id_piece) =
                position.get_id_piece_by_address(destination_address.get_index())
            {
                // 駒を取る動きが入る場合

                // hand-off
                {
                    let hand_off = ShogiNoteOpe::from_address(destination_address);
                    position.touch_1note_ope(
                        deck,
                        &hand_off,
                        false,
                        board_size,
                        &app,
                        "convert_csa_move(4)",
                    );
                    /*
                    if app.is_debug() {
                        HumanInterface::bo(deck, &position, &app);
                    }
                    */
                }

                // hand-rotate
                {
                    let hand_rotate = ShogiNoteOpe::rotate();
                    position.touch_1note_ope(
                        deck,
                        &hand_rotate,
                        false,
                        board_size,
                        &app,
                        "convert_csa_move(5)",
                    );
                    /*
                    if app.is_debug() {
                        HumanInterface::bo(deck, &position, &app);
                    }
                    */
                }

                // hand-turn
                if capture_id_piece.is_promoted() {
                    let hand_turn = ShogiNoteOpe::turn_over();
                    position.touch_1note_ope(
                        deck,
                        &hand_turn,
                        false,
                        board_size,
                        &app,
                        "convert_csa_move(6)",
                    );
                    /*
                    if app.is_debug() {
                        HumanInterface::bo(deck, &position, &app);
                    }
                    */
                }

                // hand-on
                {
                    let up = capture_id_piece.get_type();
                    let hand_on = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                        position.get_phase().get_state(),
                        up,
                    ));
                    /*
                    if app.is_debug() {
                        app.comm.println(&format!(
                            "[#convert_csa_move:駒台に置く:Phase {:?}]",
                            position.get_phase().get_value()
                        ));
                    }
                    */
                    position.touch_1note_ope(
                        deck,
                        &hand_on,
                        false,
                        board_size,
                        &app,
                        "convert_csa_move(7)",
                    );
                    /*
                    if app.is_debug() {
                        HumanInterface::bo(deck, &position, &app);
                    }
                    */
                }
            }

            /*
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#convert_csa_move:途中A:Phase {:?}]",
                    position.get_phase().get_state()
                ));
            }
            */

            // board-off
            {
                // 盤上の駒の番地。
                let board_off = ShogiNoteOpe::from_address(Address::from_cell(
                    cmove
                        .source
                        .unwrap_or_else(|| panic!(app.comm.panic("Fail. cmove.source."))),
                    position.get_board_size(),
                ));
                position.touch_1note_ope(
                    deck,
                    &board_off,
                    false,
                    board_size,
                    &app,
                    "convert_csa_move(8)",
                );
                /*
                if app.is_debug() {
                    HumanInterface::bo(deck, &position, &app);
                }
                */
            }

            // board-turn-over
            // 盤上にある駒が不成で、指し手の駒種類が成り駒なら、今、成った。
            let pre_promoted = if let Some(id_piece) = position.get_id_piece(
                cmove
                    .source
                    .unwrap_or_else(|| panic!(app.comm.panic("Fail. cmove.source."))),
            ) {
                id_piece.is_promoted()
            } else {
                false
            };
            let cur_promoted = is_promoted_piece_type(cmove.koma);
            if !pre_promoted && cur_promoted {
                let board_turn = ShogiNoteOpe::turn_over();
                position.touch_1note_ope(
                    deck,
                    &board_turn,
                    false,
                    board_size,
                    &app,
                    "convert_csa_move(9)",
                );
                /*
                if app.is_debug() {
                    HumanInterface::bo(deck, &position, &app);
                }
                */
            }

            // board-on
            {
                let board_on = ShogiNoteOpe::from_address(destination_address);
                position.touch_1note_ope(
                    deck,
                    &board_on,
                    false,
                    board_size,
                    &app,
                    "convert_csa_move(10)",
                );
                /*
                if app.is_debug() {
                    HumanInterface::bo(deck, &position, &app);
                }
                */
            }
        };

        /*
        if app.is_debug() {
            app.comm.println(&format!(
                "[#convert_csa_move:途中B:Phase {:?}]",
                position.get_phase().get_state()
            ));
        }
        */

        // change-phase
        {
            let change_phase = ShogiNoteOpe::change_phase(ply);
            /*
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#convert_csa_move:フェーズ終端:Phase {:?}]",
                    position.get_phase().get_state()
                ));
                HumanInterface::bo(deck, &position, &app);
            }
            */
            position.touch_1note_ope(
                deck,
                &change_phase,
                false,
                board_size,
                &app,
                "convert_csa_move(11)",
            );
            /*
            if app.is_debug() {
                HumanInterface::bo(deck, &position, &app);
            }
            */
        }

        /*
        if app.is_debug() {
            app.comm.println(&format!(
                "[#convert_csa_move:終了:Phase {:?}]",
                position.get_phase().get_state()
            ));
        }
        */
    }
}
