use address::*;
use application::Application;
use communication::*;
use kifu_kif::kif_move::*;
use kifu_kif::kif_tape::*;
use object_rpm::cassette_deck::*;
use object_rpm::shogi_note_operation::*;
use piece_etc::*;
use shogi_ban::game_player::*;
use shogi_ban::position::*;

pub struct KifConverter {}
impl KifConverter {
    pub fn convert_kif_tape_fragment(input_path: &str, deck: &mut CassetteDeck, app: &Application) {
        // comm.println(&format!("input_path: {}", input_path));

        // Model.
        let mut position = Position::default();
        let ktape = KifTape::load(&input_path);

        // Play.
        KifConverter::play_out_kifu_tape(&mut position, &ktape, deck, &app);
        // HumanInterface::bo(&comm, &rrecord.body.operation_track, &position);

        // Save. (Append)
        /*
        let output_tapefrag_path =
            CassetteDeck::create_file_full_path(".tapefrag", &app.kw29_conf);
            */
        deck.write_tape_fragment(position.get_board_size(), &app);
        /*
        // Save. (Append)
        deck.write_cassette_tape_box(position.get_board_size(), &app);
        */

        // comm.println("Finished.");
    }

    /// 変換には、初期局面が必要。
    fn play_out_kifu_tape(
        position: &mut Position,
        ktape: &KifTape,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        // TODO とりあえず平手初期局面だけ対応。
        position.reset_origin_position();
        GamePlayer::play_ohashi_starting(position, deck, &app);

        let mut ply = 1;
        for kmove in &ktape.items {
            let rnote_opes = KifConverter::convert_move(&app.comm, kmove, position, ply);

            for rnote_ope in rnote_opes {
                GamePlayer::touch_1note_ope(&rnote_ope, position, deck, &app);
            }

            ply += 1;
        }
    }

    /// 変換には、現局面が必要。
    pub fn convert_move(
        _comm: &Communication,
        kmove: &KifMove,
        position: &Position,
        ply: i16,
    ) -> Vec<ShogiNoteOpe> {
        let mut rmoves = Vec::new();

        let destination_address =
            Address::from_cell(kmove.destination.unwrap(), position.get_board_size());

        if kmove.is_drop {
            // 駒を打つ動きの場合
            let piece_type = jsa_piece_type_to_perfect(kmove.piece);
            let piece = Piece::from_ph_pt(Some(position.get_phase()), piece_type.unwrap());
            let drop = position.peek_hand(piece);

            // hand-off
            let hand_off = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                Some(position.get_phase()),
                drop.unwrap().get_type(),
            ));
            rmoves.push(hand_off);

            // hand-on
            let hand_on = ShogiNoteOpe::from_address(destination_address);
            rmoves.push(hand_on);
        } else {
            // 駒を進める動きの場合
            if let Some(capture_id_piece) =
                position.get_id_piece_by_address(destination_address.get_index())
            {
                // 駒を取る動きが入る場合

                // hand-off
                let hand_off = ShogiNoteOpe::from_address(destination_address);
                rmoves.push(hand_off);

                // hand-turn
                if capture_id_piece.is_promoted() {
                    let hand_turn = ShogiNoteOpe::turn_over();
                    rmoves.push(hand_turn);
                }

                // hand-rotate
                let hand_rotate = ShogiNoteOpe::rotate();
                rmoves.push(hand_rotate);

                // hand-on
                let up = capture_id_piece.get_type();
                let hand_on = ShogiNoteOpe::from_address(Address::from_hand_ph_pt(
                    Some(position.get_phase()),
                    up,
                ));
                rmoves.push(hand_on);
            }

            // board-off
            let board_off = ShogiNoteOpe::from_address(Address::from_cell(
                kmove.source.unwrap(),
                position.get_board_size(),
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
                let board_turn = ShogiNoteOpe::turn_over();
                rmoves.push(board_turn);
            }

            // board-on
            let board_on = ShogiNoteOpe::from_address(destination_address);
            rmoves.push(board_on);
        };

        // change-phase
        let change_phase = ShogiNoteOpe::change_phase(ply);
        rmoves.push(change_phase);

        rmoves
    }
}
