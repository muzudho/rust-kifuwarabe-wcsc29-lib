use address::*;
use communication::*;
use kifu_csa::csa_move::*;
use kifu_csa::csa_record::*;
use kifu_rpm::play::rpm_move_player::*;
use kifu_rpm::play::rpm_note_player::*;
use kifu_rpm::rpm_cassette_tape_recorder::*;
use kifu_rpm::thread::rpm_note_operation::*;
use piece_etc::*;
use position::*;

pub struct CsaPlayer {}
impl CsaPlayer {
    /// 変換には、現局面が必要。
    pub fn convert_move(
        _comm: &Communication,
        cmove: &CsaMove,
        position: &Position,
        ply: i16,
    ) -> Vec<RpmNoteOpe> {
        let mut p_moves = Vec::new();

        // 盤上の駒の番地。
        let destination_address = Address::from_cell(cmove.destination, position.get_board_size());

        if let Some(drop) = cmove.get_drop() {
            // 駒を打つ動きの場合

            // hand-off
            let hand_off = RpmNoteOpe::from_address(Address::from_hand_ph_pt(
                Some(position.get_phase()),
                drop,
            ));
            p_moves.push(hand_off);

            // hand-on
            let hand_on = RpmNoteOpe::from_address(destination_address);
            p_moves.push(hand_on);
        } else {
            // 駒を進める動きの場合
            if let Some(capture_id_piece) =
                position.get_id_piece_by_address(destination_address.get_index())
            {
                // 駒を取る動きが入る場合

                // hand-off
                let hand_off = RpmNoteOpe::from_address(destination_address);
                p_moves.push(hand_off);

                // hand-turn
                if capture_id_piece.is_promoted() {
                    let hand_turn = RpmNoteOpe::turn_over();
                    p_moves.push(hand_turn);
                }

                // hand-rotate
                let hand_rotate = RpmNoteOpe::rotate();
                p_moves.push(hand_rotate);

                // hand-on
                let up = capture_id_piece.get_type();
                let hand_on = RpmNoteOpe::from_address(Address::from_hand_ph_pt(
                    Some(position.get_phase()),
                    up,
                ));
                p_moves.push(hand_on);
            }

            // board-off
            // 盤上の駒の番地。
            let board_off = RpmNoteOpe::from_address(Address::from_cell(
                cmove.source.unwrap(),
                position.get_board_size(),
            ));
            p_moves.push(board_off);

            // board-turn-over
            // 盤上にある駒が不成で、指し手の駒種類が成り駒なら、今、成った。
            let pre_promoted = if let Some(id_piece) = position.get_id_piece(cmove.source.unwrap())
            {
                id_piece.is_promoted()
            } else {
                false
            };
            let cur_promoted = is_promoted_piece_type(cmove.koma);
            if !pre_promoted && cur_promoted {
                let board_turn = RpmNoteOpe::turn_over();
                p_moves.push(board_turn);
            }

            // board-on
            let board_on = RpmNoteOpe::from_address(destination_address);
            p_moves.push(board_on);
        };

        // change-phase
        let change_phase = RpmNoteOpe::change_phase(ply);
        p_moves.push(change_phase);

        p_moves
    }

    /// 変換には、初期局面が必要。
    pub fn play_out_and_record(
        comm: &Communication,
        position: &mut Position,
        crecord: &CsaRecord,
    ) -> RpmCassetteTapeRecorder {
        // TODO とりあえず平手初期局面だけ対応。
        let mut recorder = RpmCassetteTapeRecorder::default();
        position.reset_origin_position();
        RpmMovePlayer::record_ohashi_starting(comm, &mut recorder, position);

        let mut ply = 1;
        for cmove in &crecord.items {
            let rnote_opes = CsaPlayer::convert_move(comm, cmove, position, ply);

            for rnote_ope in rnote_opes {
                comm.println("csa_player.rs: touch_brandnew_note");
                RpmNotePlayer::touch_brandnew_note(&mut recorder, &rnote_ope, position, comm);
            }

            ply += 1;
        }

        recorder
    }
}
