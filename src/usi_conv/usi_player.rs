use address::*;
use communication::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_player::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_recorder::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::RpmCassetteTapeBoxConveyor;
use kifu_rpm::thread::rpm_note_operation::*;
use position::*;
use usi_conv::usi_move::*;
use usi_conv::usi_record::*;

pub struct UsiPlayer {}
impl UsiPlayer {
    pub fn convert_move(umove: UsiMove, position: &Position, ply: i16) -> Vec<RpmNoteOpe> {
        let mut rpm_move = Vec::new();

        if umove.is_resign() {
            rpm_move.push(RpmNoteOpe::resign());
            return rpm_move;
        }

        let destination_address =
            Address::from_cell(umove.destination.unwrap(), position.get_board_size());

        match umove.get_drop() {
            Some(drop) => {
                // 駒を打つ動きの場合

                // hand-off
                let hand_off = RpmNoteOpe::from_address(Address::from_hand_ph_pt(
                    Some(position.get_phase()),
                    drop,
                ));
                rpm_move.push(hand_off);

                // hand-on
                let hand_on = RpmNoteOpe::from_address(destination_address);
                rpm_move.push(hand_on);
            }
            None => {
                // 駒を進める動きの場合
                if let Some(id_piece) =
                    position.get_id_piece_by_address(destination_address.get_index())
                {
                    // 駒を取る動きが入る場合

                    // hand-off
                    let hand_off = RpmNoteOpe::from_address(destination_address);
                    rpm_move.push(hand_off);

                    // hand-turn
                    if id_piece.is_promoted() {
                        let hand_turn = RpmNoteOpe::turn_over();
                        rpm_move.push(hand_turn);
                    }

                    // hand-rotate
                    let hand_rotate = RpmNoteOpe::rotate();
                    rpm_move.push(hand_rotate);

                    // hand-on
                    let up = id_piece.get_type();
                    let hand_on = RpmNoteOpe::from_address(Address::from_hand_ph_pt(
                        Some(position.get_phase()),
                        up,
                    ));
                    rpm_move.push(hand_on);
                }

                // board-off
                let board_off = RpmNoteOpe::from_address(Address::from_cell(
                    umove.source.unwrap(),
                    position.get_board_size(),
                ));
                rpm_move.push(board_off);

                // board-turn-over
                if umove.promotion {
                    let board_turn = RpmNoteOpe::turn_over();
                    rpm_move.push(board_turn);
                }

                // board-on
                let board_on = RpmNoteOpe::from_address(destination_address);
                rpm_move.push(board_on);
            }
        }

        // change-phase
        let change_phase = RpmNoteOpe::change_phase(ply);
        rpm_move.push(change_phase);

        rpm_move
    }

    /// # Arguments
    ///
    /// * 'position' - USIレコードと 初期局面を合わせてください。
    ///
    pub fn play_out_and_record(
        position: &mut Position,
        urecord: &UsiRecord,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeRecorder,
        comm: &Communication,
    ) {
        // 局面を動かしながら変換していく。
        let mut ply = 1;
        for umove in &urecord.moves {
            let rnote_opes = UsiPlayer::convert_move(*umove, position, ply);
            //comm.println(&format!("Pmoves len: {}.", rpm_move.len()));

            for rnote_ope in rnote_opes {
                //comm.println(&format!("Pmove: '{}'.", rpm_note.to_sign(position.get_board_size(), &mut ply)));
                comm.println("usi_player.rs:play_out_and_record: touch_brandnew_note");
                RpmCassetteTapePlayer::touch_brandnew_note(
                    &rnote_ope,
                    position,
                    tape_box_conveyor,
                    recorder,
                    comm,
                );
            }

            ply += 1;
        }
    }
}
