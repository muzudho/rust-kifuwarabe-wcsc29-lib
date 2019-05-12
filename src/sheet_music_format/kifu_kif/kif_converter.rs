use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::piece_etc::*;
use instrument::position::*;
use live::base_performer::*;
use live::ohashi_performer::*;
use sheet_music_format::kifu_kif::kif_move::*;
use sheet_music_format::kifu_kif::kif_tape::*;
use sound::shogi_note_operation::*;
use studio::address::*;
use studio::application::Application;

pub struct KifConverter {}
impl KifConverter {
    /// 変換には、初期局面が必要。
    pub fn play_out_kifu_tape(
        ktape: &KifTape,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        // 大橋流を指すところから☆（*＾～＾*）
        OhashiPerformer::improvise_ohashi_starting(deck, position, &app);

        let mut ply = 1;
        for kmove in &ktape.items {
            let rnote_opes = KifConverter::convert_move(kmove, position, ply, &app);

            for rnote_ope in rnote_opes {
                BasePerformer::improvise_note_ope_no_log(deck, &rnote_ope, false, position, &app);
                HumanInterface::bo(deck, position, &app);
            }

            ply += 1;
        }
    }

    /// 変換には、現局面が必要。
    pub fn convert_move(
        kmove: &KifMove,
        position: &Position,
        ply: i16,
        app: &Application,
    ) -> Vec<ShogiNoteOpe> {
        let mut rmoves = Vec::new();

        // ####################
        // # (0) Change phase #
        // ####################
        {
            rmoves.push(ShogiNoteOpe::change_phase(ply));
        }

        let destination_address = Address::from_cell(
            kmove
                .destination
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. kmove.destination."))),
            position.get_board_size(),
        );

        if kmove.is_drop {
            // 1,4 は駒を打つ(drop)動きの場合

            // #################
            // # (1d) Hand off #
            // #################
            {
                let piece_type = jsa_piece_type_to_perfect(kmove.piece);
                let piece = Piece::from_ph_pt(
                    position.get_phase().get_state(),
                    piece_type.unwrap_or_else(|| panic!(app.comm.panic("Fail. piece_type."))),
                );
                let drop = position.peek_hand(piece);

                let hand_off = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                    position.get_phase().get_state(),
                    drop.unwrap_or_else(|| panic!(app.comm.panic("Fail. drop.")))
                        .get_type(),
                ));
                rmoves.push(hand_off);
            }

            // ################
            // # (4d) Hand on #
            // ################
            {
                let hand_on = ShogiNoteOpe::from_address(destination_address);
                rmoves.push(hand_on);
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
                    rmoves.push(hand_off);
                }

                // #################
                // # (2) Hand turn #
                // #################
                if capture_id_piece.is_promoted() {
                    let hand_turn = ShogiNoteOpe::turn_over();
                    rmoves.push(hand_turn);
                }

                // ###################
                // # (3) Hand rotate #
                // ###################
                {
                    let hand_rotate = ShogiNoteOpe::rotate();
                    rmoves.push(hand_rotate);
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
                    rmoves.push(hand_on);
                }
            }

            // 5～7は、盤上の駒を進める動き。

            // #################
            // # (5) Board off #
            // #################
            {
                let board_off = ShogiNoteOpe::from_address(Address::from_cell(
                    kmove
                        .source
                        .unwrap_or_else(|| panic!(app.comm.panic("Fail. kmove.source."))),
                    position.get_board_size(),
                ));
                rmoves.push(board_off);
            }

            // #######################
            // # (6) Board turn over #
            // #######################
            {
                // 盤上にある駒が不成で、指し手の駒種類が成り駒なら、今、成った。
                if let Some(id_piece) = position.get_id_piece(
                    kmove
                        .source
                        .unwrap_or_else(|| panic!(app.comm.panic("Fail. position.get_id_piece."))),
                ) {
                    id_piece.is_promoted()
                } else {
                    false
                };

                if kmove.is_promote {
                    let board_turn = ShogiNoteOpe::turn_over();
                    rmoves.push(board_turn);
                }
            }

            // ################
            // # (7) Board on #
            // ################
            {
                let board_on = ShogiNoteOpe::from_address(destination_address);
                rmoves.push(board_on);
            }
        };

        // ####################
        // # (8) Change phase #
        // ####################
        {
            rmoves.push(ShogiNoteOpe::change_phase(ply));
        }

        rmoves
    }
}
