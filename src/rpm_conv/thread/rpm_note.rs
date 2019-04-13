use communication::*;
use position::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_model::rpm_book_file::*;

#[derive(Debug)]
pub struct RpmNote {
    operation: RpmNoteOpe,
    // 駒の背番号。
    identify: i8,
}
impl RpmNote {
    pub fn create_rpm_note(operation_note: RpmNoteOpe, identify_num: i8) -> RpmNote {
        RpmNote {
            operation: operation_note,
            identify: identify_num,
        }
    }

    pub fn get_ope(&self) -> RpmNoteOpe {
        self.operation
    }

    pub fn get_id(&self) -> i8 {
        self.identify
    }

    pub fn is_phase_change(&self) -> bool {
        self.operation.is_phase_change()
    }

    /// 操作と、駒番号を解析。レコードの終端なら None を返す。
    pub fn parse_1note(comm:&Communication, record_for_json:&RpmRecordForJson, note_start:usize, board_size:BoardSize) -> Option<RpmNote> {
        let size = record_for_json.body.operation.len();

        if size <= note_start {
            return None;
        }

        let mut token_start = 0;
        let note_ope = if let Some(note_ope) = RpmNoteOpe::parse_1note(
            &comm,
            &record_for_json.body.operation[note_start],
            &mut token_start,
            board_size) {
            note_ope
        } else {
            panic!("Unexpected operation note token. {}", record_for_json.body.operation[note_start])
        };

        let piece_num = record_for_json.body.piece_number[note_start];

        Some(RpmNote::create_rpm_note(note_ope, piece_num))
    }
}
