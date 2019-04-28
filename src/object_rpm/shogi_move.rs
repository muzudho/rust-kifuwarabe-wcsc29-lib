///
/// Rpm棋譜のムーブ。
///
/// 局面から独立しています。
///
use address::*;
use board_size::*;
use common::caret::*;
use communication::*;
use kifu_rpm::rpm_tape::*;
use kifu_usi::usi_move::*;
use object_rpm::shogi_note::*;
use piece_etc::*;
use std::fmt;

/// １手分。
//#[derive(Debug)]
pub struct ShogiMove {
    pub notes: Vec<ShogiNote>,

    // 動作確認用。
    pub start: usize,
    pub end: usize,
}
impl fmt::Display for ShogiMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();

        for note in &self.notes {
            text = format!("{} {}", text, note).to_string()
        }

        write!(f, "({}:{}){}", self.start, self.end, text)
    }
}
impl ShogiMove {
    /// 次の1手分解析。
    ///
    /// # Arguments
    ///
    /// (parsed_note_count, move_opt)
    ///
    /// parsed_note_count は巻き戻すのに使う。
    pub fn parse_1move(
        comm: &Communication,
        cassette_tape_j: &RpmTape,
        note_caret: &mut Caret,
        board_size: BoardSize,
    ) -> (usize, Option<ShogiMove>) {
        let mut parsed_note_count = 0;
        let mut notes_buffer = Vec::new();
        let mut first_used_caret = 0;
        let mut last_used_caret = 0;

        let note_size = cassette_tape_j.tracks.ope.len();
        if note_size == 1 {
            panic!(
                "操作トラックが 1ノート ということは無いはず。 {:?}",
                cassette_tape_j.tracks.ope
            )
        }

        //comm.print(&format!("Parse 1move: note_caret: {}, size: {}.", *note_caret, size));
        let mut is_first = true;

        // 次のフェーズ・チェンジまで読み進める。
        'j_loop: loop {
            if note_caret.is_greater_than_or_equal_to(note_size as i16) {
                // トラックの終わり。
                //comm.print("Break: End of track.");
                break 'j_loop;
            }

            //comm.print(&format!("Scanning: note_caret: {}.", note_caret));

            if let (sub_first_used_caret, sub_last_used_caret, Some(note)) =
                ShogiNote::parse_1note(comm, cassette_tape_j, note_caret, board_size)
            {
                parsed_note_count += 1;

                if note.is_phase_change() {
                    if is_first {

                    } else {
                        //comm.print("Break: Phase change.");
                        break 'j_loop;
                    }
                }

                //comm.print(&format!("Push: {:?}.", note));
                notes_buffer.push(note);
                first_used_caret = sub_first_used_caret;
                last_used_caret = sub_last_used_caret;
            } else {
                // パースできるノートが無かった。
                //comm.print("Break: None.");
                break 'j_loop;
            };

            is_first = false;
        }

        if notes_buffer.is_empty() {
            (parsed_note_count, None)
        } else if notes_buffer.len() == 1 {
            panic!(
                "指し手が 1ノート ということは無いはず。 {:?}",
                cassette_tape_j.tracks.ope
            )
        } else {
            (
                parsed_note_count,
                Some(ShogiMove {
                    notes: notes_buffer,
                    start: first_used_caret as usize,
                    end: last_used_caret as usize,
                }),
            )
        }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// この指し手が、どの駒が動いたものによるものなのか、またどこにあった駒なのかを返します。
    ///
    /// # Returns
    ///
    /// (どの駒を動かした一手か, どこの駒を動かした一手か, あれば取った駒，取った駒の番地)
    pub fn to_first_touch_piece_id(
        &self,
        board_size: BoardSize,
    ) -> (
        PieceIdentify,
        Address,
        Option<PieceIdentify>,
        Option<Address>,
    ) {
        // とりあえず USI move に変換するついでに、欲しい情報を得る。
        let (_umove, subject_pid, subject_addr, object_pid_opt, object_address_opt) =
            self.to_usi_move(board_size);

        (
            subject_pid,
            subject_addr,
            object_pid_opt,
            object_address_opt,
        )
    }

    /// 一手。フェーズ・チェンジ・ノートや「ほこり取り」は含まない。
    ///
    /// 決まっている並びをしているものとする。
    ///
    /// # Returns
    ///
    /// (Usi move, どの駒を動かした一手か, どこの駒を動かした一手か, あれば取った駒，取った駒の番地)
    pub fn to_usi_move(
        &self,
        board_size: BoardSize,
    ) -> (
        UsiMove,
        PieceIdentify,
        Address,
        Option<PieceIdentify>,
        Option<Address>,
    ) {
        // 動作の主体。
        let mut subject_pid_opt;
        let mut subject_address_opt;
        let mut subject_promotion = false;
        // 動作に巻き込まれる方。
        let mut object_pid_opt = None;
        let mut object_address_opt = None;
        let mut _object_promotion = false;
        // 基本情報。
        let mut src_opt = None;
        let mut dst_opt = None;
        let mut drop_opt = None;

        let mut i = 0;
        let mut note = if i < self.notes.len() {
            self.notes[i]
        } else {
            panic!(
                "Unexpected 1st note of move(10): 超え。 {}/{}, {}",
                i,
                self.notes.len(),
                self.to_human_presentable(board_size)
            );
        };

        // 盤上の自駒 or 盤上の相手の駒 or 駒台の自駒
        if let Some(address) = note.get_ope().address {
            if let Some(piece) = address.get_hand_piece() {
                // 駒台なら必ず自駒のドロップ。
                subject_pid_opt = note.get_id();
                subject_address_opt = Some(address);
                drop_opt = Some(PieceType::from_piece(piece));

                // 次は置くだけ。
                i += 1;
                if i < self.notes.len() {
                    note = self.notes[i];
                } else {
                    panic!(
                        "Unexpected 1st note of move(20): 超え。 {}/{}, {}",
                        i,
                        self.notes.len(),
                        self.to_human_presentable(board_size)
                    );
                }

                if let Some(address) = note.get_ope().address {
                    dst_opt = Some(board_size.address_to_cell(address.get_index()));
                } else {
                    panic!(
                        "Unexpected 1st note of move(30): {}.",
                        note.to_human_presentable(board_size)
                    );
                }

            // その次は無い。

            // 盤上
            } else {
                // これが盤上の自駒か、相手の駒かは、まだ分からない。仮に入れておく。
                subject_pid_opt = note.get_id();
                subject_address_opt = Some(address);
                src_opt = Some(board_size.address_to_cell(address.get_index()));

                // 次。
                i += 1;
                if i < self.notes.len() {
                    note = self.notes[i];
                } else {
                    panic!(
                        "Unexpected 1st note of move(40): 超え。 {}/{}, {}",
                        i,
                        self.notes.len(),
                        self.to_human_presentable(board_size)
                    );
                }

                if note.get_ope().sky_turn {
                    // +。駒を裏返した。自駒を成ったのか、取った成り駒を表返したのかは、まだ分からない。仮に成ったことにしておく。
                    subject_promotion = true;

                    // 次。
                    i += 1;
                    if i < self.notes.len() {
                        note = self.notes[i];
                    } else {
                        panic!(
                            "Unexpected 1st note of move(50): 超え。 {}/{}, {}",
                            i,
                            self.notes.len(),
                            self.to_human_presentable(board_size)
                        );
                    }
                }

                if note.get_ope().sky_rotate {
                    // -。向きを変えているようなら、相手の駒を取ったようだ。いろいろキャンセルする。
                    object_pid_opt = subject_pid_opt;
                    object_address_opt = subject_address_opt;
                    _object_promotion = subject_promotion;
                    subject_pid_opt = None;
                    subject_address_opt = None;
                    src_opt = None;
                    subject_promotion = false;

                    // 次。
                    i += 1;
                    if i < self.notes.len() {
                        note = self.notes[i];
                    } else {
                        panic!(
                            "Unexpected 1st note of move(55): 超え。 {}/{}, {}",
                            i,
                            self.notes.len(),
                            self.to_human_presentable(board_size)
                        );
                    }

                    // 自分の駒台に置く動き。
                    if let Some(_address) = note.get_ope().address {
                        // 次は、盤上の自駒を触る。
                        i += 1;
                        if i < self.notes.len() {
                            note = self.notes[i];
                        } else {
                            panic!(
                                "Unexpected 1st note of move(60): 超え。 {}/{}, {}",
                                i,
                                self.notes.len(),
                                self.to_human_presentable(board_size)
                            );
                        }

                        if let Some(address) = note.get_ope().address {
                            subject_pid_opt = note.get_id();
                            subject_address_opt = Some(address);
                            src_opt = Some(board_size.address_to_cell(address.get_index()));
                            // 次。
                            i += 1;
                            if i < self.notes.len() {
                                note = self.notes[i];
                            } else {
                                panic!(
                                    "Unexpected 1st note of move(80): 超え。 {}/{}, {}",
                                    i,
                                    self.notes.len(),
                                    self.to_human_presentable(board_size)
                                );
                            }
                        }
                    } else {
                        panic!(
                            "Unexpected 1st note of move(70): {}.",
                            note.to_human_presentable(board_size)
                        );
                    }
                } else {
                    // 盤上の自駒を触ったのだと確定した。
                }

                if note.get_ope().sky_turn {
                    // +。盤上の自駒が成った。
                    subject_promotion = true;

                    // 次。
                    i += 1;
                    if i < self.notes.len() {
                        note = self.notes[i];
                    } else {
                        panic!(
                            "Unexpected 1st note of move(90): 超え。 {}/{}, {}",
                            i,
                            self.notes.len(),
                            self.to_human_presentable(board_size)
                        );
                    }
                }

                if let Some(address) = note.get_ope().address {
                    // 行き先に盤上の自駒を進めた。
                    dst_opt = Some(board_size.address_to_cell(address.get_index()));

                    // これで終わり。
                }
            }
        } else {
            panic!(
                "Unexpected 1st note of move(100): {}.",
                note.to_human_presentable(board_size)
            );
        }

        if i + 1 < self.notes.len() {
            panic!(
                "Unexpected 1st note of move(110): 余り。 {}/{}, {}",
                i,
                self.notes.len(),
                self.to_human_presentable(board_size)
            );
        }

        let umove = if let Some(drop) = drop_opt {
            UsiMove::create_drop(dst_opt.unwrap(), drop, board_size)
        } else if let Some(dst) = dst_opt {
            UsiMove::create_walk(src_opt.unwrap(), dst, subject_promotion, board_size)
        } else {
            panic!(
                "Unexpected dst. move.len: '{}' > 1, move: '{}'.",
                self.len(),
                self
            )
        };

        // USIの指し手が作れれば、 動作の主体 が分からないことはないはず。
        if let Some(subject_idp) = subject_pid_opt {
            (
                umove,
                subject_idp,
                subject_address_opt.unwrap(),
                object_pid_opt,
                object_address_opt,
            )
        } else {
            panic!("Unexpected rpm move. id fail.")
        }
    }

    pub fn to_operation_string(&self, board_size: BoardSize) -> String {
        let mut text = String::new();

        for i in 0..self.len() {
            text = format!("{} {}", text, &self.notes[i].get_ope().to_sign(board_size));
        }

        text
    }

    pub fn to_identify_string(&self) -> String {
        let mut text = String::new();

        for i in 0..self.len() {
            text = format!(
                "{} {}",
                text,
                match &self.notes[i].get_id() {
                    Some(pid) => pid.get_number().to_string(),
                    None => "-1".to_string(),
                }
            );
        }

        text
    }

    /// Human presentable.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        let mut text = String::new();

        for note in &self.notes {
            text = format!("{} {}", text, note.to_human_presentable(board_size))
        }

        format!("({}:{}){}", self.start, self.end, text)
    }
}
