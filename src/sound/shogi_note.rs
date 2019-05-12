use instrument::piece_etc::*;
use sound::shogi_note_operation::*;
use std::fmt;
use studio::application::Application;
use studio::board_size::*;
//use studio::common::caret::*;
//use studio::common::closed_interval::ClosedInterval;

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
    // 向き。'.' が正順、'L' で逆順。指し手を戻しているときは逆順で、真。
    facing_left: bool,
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
    // ###############
    // # Constructor #
    // ###############
    pub fn from_id_ope(
        pid: Option<PieceIdentify>,
        operation_note: ShogiNoteOpe,
        is_facing_left: bool,
    ) -> Self {
        Self {
            identify: pid,
            operation: operation_note,
            facing_left: is_facing_left,
        }
    }

    // #####
    // # G #
    // #####

    pub fn get_ope(&self) -> ShogiNoteOpe {
        self.operation
    }

    pub fn get_id(&self) -> Option<PieceIdentify> {
        self.identify
    }

    // #####
    // # I #
    // #####

    pub fn is_facing_left(&self) -> bool {
        self.facing_left
    }

    pub fn is_phase_change(&self) -> bool {
        self.operation.is_phase_change()
    }

    // #####
    // # P #
    // #####

    pub fn parse_facing_left(token: &str) -> bool {
        match token {
            "." => false,
            "L" => true,
            _ => panic!("Unexpected facing note token. {}", token),
        }
    }

    /*
    /// 次のノート１つ読取。操作と、駒番号を解析。レコードの終端なら None を返す。
    ///
    /// # Returns
    ///
    /// (closed_interval, note_opt)
    pub fn parse_1note(
        id_vec: &[&str],
        ope_vec: &[&str],
        facing_vec: &[&str],
        note_caret: &mut Caret,
        board_size: BoardSize,
        app: &Application,
    ) -> (ClosedInterval, Option<ShogiNote>) {
        let mut closed_interval = ClosedInterval::new_facing_right();

        // 数字を返却してから、キャレットを移動。
        let n0 = note_caret.seek_a_note(&app);
        let n0index = n0
            .index
            .unwrap_or_else(|| panic!(app.comm.panic("n0 fail.")));

        let mut token_caret = Caret::new_facing_right_caret();

        // 背番号。
        let pnum: i8 = id_vec[n0index]
            .parse()
            .unwrap_or_else(|err| panic!(app.comm.println(&format!("{}", err))));
        let pid_opt = if pnum == -1 {
            // フェーズ・チェンジ。
            None
        } else {
            PieceIdentify::from_number(pnum)
        };

        // 操作。
        let (sub_closed_interval, note_ope) = if let (sub_closed_interval, Some(note_ope)) =
            ShogiNoteOpe::parse_1ope(&ope_vec[n0index], &mut token_caret, board_size, &app)
        {
            (sub_closed_interval, note_ope)
        } else {
            panic!("Unexpected operation note token. {}", ope_vec[n0index])
        };

        // 向き。'.' が正順、'L' で逆順。
        let is_facing_left = ShogiNote::parse_facing_left(facing_vec[n0index]);

        closed_interval.intersect_caret_number(n0.expected_caret);
        closed_interval.intersect_caret_number(sub_closed_interval.get_minimum_caret_number());
        closed_interval.intersect_caret_number(sub_closed_interval.get_maximum_caret_number());

        (
            closed_interval,
            Some(ShogiNote::from_id_ope(pid_opt, note_ope, is_facing_left)),
        )
    }
    */

    // #####
    // # T #
    // #####

    pub fn to_facing_left_str(facing_left: bool) -> String {
        if facing_left { "L" } else { "." }.to_string()
    }

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

    pub fn create_human_presentable_of_vec(
        vec: Vec<ShogiNote>,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        let mut text = String::new();

        for note in vec {
            text = format!("{} {}", text, note.to_human_presentable(board_size, &app));
        }

        text
    }
}
