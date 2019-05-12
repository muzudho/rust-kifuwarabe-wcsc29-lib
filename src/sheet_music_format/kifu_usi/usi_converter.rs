use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::position::*;
use live::base_performer::*;
use sheet_music_format::kifu_usi::usi_move::*;
use sheet_music_format::kifu_usi::usi_tape::*;
use sound::shogi_note_operation::*;
use studio::address::*;
use studio::application::Application;

pub struct UsiConverter {}
impl UsiConverter {
    /// # Arguments
    ///
    /// * 'position' - USIレコードと 初期局面を合わせてください。
    ///
    pub fn play_out_usi_tape(
        position: &mut Position,
        utape: &UsiTape,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        // 局面を動かしながら変換していく。
        let mut ply = 1;
        for umove in &utape.moves {
            let rnote_opes = UsiConverter::convert_move(*umove, position, ply, &app);
            //comm.println(&format!("Pmoves len: {}.", rpm_move.len()));

            for rnote_ope in rnote_opes {
                BasePerformer::improvise_note_ope_no_log(deck, &rnote_ope, false, position, &app);
                HumanInterface::bo(deck, position, &app);
            }

            ply += 1;
        }
    }

    pub fn convert_move(
        umove: UsiMove,
        position: &Position,
        ply: i16,
        app: &Application,
    ) -> Vec<ShogiNoteOpe> {
        let mut rpm_move = Vec::new();

        // ####################
        // # (0) Change phase #
        // ####################
        {
            rpm_move.push(ShogiNoteOpe::change_phase(ply));
        }

        if umove.is_resign() {
            rpm_move.push(ShogiNoteOpe::resign());

            // change-phase
            rpm_move.push(ShogiNoteOpe::change_phase(ply));
            return rpm_move;
        }

        let destination_address = Address::from_cell(
            umove
                .destination
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. umove.destination."))),
            position.get_board_size(),
        );

        match umove.get_drop() {
            Some(drop) => {
                // 1,4 は駒を打つ(drop)動きの場合

                // #################
                // # (1d) Hand off #
                // #################
                {
                    let hand_off = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                        position.get_phase().get_state(),
                        drop,
                    ));
                    rpm_move.push(hand_off);
                }

                // ################
                // # (4d) Hand on #
                // ################
                {
                    let hand_on = ShogiNoteOpe::from_address(destination_address);
                    rpm_move.push(hand_on);
                }
            }
            None => {
                // 駒を進める動きの場合
                if let Some(id_piece) =
                    position.get_id_piece_by_address(destination_address.get_index())
                {
                    // 1～4は、駒を取る(capture)動き。

                    // #################
                    // # (1c) Hand off #
                    // #################
                    {
                        let hand_off = ShogiNoteOpe::from_address(destination_address);
                        rpm_move.push(hand_off);
                    }

                    // #################
                    // # (2) Hand turn #
                    // #################
                    if id_piece.is_promoted() {
                        let hand_turn = ShogiNoteOpe::turn_over();
                        rpm_move.push(hand_turn);
                    }

                    // ###################
                    // # (3) Hand rotate #
                    // ###################
                    {
                        let hand_rotate = ShogiNoteOpe::rotate();
                        rpm_move.push(hand_rotate);
                    }

                    // ################
                    // # (4c) Hand on #
                    // ################
                    {
                        let up = id_piece.get_type();
                        let hand_on = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                            position.get_phase().get_state(),
                            up,
                        ));
                        rpm_move.push(hand_on);
                    }
                }

                // 5～7は、盤上の駒を進める動き。

                // #################
                // # (5) Board off #
                // #################
                {
                    let board_off = ShogiNoteOpe::from_address(Address::from_cell(
                        umove
                            .source
                            .unwrap_or_else(|| panic!(app.comm.panic("Fail. umove.source."))),
                        position.get_board_size(),
                    ));
                    rpm_move.push(board_off);
                }

                // #######################
                // # (6) Board turn over #
                // #######################
                if umove.promotion {
                    let board_turn = ShogiNoteOpe::turn_over();
                    rpm_move.push(board_turn);
                }

                // ################
                // # (7) Board on #
                // ################
                {
                    let board_on = ShogiNoteOpe::from_address(destination_address);
                    rpm_move.push(board_on);
                }
            }
        }

        // ####################
        // # (8) Change phase #
        // ####################
        {
            rpm_move.push(ShogiNoteOpe::change_phase(ply));
        }

        rpm_move
    }
}
