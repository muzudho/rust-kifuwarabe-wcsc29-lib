use board_size::*;
use common::caret::*;
///
/// Rpm棋譜のノート。
///
/// 局面から独立しています。
///
use common::closed_interval::ClosedInterval;
use communication::*;
// use kifu_rpm::rpm_tape::*;
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
    /// (closed_interval, note_opt)
    pub fn parse_1note(
        comm: &Communication,
        ope_vec: &Vec<&str>,
        id_vec: &Vec<&str>,
        note_caret: &mut Caret,
        board_size: BoardSize,
    ) -> (ClosedInterval, Option<ShogiNote>) {
        let mut closed_interval = ClosedInterval::new();

        // 数字を返却してから、キャレットを移動。
        let n0 = note_caret.go_next(comm);

        let mut token_caret = Caret::new_facing_right_caret();
        let (sub_closed_interval, note_ope) = if let (sub_closed_interval, Some(note_ope)) =
            ShogiNoteOpe::parse_1ope(&ope_vec[n0 as usize], &mut token_caret, board_size, &comm)
        {
            (sub_closed_interval, note_ope)
        } else {
            panic!("Unexpected operation note token. {}", ope_vec[n0 as usize])
        };

        let pnum: i8 = id_vec[n0 as usize].parse().unwrap();
        let pid_opt = if pnum == -1 {
            // フェーズ・チェンジ。
            None
        } else {
            PieceIdentify::from_number(pnum)
        };

        closed_interval.intersect(n0);
        closed_interval.intersect(sub_closed_interval.get_minimum());
        closed_interval.intersect(sub_closed_interval.get_maximum());

        (
            closed_interval,
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
