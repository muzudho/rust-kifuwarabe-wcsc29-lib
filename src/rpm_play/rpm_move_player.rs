use address::*;
use board_size::*;
use communication::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_conv::rpm_record::*;
use rpm_play::rpm_note_player::*;
use std::*;

pub struct RpmMovePlayer {
}
impl RpmMovePlayer {
    /// 初期化に使う。
    fn init_note(ph:Phase, file:i8, rank:i8, pid:PieceIdentify, bs:BoardSize) -> (RpmNoteOpe, RpmNoteOpe, RpmNoteOpe) {
        (
            RpmNoteOpe::from_address(Address::from_hand_pi(Piece::from_ph_pid(Some(ph), pid))),
            RpmNoteOpe::from_address(Address::from_cell(Cell::from_file_rank(file, rank), bs)),
            RpmNoteOpe::change_phase()
        )
    }

    /// オリジン・ポジションから、平手初期局面に進めます。
    pub fn play_out_to_starting_position(comm:&Communication, rrecord:&mut RpmRecord, pos:&mut Position) {
        //println!("#Position: play_out_to_starting_position().");
        //use piece_etc::IdentifiedPiece;
        use piece_etc::Phase::*;
        use piece_etc::PieceIdentify::*;

        // 大橋流の順序にしてください。
        let bs = pos.get_board_size();
        let array : [(RpmNoteOpe, RpmNoteOpe, RpmNoteOpe);40] = [
            RpmMovePlayer::init_note(Second, 5,1,K00,bs),
            RpmMovePlayer::init_note(First, 5,9,K01,bs),
            RpmMovePlayer::init_note(Second, 4,1,G02,bs),
            RpmMovePlayer::init_note(First, 6,9,G03,bs),
            RpmMovePlayer::init_note(Second, 6,1,G04,bs),
            RpmMovePlayer::init_note(First, 4,9,G05,bs),
            RpmMovePlayer::init_note(Second, 3,1,S06,bs),
            RpmMovePlayer::init_note(First, 7,9,S07,bs),
            RpmMovePlayer::init_note(Second, 7,1,S08,bs),
            RpmMovePlayer::init_note(First, 3,9,S09,bs),
            RpmMovePlayer::init_note(Second, 2,1,N10,bs),
            RpmMovePlayer::init_note(First, 8,9,N11,bs),
            RpmMovePlayer::init_note(Second, 8,1,N12,bs),
            RpmMovePlayer::init_note(First, 2,9,N13,bs),
            RpmMovePlayer::init_note(Second, 1,1,L14,bs),
            RpmMovePlayer::init_note(First, 9,9,L15,bs),
            RpmMovePlayer::init_note(Second, 9,1,L16,bs),
            RpmMovePlayer::init_note(First, 1,9,L17,bs),
            RpmMovePlayer::init_note(Second, 2,2,B18,bs),
            RpmMovePlayer::init_note(First, 8,8,B19,bs),
            RpmMovePlayer::init_note(Second, 8,2,R20,bs),
            RpmMovePlayer::init_note(First, 2,8,R21,bs),
            RpmMovePlayer::init_note(Second, 5,3,P22,bs),
            RpmMovePlayer::init_note(First, 5,7,P23,bs),
            RpmMovePlayer::init_note(Second, 4,3,P24,bs),
            RpmMovePlayer::init_note(First, 6,7,P25,bs),
            RpmMovePlayer::init_note(Second, 6,3,P26,bs),
            RpmMovePlayer::init_note(First, 4,7,P27,bs),
            RpmMovePlayer::init_note(Second, 3,3,P28,bs),
            RpmMovePlayer::init_note(First, 7,7,P29,bs),
            RpmMovePlayer::init_note(Second, 7,3,P30,bs),
            RpmMovePlayer::init_note(First, 3,7,P31,bs),
            RpmMovePlayer::init_note(Second, 2,3,P32,bs),
            RpmMovePlayer::init_note(First, 8,7,P33,bs),
            RpmMovePlayer::init_note(Second, 8,3,P34,bs),
            RpmMovePlayer::init_note(First, 2,7,P35,bs),
            RpmMovePlayer::init_note(Second, 1,3,P36,bs),
            RpmMovePlayer::init_note(First, 9,7,P37,bs),
            RpmMovePlayer::init_note(Second, 9,3,P38,bs),
            RpmMovePlayer::init_note(First, 1,7,P39,bs),
        ];

        for element in array.iter() {
            RpmNotePlayer::touch_brandnew_note(comm, rrecord, &element.0, pos);
            RpmNotePlayer::touch_brandnew_note(comm, rrecord, &element.1, pos);
            RpmNotePlayer::touch_brandnew_note(comm, rrecord, &element.2, pos);
        }
    }

    /// 1手削除する。
    pub fn pop_current_1move_on_record(comm:&Communication, rrecord:&mut RpmRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = RpmNotePlayer::pop_current_1note_on_record(comm, rrecord, position) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    /// 1手進める。
    /// 
    /// # Return
    /// 
    /// 合法タッチか否か。
    pub fn forward_1move_on_record(comm:&Communication, rrecord:&mut RpmRecord, position:&mut Position) -> bool {
        let mut is_first = true;
        let mut forwarding_count = 0;

        // 最後尾に達していたのなら動かさず終了。
        while let (is_legal_touch, Some(rpm_note)) = RpmNotePlayer::forward_1note_on_record(comm, rrecord, position) {
            if !is_legal_touch {
                // TODO 非合法タッチなら、動かした分 戻したい。
                for _i in 0..forwarding_count {
                    RpmNotePlayer::back_1note_on_record(comm, rrecord, position);
                }

                return false;
            }

            if !is_first && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            is_first = false;
            forwarding_count += 1;
        }

        true
    }

    /// 1手戻す。
    pub fn back_1move_on_record(comm:&Communication, rrecord:&mut RpmRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = RpmNotePlayer::back_1note_on_record(comm, rrecord, position) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }
}
