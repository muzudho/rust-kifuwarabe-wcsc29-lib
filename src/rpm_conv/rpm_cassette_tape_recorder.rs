use board_size::*;
use communication::*;
use human::human_interface::*;
use position::*;
use rpm_conv::rpm_cassette_tape::*;
use rpm_conv::thread::rpm_move::*;
use rpm_conv::thread::rpm_note::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_play::rpm_note_player::*;
use std::*;

pub struct RpmCassetteTapeRecorder {
    /// 何も指していない状態で 1。
    /// TODO 本将棋の大橋流の最初の玉は Ply=-39 にしたい。
    pub ply: i16,
    pub cassette_tape: RpmCassetteTape,
}
impl RpmCassetteTapeRecorder {
    pub fn default() -> Self {
        RpmCassetteTapeRecorder {
            ply: 1,
            cassette_tape: RpmCassetteTape::default(),
        }
    }

    pub fn from_cassette_tape(ply_num: i16, inner_cassette_tape: RpmCassetteTape) -> Self {
        RpmCassetteTapeRecorder {
            ply: ply_num,
            cassette_tape: inner_cassette_tape,
        }
    }

    pub fn clear(&mut self) {
        self.ply = 1;
        self.cassette_tape = RpmCassetteTape::default();
    }

    pub fn reset_caret(&mut self) {
        self.cassette_tape.reset_caret();
    }

    pub fn record_next_note(&mut self, note: RpmNote) {
        if self.cassette_tape.caret >= -1 {
            // 1を足したら根元が0以上の場合、正のテープ。
            // 最後尾かどうか判断。
            if self.cassette_tape.is_positive_peak() {
                // 最後尾に達していれば、追加。
                self.cassette_tape.tape.positive_notes.push(note);
                self.cassette_tape.caret += 1;
            } else {
                // 最後尾でなければ、上書き。
                self.cassette_tape.tape.positive_notes[self.cassette_tape.caret as usize] = note;
                self.cassette_tape.caret += 1;

                // 仮のおわり を更新。
                self.cassette_tape
                    .tape
                    .positive_notes
                    .truncate(self.cassette_tape.caret as usize);
            }
        } else {
            // 負のテープの場合、この処理は失敗。
            panic!("Record next fail in negative tape.");
        }
    }

    pub fn record_back_note(&mut self, note: RpmNote) {
        if self.cassette_tape.caret > 0 {
            // 1を引いても羽先が0以上なら、正のテープ。
            // 正のテープの場合、この処理は失敗。
            panic!("Record back fail in positive tape.");
        }

        // 置換／上書き。新しいテープを作成。
        self.cassette_tape.tape = self
            .cassette_tape
            .tape
            .overwrite_note(self.cassette_tape.caret, note);
        self.cassette_tape.caret -= 1;
    }
    pub fn record_next_move(&mut self, rmove: &RpmMove) {
        for note in rmove.notes.iter() {
            self.record_next_note(*note);
            if let Some(recorded_ply) = note.get_ope().get_phase_change() {
                self.ply = recorded_ply;
            }
        }
    }
    pub fn record_back_move(&mut self, rmove: &RpmMove) {
        for note in rmove.notes.iter() {
            self.record_back_note(*note);
            if let Some(recorded_ply) = note.get_ope().get_phase_change() {
                self.ply = recorded_ply;
            }
        }
    }

    pub fn delete_back(&mut self) -> Option<RpmNote> {
        let (new_tape, removed_note_opt) = self
            .cassette_tape
            .tape
            .delete_back_note(self.cassette_tape.caret);
        self.cassette_tape.tape = new_tape;

        self.cassette_tape.caret -= 1;

        if let Some(removed_note) = removed_note_opt {
            if let Some(recorded_ply) = removed_note.get_ope().get_phase_change() {
                self.ply = recorded_ply;
            }

            Some(removed_note)
        } else {
            None
        }
    }
    pub fn delete_next(&mut self) -> Option<RpmNote> {
        let (new_tape, removed_note_opt) = self
            .cassette_tape
            .tape
            .delete_next_note(self.cassette_tape.caret);
        self.cassette_tape.tape = new_tape;

        self.cassette_tape.caret -= 1;

        if let Some(removed_note) = removed_note_opt {
            if let Some(recorded_ply) = removed_note.get_ope().get_phase_change() {
                self.ply = recorded_ply;
            }

            Some(removed_note)
        } else {
            None
        }
    }

    /// Human presentable large log.
    pub fn to_dump(&self, board_size: BoardSize) -> String {
        self.cassette_tape.to_dump(board_size)
    }

    /// 棋譜読取。
    pub fn read_tape(&mut self, comm: &Communication, line: &str, position: &mut Position) {
        let mut start = 0;

        loop {
            if line.len() <= start {
                return;
            }

            let rnote_ope_opt =
                RpmNoteOpe::parse_1note(&comm, &line, &mut start, position.get_board_size());

            if let Some(rnote_ope) = rnote_ope_opt {
                RpmNotePlayer::touch_brandnew_note(self, &rnote_ope, position, comm);

                let ply = if let Some(ply) = rnote_ope.get_phase_change() {
                    ply
                } else {
                    -1
                };
                HumanInterface::bo(comm, &self.cassette_tape, ply, &position);
            }
        }
    }
}
