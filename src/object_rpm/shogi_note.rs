///
/// Rpm棋譜のノート。
///
/// 局面から独立しています。
///
use board_size::*;
use common::caret::*;
use communication::*;
use kifu_rpm::rpm_tape::*;
use object_rpm::shogi_note_operation::*;
use piece_etc::*;
use std::fmt;

//#[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub struct ShogiNote {
    // 駒の背番号。フェーズ・チェンジのときは None。
    identify: Option<PieceIdentify>,
    operation: ShogiNoteOpe,
}
impl fmt::Display for ShogiNote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}'{}'",
            match self.identify {
                Some(pid) => pid.to_human_presentable(),
                None => "--".to_string(),
            },
            self.operation
        )
    }
}
impl ShogiNote {
    pub fn from_id_ope(pid: Option<PieceIdentify>, operation_note: ShogiNoteOpe) -> ShogiNote {
        ShogiNote {
            identify: pid,
            operation: operation_note,
        }
    }

    pub fn get_ope(&self) -> ShogiNoteOpe {
        self.operation
    }

    pub fn get_id(&self) -> Option<PieceIdentify> {
        self.identify
    }

    pub fn is_phase_change(&self) -> bool {
        self.operation.is_phase_change()
    }

    /// 次のノート１つ読取。操作と、駒番号を解析。レコードの終端なら None を返す。
    ///
    /// # Returns
    ///
    /// (first_used_caret, last_used_caret, note_opt)
    pub fn parse_1note(
        comm: &Communication,
        cassette_tape_j: &RpmTape,
        note_caret: &mut Caret,
        board_size: BoardSize,
    ) -> (i16, i16, Option<ShogiNote>) {
        let size = cassette_tape_j.tracks.ope.len();

        if note_caret.is_greater_than_or_equal_to(size as i16) {
            // 範囲外はエラーで落とす。
            panic!(
                "Out of bounds exception: size: {}, caret: {}.",
                size,
                note_caret.to_human_presentable()
            );
        }

        // カウントアップ。
        let first_used_caret = note_caret.go_next(comm);

        // TODO 毎回スプリットするのはもったいない☆（＾～＾）
        let ope_vec: Vec<&str> = cassette_tape_j.tracks.ope.split(' ').collect();

        let mut token_caret = Caret::new_facing_right_caret();
        let (last_used_caret, note_ope) = if let (sub_last_used_caret, Some(note_ope)) =
            ShogiNoteOpe::parse_1ope(
                &ope_vec[first_used_caret as usize],
                &mut token_caret,
                board_size,
                &comm,
            ) {
            (sub_last_used_caret, note_ope)
        } else {
            panic!(
                "Unexpected operation note token. {}",
                ope_vec[first_used_caret as usize]
            )
        };

        // TODO 毎回スプリットするのはもったいない☆（＾～＾）
        let id_vec: Vec<&str> = cassette_tape_j.tracks.id.split(' ').collect();

        let pnum: i8 = id_vec[first_used_caret as usize].parse().unwrap();
        let pid_opt = if pnum == -1 {
            // フェーズ・チェンジ。
            None
        } else {
            PieceIdentify::from_number(pnum)
        };

        (
            first_used_caret,
            last_used_caret,
            Some(ShogiNote::from_id_ope(pid_opt, note_ope)),
        )
    }

    /// For log.
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
}