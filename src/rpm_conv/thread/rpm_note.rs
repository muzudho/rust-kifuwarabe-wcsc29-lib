///
/// Rpm棋譜のノート。
/// 
/// 局面から独立しています。
/// 
use board_size::*;
use communication::*;
use piece_etc::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_for_json::rpm_book_file::*;
use std::fmt;

//#[derive(Debug)]
pub struct RpmNote {
    // 駒の背番号。フェーズ・チェンジのときは None。
    identify: Option<PieceIdentify>,
    operation: RpmNoteOpe,
}
impl fmt::Display for RpmNote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'{}",
            match self.identify {
                Some(pid) => {
                    pid.to_human_presentable()
                },
                None => { "--".to_string() },
            },
            self.operation
        )
    }
}
impl RpmNote {
    pub fn create_rpm_note(operation_note: RpmNoteOpe, pid:Option<PieceIdentify>) -> RpmNote {
        RpmNote {
            identify: pid,
            operation: operation_note,
        }
    }

    pub fn get_ope(&self) -> RpmNoteOpe {
        self.operation
    }

    pub fn get_id(&self) -> Option<PieceIdentify> {
        self.identify
    }

    pub fn is_phase_change(&self) -> bool {
        self.operation.is_phase_change()
    }

    /// 操作と、駒番号を解析。レコードの終端なら None を返す。
    pub fn parse_1note(comm:&Communication, record_for_json:&RpmRecordForJson, note_start:&mut usize, board_size:BoardSize) -> Option<RpmNote> {
        let size = record_for_json.body.operation.len();

        if size <= *note_start {
            // 範囲外はエラーで落とす。
            panic!("Out of bounds exception: size: {}, note_start: {}.", size, *note_start);
        }

        let mut token_start = 0;
        let note_ope = if let Some(note_ope) = RpmNoteOpe::parse_1note(
            &comm,
            &record_for_json.body.operation[*note_start],
            &mut token_start,
            board_size) {
            note_ope
        } else {
            panic!("Unexpected operation note token. {}", record_for_json.body.operation[*note_start])
        };

        let pnum = record_for_json.body.piece_number[*note_start];
        let pid_opt = if pnum == -1 {
            // フェーズ・チェンジ。
            None
        } else {
            PieceIdentify::from_number(pnum)
        };

        // カウントアップ。
        *note_start += 1;
        Some(RpmNote::create_rpm_note(note_ope, pid_opt))
    }
}
