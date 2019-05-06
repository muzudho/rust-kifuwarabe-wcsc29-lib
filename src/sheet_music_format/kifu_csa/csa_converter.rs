use audio_compo::cassette_deck::*;
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
        crecord: &CsaTape,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        let board_size = position.get_board_size();

        // 大橋流を指すところから☆（*＾～＾*）
        OhashiPlayer::play_ohashi_starting(position, deck, app);

        let mut ply = 1;
        for cmove in &crecord.items {
            let rnote_opes = CsaConverter::convert_move(cmove, position, ply, &app);

            for rnote_ope in rnote_opes {
                position.touch_1note_ope(deck, &rnote_ope, false, board_size, &app);
            }

            ply += 1;
        }
    }

    /// 変換には、現局面が必要。
    pub fn convert_move(
        cmove: &CsaMove,
        position: &Position,
        ply: i16,
        app: &Application,
    ) -> Vec<ShogiNoteOpe> {
        let mut p_moves = Vec::new();

        // change-phase
        p_moves.push(ShogiNoteOpe::change_phase(ply));

        // 盤上の駒の番地。
        let destination_address = Address::from_cell(cmove.destination, position.get_board_size());

        if let Some(drop) = cmove.get_drop() {
            // 駒を打つ動きの場合

            // hand-off
            let hand_off = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                position.get_phase().get_value(),
                drop,
            ));
            p_moves.push(hand_off);

            // hand-on
            let hand_on = ShogiNoteOpe::from_address(destination_address);
            p_moves.push(hand_on);
        } else {
            // 駒を進める動きの場合
            if let Some(capture_id_piece) =
                position.get_id_piece_by_address(destination_address.get_index())
            {
                // 駒を取る動きが入る場合

                // hand-off
                let hand_off = ShogiNoteOpe::from_address(destination_address);
                p_moves.push(hand_off);

                // hand-rotate
                let hand_rotate = ShogiNoteOpe::rotate();
                p_moves.push(hand_rotate);

                // hand-turn
                if capture_id_piece.is_promoted() {
                    let hand_turn = ShogiNoteOpe::turn_over();
                    p_moves.push(hand_turn);
                }

                // hand-on
                let up = capture_id_piece.get_type();
                let hand_on = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                    position.get_phase().get_value(),
                    up,
                ));
                p_moves.push(hand_on);
            }

            // board-off
            // 盤上の駒の番地。
            let board_off = ShogiNoteOpe::from_address(Address::from_cell(
                cmove
                    .source
                    .unwrap_or_else(|| panic!(app.comm.panic("Fail. cmove.source."))),
                position.get_board_size(),
            ));
            p_moves.push(board_off);

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
                p_moves.push(board_turn);
            }

            // board-on
            let board_on = ShogiNoteOpe::from_address(destination_address);
            p_moves.push(board_on);
        };

        // change-phase
        p_moves.push(ShogiNoteOpe::change_phase(ply));

        p_moves
    }
}
