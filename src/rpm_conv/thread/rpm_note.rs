///
/// Rpm棋譜のノート。
///
/// 局面から独立しています。
///
use board_size::*;
use common::caret::*;
use communication::*;
use piece_etc::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_for_json::rpm_book_file::*;
use std::fmt;

//#[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub struct RpmNote {
    // 駒の背番号。フェーズ・チェンジのときは None。
    identify: Option<PieceIdentify>,
    operation: RpmNoteOpe,
}
impl fmt::Display for RpmNote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "'{}'{}",
            match self.identify {
                Some(pid) => pid.to_human_presentable(),
                None => "--".to_string(),
            },
            self.operation
        )
    }
}
impl RpmNote {
    /// Human presentable.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        format!(
            "'{}'{}",
            match self.identify {
                Some(pid) => pid.to_human_presentable(),
                None => "--".to_string(),
            },
            self.operation.to_human_presentable(board_size)
        )
    }

    pub fn from_id_ope(pid: Option<PieceIdentify>, operation_note: RpmNoteOpe) -> RpmNote {
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

    /// 次のノート１つ読取。操作と、駒番号を解析。レコードの終端なら None を返す。
    pub fn parse_next_1note(
        comm: &Communication,
        record_for_json: &RpmRecordForJson,
        caret: &mut usize,
        board_size: BoardSize,
    ) -> Option<RpmNote> {
        let size = record_for_json.body.operation.len();

        if size <= *caret {
            // 範囲外はエラーで落とす。
            panic!(
                "Out of bounds exception: size: {}, caret: {}.",
                size, *caret
            );
        }

        let mut token_caret = Caret::new();
        let note_ope = if let Some(note_ope) = RpmNoteOpe::parse_next_1note(
            &comm,
            &record_for_json.body.operation[*caret],
            &mut token_caret,
            board_size,
        ) {
            note_ope
        } else {
            panic!(
                "Unexpected operation note token. {}",
                record_for_json.body.operation[*caret]
            )
        };

        let pnum = record_for_json.body.piece_number[*caret];
        let pid_opt = if pnum == -1 {
            // フェーズ・チェンジ。
            None
        } else {
            PieceIdentify::from_number(pnum)
        };

        // カウントアップ。
        *caret += 1;
        Some(RpmNote::from_id_ope(pid_opt, note_ope))
    }
}
