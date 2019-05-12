use audio_compo::audio_rack::*;
use human::human_interface::*;
use instrument::piece_etc::*;
use instrument::position::*;
use live::base_performer::*;
use live::ohashi_performer::*;
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
        rack: &mut AudioRack,
        position: &mut Position,
        app: &Application,
    ) {
        if app.is_debug() {
            app.comm.println("[#play_out_csa_tape:開始]");
        }

        // 大橋流を指すところから☆（*＾～＾*）
        OhashiPerformer::improvise_ohashi_starting(rack, position, app);
        if app.is_debug() {
            app.comm.println("[#play_out_csa_tape:大橋流終わり]");
            HumanInterface::bo(rack, &position, &app);
        }

        let mut ply = 1;
        for cmove in &ctape.items {
            if app.is_debug() {
                app.comm
                    .println(&format!("[#play_out_csa_tape: Ply: {}]", ply));
            }

            // 盤を動かしていく。
            CsaConverter::convert_csa_move(cmove, rack, position, ply, &app);

            ply += 1;
        }

        if app.is_debug() {
            app.comm.println("[#play_out_csa_tape:終了]");
        }
    }

    /// １ノートずつ盤を動かしながら、ノートを作ります。
    fn convert_csa_move(
        cmove: &CsaMove,
        rack: &mut AudioRack,
        position: &mut Position,
        ply: i16,
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

        // ####################
        // # (0) Change phase #
        // ####################
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

            BasePerformer::improvise_note_ope_no_log(rack, &change_phase, false, position, &app);
            HumanInterface::bo(rack, position, &app);

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
            // 1,4 は駒を打つ(drop)動きの場合

            // #################
            // # (1d) Hand off #
            // #################
            {
                let hand_off = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                    position.get_phase().get_state(),
                    drop,
                ));
                BasePerformer::improvise_note_ope_no_log(rack, &hand_off, false, position, &app);
                HumanInterface::bo(rack, position, &app);
                /*
                if app.is_debug() {
                    HumanInterface::bo(deck, &position, &app);
                }
                */
            }

            // ################
            // # (4d) Hand on #
            // ################
            {
                let hand_on = ShogiNoteOpe::from_address(destination_address);
                BasePerformer::improvise_note_ope_no_log(rack, &hand_on, false, position, &app);
                HumanInterface::bo(rack, position, &app);
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
                // 1～4は、駒を取る(capture)動き。

                // #################
                // # (1c) Hand off #
                // #################
                {
                    let hand_off = ShogiNoteOpe::from_address(destination_address);
                    BasePerformer::improvise_note_ope_no_log(
                        rack, &hand_off, false, position, &app,
                    );
                    HumanInterface::bo(rack, position, &app);
                    /*
                    if app.is_debug() {
                        HumanInterface::bo(deck, &position, &app);
                    }
                    */
                }

                // #################
                // # (2) Hand turn #
                // #################
                if capture_id_piece.is_promoted() {
                    let hand_turn = ShogiNoteOpe::turn_over();
                    BasePerformer::improvise_note_ope_no_log(
                        rack, &hand_turn, false, position, &app,
                    );
                    HumanInterface::bo(rack, position, &app);
                    /*
                    if app.is_debug() {
                        HumanInterface::bo(deck, &position, &app);
                    }
                    */
                }

                // ###################
                // # (3) Hand rotate #
                // ###################
                {
                    let hand_rotate = ShogiNoteOpe::rotate();
                    BasePerformer::improvise_note_ope_no_log(
                        rack,
                        &hand_rotate,
                        false,
                        position,
                        &app,
                    );
                    HumanInterface::bo(rack, position, &app);
                    /*
                    if app.is_debug() {
                        HumanInterface::bo(deck, &position, &app);
                    }
                    */
                }

                // ################
                // # (4c) Hand on #
                // ################
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
                    BasePerformer::improvise_note_ope_no_log(rack, &hand_on, false, position, &app);
                    HumanInterface::bo(rack, position, &app);
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

            // 5～7は、盤上の駒を進める動き。

            // #################
            // # (5) Board off #
            // #################
            {
                // 盤上の駒の番地。
                let board_off = ShogiNoteOpe::from_address(Address::from_cell(
                    cmove
                        .source
                        .unwrap_or_else(|| panic!(app.comm.panic("Fail. cmove.source."))),
                    position.get_board_size(),
                ));
                BasePerformer::improvise_note_ope_no_log(rack, &board_off, false, position, &app);
                HumanInterface::bo(rack, position, &app);
                /*
                if app.is_debug() {
                    HumanInterface::bo(rack, &position, &app);
                }
                */
            }

            // #######################
            // # (6) Board turn over #
            // #######################
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
                BasePerformer::improvise_note_ope_no_log(rack, &board_turn, false, position, &app);
                HumanInterface::bo(rack, position, &app);
                /*
                if app.is_debug() {
                    HumanInterface::bo(rack, &position, &app);
                }
                */
            }

            // ################
            // # (7) Board on #
            // ################
            {
                let board_on = ShogiNoteOpe::from_address(destination_address);
                BasePerformer::improvise_note_ope_no_log(rack, &board_on, false, position, &app);
                HumanInterface::bo(rack, position, &app);
                /*
                if app.is_debug() {
                    HumanInterface::bo(rack, &position, &app);
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

        // ####################
        // # (8) Change phase #
        // ####################
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
            BasePerformer::improvise_note_ope_no_log(rack, &change_phase, false, position, &app);
            HumanInterface::bo(rack, position, &app);
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
