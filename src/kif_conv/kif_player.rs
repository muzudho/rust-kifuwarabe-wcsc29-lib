use address::*;
use communication::*;
use common_operation::*;
use kif_conv::kif_move::*;
use kif_conv::kif_record::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_conv::rpm_record::*;

pub struct KifPlayer {

}
impl KifPlayer {
    /// 変換には、現局面が必要。
    pub fn convert_move(
        _comm:&Communication,
        kmove:&KifMove,
        position:&Position,
        _ply:i16) -> Vec<RpmNoteOpe> {
        let mut rmoves = Vec::new();

        let destination_address = Address::from_cell(
            kmove.destination.unwrap(),
            position.get_board_size()
        );
        
        if kmove.is_drop {
            // 駒を打つ動きの場合
            let piece_type = jsa_piece_type_to_perfect(kmove.piece);
            let piece = Piece::from_ph_pt(Some(position.get_phase()), piece_type.unwrap());
            let drop = position.peek_hand(piece);

            // hand-off
            let hand_off = RpmNoteOpe::from_address(Address::from_hand(Some(position.get_phase()), drop.unwrap().get_type()));
            rmoves.push(hand_off);

            // hand-on
            let hand_on = RpmNoteOpe::from_address(destination_address);
            rmoves.push(hand_on);
        } else {
            // 駒を進める動きの場合
            if let Some(capture_id_piece) = position.get_id_piece_by_address(destination_address.get_index()) {
                // 駒を取る動きが入る場合

                // hand-off
                let hand_off = RpmNoteOpe::from_address(destination_address);
                rmoves.push(hand_off);

                // hand-turn
                if capture_id_piece.is_promoted() {
                    let hand_turn = RpmNoteOpe::turn_over();
                    rmoves.push(hand_turn);
                }

                // hand-rotate
                let hand_rotate = RpmNoteOpe::rotate();
                rmoves.push(hand_rotate);

                // hand-on
                let up = capture_id_piece.get_type();
                let hand_on = RpmNoteOpe::from_address(Address::from_hand(Some(position.get_phase()), up));
                rmoves.push(hand_on);
            }

            // board-off
            let board_off = RpmNoteOpe::from_address(Address::from_cell(
                kmove.source.unwrap(),
                position.get_board_size()
            ));
            rmoves.push(board_off);

            // board-turn-over
            // 盤上にある駒が不成で、指し手の駒種類が成り駒なら、今、成った。
            if let Some(id_piece) = position.get_id_piece(kmove.source.unwrap()) {
                id_piece.is_promoted()
            } else {
                false
            };
            
            if kmove.is_promote {
                let board_turn = RpmNoteOpe::turn_over();
                rmoves.push(board_turn);
            }

            // board-on
            let board_on = RpmNoteOpe::from_address(destination_address);
            rmoves.push(board_on);
        };

        // change-phase
        let change_phase = RpmNoteOpe::change_phase();
        rmoves.push(change_phase);

        rmoves
    }

    /// 変換には、初期局面が必要。
    pub fn play_out_record(
        comm:&Communication,
        position:&mut Position,
        krecord:&KifRecord,
        rrecord:&mut RpmRecord) {

        // TODO とりあえず平手初期局面だけ対応。
        position.reset_startpos();

        let mut ply = 1;
        for kmove in &krecord.items {
            let rmoves = KifPlayer::convert_move(
                comm,
                kmove,
                position,
                ply);

            for rnote in rmoves {
                CommonOperation::touch_beautiful_world(comm, rrecord, &rnote, position);
            }

            ply += 1;
        }
    }
}