use address::*;
use communication::*;
use position::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_conv::rpm_record::*;
use rpm_play::rpm_player::*;
use usi_conv::usi_move::*;
use usi_conv::usi_record::*;

pub struct UsiConverter {

}
impl UsiConverter {
    pub fn convert_move(umove:UsiMove, position:&Position) -> Vec<RpmNoteOpe> {
        let mut rpm_move = Vec::new();

        if umove.is_resign() {
            rpm_move.push(RpmNoteOpe::resign());
            return rpm_move;
        }

        let destination_address = Address::from_cell(
            umove.destination.unwrap(),
            position.get_board_size());
        
        match umove.get_drop()
        {
            Some(drop) => {
                // 駒を打つ動きの場合

                // hand-off
                let hand_off = RpmNoteOpe::from_address(Address::from_hand_ph_pt(Some(position.get_phase()), drop));
                rpm_move.push(hand_off);

                // hand-on
                let hand_on = RpmNoteOpe::from_address(destination_address);
                rpm_move.push(hand_on);
            },
            None => {
                // 駒を進める動きの場合
                if let Some(id_piece) = position.get_id_piece_by_address(destination_address.get_index()) {
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
                    let hand_on = RpmNoteOpe::from_address(Address::from_hand_ph_pt(Some(position.get_phase()), up));
                    rpm_move.push(hand_on);
                }

                // board-off
                let board_off = RpmNoteOpe::from_address(Address::from_cell(
                    umove.source.unwrap(),
                    position.get_board_size()));
                rpm_move.push(board_off);

                // board-turn-over
                if umove.promotion {
                    let board_turn = RpmNoteOpe::turn_over();
                    rpm_move.push(board_turn);
                }

                // board-on
                let board_on = RpmNoteOpe::from_address(destination_address);
                rpm_move.push(board_on);
            },
        }

        // change-phase
        let change_phase = RpmNoteOpe::change_phase();
        rpm_move.push(change_phase);

        rpm_move
    }

    /// # Arguments
    /// 
    /// * 'position' - USIレコードと 初期局面を合わせてください。
    /// 
    pub fn play_out_record(
        comm:&Communication,
        position:&mut Position,
        urecord:&UsiRecord,
        rpm_record:&mut RpmRecord) {

        // 局面を動かしながら変換していく。
        // let mut ply = 0;
        for umove in &urecord.moves {
            let rpm_move = UsiConverter::convert_move(*umove, position);
            //comm.println(&format!("Pmoves len: {}.", rpm_move.len()));

            for rpm_note in rpm_move {
                //comm.println(&format!("Pmove: '{}'.", rpm_note.to_sign(position.get_board_size(), &mut ply)));
                RpmPlayer::touch_beautiful_world(comm, rpm_record, &rpm_note, position);
            }
        }
    }
}