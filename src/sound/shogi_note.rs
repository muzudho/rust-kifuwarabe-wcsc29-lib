use instrument::piece_etc::*;
use sound::shogi_note_operation::*;
use std::fmt;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::*;
use studio::common::closed_interval::ClosedInterval;

///
/// Rpm棋譜のノート。
///
/// 局面から独立しています。
///
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
        ope_vec: &[&str],
        id_vec: &[&str],
        note_caret: &mut Caret,
        board_size: BoardSize,
        app: &Application,
    ) -> (ClosedInterval, Option<ShogiNote>) {
        let mut closed_interval = ClosedInterval::new_facing_right();

        // 数字を返却してから、キャレットを移動。
        let n0 = note_caret.seek_a_note(&app);

        let mut token_caret = Caret::new_facing_right_caret();
        let (sub_closed_interval, note_ope) = if let (sub_closed_interval, Some(note_ope)) =
            ShogiNoteOpe::parse_1ope(&ope_vec[n0.index], &mut token_caret, board_size, &app)
        {
            (sub_closed_interval, note_ope)
        } else {
            panic!("Unexpected operation note token. {}", ope_vec[n0.index])
        };

        let pnum: i8 = id_vec[n0.index]
            .parse()
            .unwrap_or_else(|err| panic!(app.comm.println(&format!("{}", err))));
        let pid_opt = if pnum == -1 {
            // フェーズ・チェンジ。
            None
        } else {
            PieceIdentify::from_number(pnum)
        };

        closed_interval.intersect_caret_number(n0.expected_caret_number);
        closed_interval.intersect_caret_number(sub_closed_interval.get_minimum_caret_number());
        closed_interval.intersect_caret_number(sub_closed_interval.get_maximum_caret_number());

        (
            closed_interval,
            Some(ShogiNote::from_id_ope(pid_opt, note_ope)),
        )
    }

    /// For log.
    pub fn to_human_presentable(&self, board_size: BoardSize, app: &Application) -> String {
        format!(
            "'{}'{}",
            match self.identify {
                Some(pid) => pid.to_human_presentable(),
                None => "--".to_string(),
            },
            self.operation.to_human_presentable(board_size, &app)
        )
    }
}
