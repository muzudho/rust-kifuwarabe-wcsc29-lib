use application::Application;
use board_size::BoardSize;
use human::human_interface::*;
use object_rpm::cassette_tape::CassetteTape;
use object_rpm::cassette_tape_box::*;
use object_rpm::shogi_move::ShogiMove;
use object_rpm::shogi_move::*;
use object_rpm::shogi_note::ShogiNote;
use object_rpm::shogi_note::*;
use position::*;

/// カセット・デッキ。
pub struct CassetteDeck {
    /// 何も指していない状態で 1。
    /// TODO 本将棋の大橋流の最初の玉は Ply=-39 にしたい。
    /// トレーニング・テープの 手目。
    pub t_tape_ply: i16,

    /// トレーニング・テープ。最初は空っぽ。
    t_tape: Option<CassetteTape>,

    /// ラーニング・テープの 手目。
    pub l_tape_ply: i16,

    /// ラーニング・テープ。
    l_tape: Option<CassetteTape>,

    /// ラーニング・テープ書き込み用のテープ・ボックス。
    l_box: Option<CassetteTapeBox>,
}
impl CassetteDeck {
    pub fn new_empty(app: &Application) -> Self {
        CassetteDeck {
            t_tape_ply: 1,
            t_tape: None,
            l_tape_ply: 1,
            l_box: Some(CassetteTapeBox::new_at_random(&app)),
            l_tape: Some(CassetteTape::new_facing_right_at_random(&app)),
        }
    }

    pub fn change(&mut self, t_tape_opt: Option<CassetteTape>) {
        self.t_tape = t_tape_opt;
        self.t_tape_ply = 1;

        // 今までのラーニング・テープは、外部ファイルに保存。
        {
            if let Some(box_for_write) = &self.l_box {
                box_for_write.write_cassette_tape_box(
                    board_size,
                    &self.get_mut_recording_tape(&app),
                    &app.comm,
                )
            } else {
                panic!("Get tape box fail.")
            }
            self.choice_box_automatically(&app);
        }

        // 新しいラーニング・テープに差し替える。
        {
            // TODO 本当は満杯になるまで使い回したい☆（＾～＾）
            self.l_box = Some(CassetteTapeBox::new_at_random(&app));
        }
    }

    pub fn clear_tape_editor(&mut self, app: &Application) {
        self.recording_tape_ply = 1;
        self.get_mut_recording_tape(&app).clear();
    }

    pub fn reset_caret(&mut self, app: &Application) {
        self.get_mut_recording_tape(&app).reset_caret();
    }

    /// 記録用テープがなければ、新しいテープを入れておきます。
    pub fn supply_automatic(&mut self, app: &Application) {
        if let Some(ref _tape) = self.recording_tape {
        } else {
            self.recording_tape = Some(CassetteTape::new_facing_right_at_random(&app));
        }
    }

    pub fn get_mut_recording_tape(&mut self, app: &Application) -> &mut CassetteTape {
        self.supply_automatic(&app);
        if let Some(ref mut tape) = self.recording_tape {
            &mut tape
        } else {
            panic!("Recording tape none.");
        }
    }

    /// テープボックスを指定するぜ☆（＾～＾）
    pub fn choice_box_manually(&mut self, file: &str) {
        self.current_box_for_write = Some(CassetteTapeBox::new_with_file(file));
    }

    /// テープ・フラグメント単位で書き込めるぜ☆（*＾～＾*）
    pub fn write_cassette_tape_fragment(&mut self, board_size: BoardSize, app: &Application) {
        self.get_mut_recording_tape(&app)
            .write_cassette_tape_fragment(board_size, &app.comm);
    }

    /// テープ・ボックス単位で書き込めるぜ☆（*＾～＾*）
    pub fn write_cassette_tape_box(&mut self, board_size: BoardSize, app: &Application) {}

    /// 記録用テープの文字化。
    pub fn to_mut_recording_tape_sign(
        &mut self,
        board_size: BoardSize,
        app: &Application,
    ) -> (String, String) {
        self.get_mut_recording_tape(&app).to_sign(board_size)
    }

    pub fn clear_recording_tape(&mut self, app: &Application) {
        self.get_mut_recording_tape(&app).clear();
    }

    pub fn put_1move(&mut self, rmove: &ShogiMove, app: &Application) {
        for note in rmove.notes.iter() {
            self.put_1note(*note, app);
            if let Some(recorded_ply) = note.get_ope().get_phase_change() {
                self.recording_tape_ply = recorded_ply;
            }
        }
    }

    /// キャレット位置に、ノートを上書き、または追加をするぜ☆（＾～＾）
    pub fn put_1note(&mut self, note: ShogiNote, app: &Application) {
        if let Some(tape) = self.recording_tape {
            let (is_positive, index) = tape.caret.to_index();

            if is_positive {
                // 正のテープ。
                // 最先端かどうか判断。
                if tape.is_positive_peak() && !tape.caret.is_facing_left() {
                    // 正の絶対値が大きい方の新しい要素を追加しようとしている。
                    // Push note to positive number side of recording tape.
                    self.get_mut_recording_tape(&app)
                        .tracks
                        .positive_notes
                        .push(note);
                    self.get_mut_recording_tape(&app)
                        .caret
                        .go_next(&app.comm, "r_n+new");
                } else {
                    // 先端でなければ、上書き。
                    self.get_mut_recording_tape(&app).tracks.positive_notes[index] = note;
                    self.get_mut_recording_tape(&app)
                        .caret
                        .go_next(&app.comm, "r_n+exists");

                    // 仮のおわり を更新。
                    let (_is_positive, index) = tape.caret.to_index();
                    self.get_mut_recording_tape(&app)
                        .tracks
                        .positive_notes
                        .truncate(index);
                }
            } else {
                // 負のテープ。
                // 最先端かどうか判断。
                if tape.is_negative_peak() && tape.caret.is_facing_left() {
                    // 負の絶対値が大きい方の新しい要素を追加しようとしている。
                    self.get_mut_recording_tape(&app)
                        .tracks
                        .negative_notes
                        .push(note);
                    self.get_mut_recording_tape(&app)
                        .caret
                        .go_next(&app.comm, "r_n-new");
                } else {
                    // 先端でなければ、上書き。
                    self.get_mut_recording_tape(&app).tracks.negative_notes[index] = note;
                    self.get_mut_recording_tape(&app)
                        .caret
                        .go_next(&app.comm, "r_n-exists");

                    // 仮のおわり を更新。
                    let (_is_positive, index) = self.get_mut_recording_tape(&app).caret.to_index();
                    self.get_mut_recording_tape(&app)
                        .tracks
                        .negative_notes
                        .truncate(index);
                }
            }
        } else {
            panic!("Recording tape is none.")
        };
    }

    pub fn delete_1note(&mut self, app: &Application) -> Option<ShogiNote> {
        let recording_cassette_tape = self.get_mut_recording_tape(app);
        let caret = &recording_cassette_tape.caret;

        let (new_tape, removed_note_opt) = recording_cassette_tape.tracks.new_truncated_tape(caret);
        recording_cassette_tape.tracks = new_tape;

        if let Some(removed_note) = removed_note_opt {
            if let Some(recorded_ply) = removed_note.get_ope().get_phase_change() {
                self.recording_tape_ply = recorded_ply;
            }

            Some(removed_note)
        } else {
            None
        }
    }

    /// 棋譜のカーソルが指している要素を削除して、１つ戻る。
    pub fn pop_1note(&mut self, position: &mut Position, app: &Application) -> Option<ShogiNote> {
        HumanInterface::show_position(&app.comm, -1, position);

        if let Some(rpm_note) = self.delete_1note(app) {
            let board_size = position.get_board_size();
            let (_is_legal_touch, _piece_identify_opt) =
                position.touch_beautiful_1note(&rpm_note.get_ope(), &app.comm, board_size);
            Some(rpm_note)
        } else {
            None
        }
    }

    /// 1手削除する。
    pub fn pop_1move(&mut self, position: &mut Position, app: &Application) {
        let mut count = 0;
        // 開始前に達したら終了。
        while let Some(rpm_note) = self.pop_1note(position, app) {
            if count != 0 && rpm_note.is_phase_change() {
                // フェーズ切り替えしたら終了。（ただし、初回除く）
                break;
            }

            // それ以外は繰り返す。
            count += 1;
        }
    }
}
