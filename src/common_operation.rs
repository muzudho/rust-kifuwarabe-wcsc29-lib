use communication::*;
use parser::*;
use position::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_conv::rpm_record::*;
use std::*;
use usi_conv::usi_record::*;

pub struct CommonOperation {
}
impl CommonOperation {
    /// 盤に触れて、棋譜も書くぜ☆（＾～＾）
    pub fn touch_beautiful_world(comm:&Communication, rpm_record:&mut RpmRecord, rpm_note:&RpmNoteOpe, position:&mut Position) {
        let piece_id_number = if let (_is_legal_touch, Some(piece_identify)) = position.touch_world(comm, &rpm_note) {
            piece_identify.get_id().get_number()
        } else {
            -1
        };
        rpm_record.add_note(&rpm_note, piece_id_number);
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_current_1mark(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        let mut cursor_clone = rpm_record.body.cursor; // .clone();
        if let Some(rpm_note) = rpm_record.body.operation_track.pop_current(&mut rpm_record.body.cursor, &mut rpm_record.body.ply) {
            rpm_record.body.identify_track.pop_current(&mut cursor_clone);

            let (_is_legal_touch, _piece_identify_opt) = position.touch_world(comm, &rpm_note);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手削除する。
    pub fn pop_current_1ply(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = CommonOperation::pop_current_1mark(comm, rpm_record, position) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    /// 棋譜のカーソルが指している要素をもう１回タッチし、カーソルは１つ戻す。
    pub fn back_1note(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) -> Option<RpmNoteOpe> {
        if let Some(rpm_note) = rpm_record.body.operation_track.get_current(rpm_record.body.cursor) {
            let (_is_legal_touch, _piece_identify_opt) = position.touch_world(comm, &rpm_note);
            rpm_record.back();
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手戻す。
    pub fn back_1ply(comm:&Communication, rpm_record:&mut RpmRecord, position:&mut Position) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = CommonOperation::back_1note(comm, rpm_record, position) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }

    pub fn read_usi_moves(comm:&Communication, line:&str, start:&mut usize, position:&mut Position) -> Option<UsiRecord> {
        if Parser::match_keyword(&comm, &line, "moves", start) || 
            Parser::match_keyword(&comm, &line, " moves", start) {
        } else {
            // comm.println(&format!("#Moves not matched. line: '{}', start: {}.", line, start));
            return None;
        }

        Parser::skip_spaces(&comm, &line, start);

        let mut logical_record = UsiRecord::new();

        // `position startpos moves `. [0]p, [1]o, ...

        // Examples.
        // position startpos moves 2g2f 8c8d
        let mut temp_u_record = UsiRecord::new();
        temp_u_record.parse_usi_some_moves(&comm, line, start, position.get_board_size());
        // comm.println(&format!("#temp_record.items.len: {}", temp_u_record.items.len()));

        // TODO 指し手通り、進めたい。
        for mov in &temp_u_record.items {
            // comm.println(&format!("#Move: `{}`.", mov.to_sign()));
            logical_record.make_move(*mov, position);
            //comm.println(&position.to_text(comm, logical_record.get_current_phase()));
        }

        Some(logical_record)
    }
}