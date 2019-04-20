///
/// Rpm棋譜のムーブ。
///
/// 局面から独立しています。
///
use address::*;
use board_size::*;
use communication::*;
use piece_etc::*;
use rpm_conv::thread::rpm_note::*;
use rpm_for_json::rpm_book_file::*;
use std::fmt;
use usi_conv::usi_move::*;

/// １手分。
//#[derive(Debug)]
pub struct RpmMove {
    pub notes: Vec<RpmNote>,

    // 動作確認用。
    pub start: usize,
    pub end: usize,
}
impl fmt::Display for RpmMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();

        for note in &self.notes {
            text = format!("{} {}", text, note).to_string()
        }

        write!(f, "({}:{}){}", self.start, self.end, text)
    }
}
impl RpmMove {
    /// Human presentable.
    pub fn to_log(&self, board_size: BoardSize) -> String {
        let mut text = String::new();

        for note in &self.notes {
            text = format!("{} {}", text, note.to_log(board_size))
        }

        format!("({}:{}){}", self.start, self.end, text)
    }

    /// 1手分解析。
    pub fn parse_1move(
        comm: &Communication,
        record_for_json: &RpmRecordForJson,
        note_start: &mut usize,
        board_size: BoardSize,
    ) -> Option<RpmMove> {
        let mut rmove = RpmMove {
            notes: Vec::new(),
            start: *note_start,
            end: *note_start,
        };

        let size = record_for_json.body.operation.len();
        if size == 1 {
            panic!(
                "操作トラックが 1ノート ということは無いはず。 {:?}",
                record_for_json.body.operation
            )
        }

        //comm.print(&format!("Parse 1move: note_start: {}, size: {}.", *note_start, size));
        let mut is_first = true;

        // 次のフェーズ・チェンジまで読み進める。
        'j_loop: loop {
            if size <= *note_start {
                // トラックの終わり。
                //comm.print("Break: End of track.");
                break 'j_loop;
            }

            //comm.print(&format!("Scanning: note_start: {}.", note_start));

            let note_opt = RpmNote::parse_1note(comm, record_for_json, note_start, board_size);

            match note_opt {
                Some(note) => {
                    if note.is_phase_change() {
                        if is_first {

                        } else {
                            //comm.print("Break: Phase change.");
                            break 'j_loop;
                        }
                    }

                    //comm.print(&format!("Push: {:?}.", note));
                    rmove.notes.push(note);
                }
                None => {
                    //comm.print("Break: None.");
                    break 'j_loop;
                }
            }

            is_first = false;
        }

        if rmove.is_empty() {
            None
        } else if rmove.len() == 1 {
            panic!(
                "指し手が 1ノート ということは無いはず。 {:?}",
                record_for_json.body.operation
            )
        } else {
            rmove.end = *note_start;
            Some(rmove)
        }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// この指し手が、どの駒が動いたものによるものなのか、またどこにあった駒なのかを返します。
    pub fn to_first_touch_piece_id(&self, board_size: BoardSize) -> (PieceIdentify, Address) {
        // とりあえず USI move に変換するついでに、欲しい情報を得る。
        let (_umove, first_touch_pid, first_touch_addr) = self.to_usi_move(board_size);

        (first_touch_pid, first_touch_addr)
    }

    /// 一手。フェーズ・チェンジ・ノートや「ほこり取り」は含まない。
    ///
    /// 決まっている並びをしているものとする。
    ///
    /// # Returns
    ///
    /// Usi move,
    /// どの駒を動かした一手か,
    /// どこの駒を動かした一手か,
    pub fn to_usi_move(&self, board_size: BoardSize) -> (UsiMove, PieceIdentify, Address) {
        //let mut touched_source = false;

        let mut first_touch_idp_opt;
        let mut first_touch_address_opt;
        let mut src_opt = None;
        let mut dst_opt = None;
        let mut promotion = false;
        let mut drop_opt = None;

        let mut i = 0;
        let mut note = if i < self.notes.len() {
            self.notes[i]
        } else {
            panic!(
                "Unexpected 1st note of move(10): 超え。 {}/{}, {}",
                i,
                self.notes.len(),
                self.to_log(board_size)
            );
        };

        // 盤上の自駒 or 盤上の相手の駒 or 駒台の自駒
        if let Some(address) = note.get_ope().address {
            if let Some(piece) = address.get_hand_piece() {
                // 駒台なら必ず自駒のドロップ。
                first_touch_idp_opt = note.get_id();
                first_touch_address_opt = Some(address);
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
                        self.to_log(board_size)
                    );
                }

                if let Some(address) = note.get_ope().address {
                    dst_opt = Some(board_size.address_to_cell(address.get_index()));
                } else {
                    panic!(
                        "Unexpected 1st note of move(30): {}.",
                        note.to_log(board_size)
                    );
                }

            // その次は無い。

            // 盤上
            } else {
                // これが盤上の自駒か、相手の駒かは、まだ分からない。仮に入れておく。
                first_touch_idp_opt = note.get_id();
                first_touch_address_opt = Some(address);
                src_opt = Some(board_size.address_to_cell(address.get_index()));

                // 次。
                i += 1;
                note = self.notes[i];

                if note.get_ope().sky_turn {
                    // +。駒を裏返した。自駒を成ったのか、取った成り駒を表返したのかは、まだ分からない。仮に成ったことにしておく。
                    promotion = true;

                    // 次。
                    i += 1;
                    if i < self.notes.len() {
                        note = self.notes[i];
                    } else {
                        panic!(
                            "Unexpected 1st note of move(50): 超え。 {}/{}, {}",
                            i,
                            self.notes.len(),
                            self.to_log(board_size)
                        );
                    }
                }

                if note.get_ope().sky_rotate {
                    // -。向きを変えているようなら、相手の駒を取ったようだ。いろいろキャンセルする。
                    first_touch_idp_opt = None;
                    first_touch_address_opt = None;
                    src_opt = None;
                    promotion = false;

                    // 次。
                    i += 1;
                    if i < self.notes.len() {
                        note = self.notes[i];
                    } else {
                        panic!(
                            "Unexpected 1st note of move(40): 超え。 {}/{}, {}",
                            i,
                            self.notes.len(),
                            self.to_log(board_size)
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
                                self.to_log(board_size)
                            );
                        }

                        if let Some(address) = note.get_ope().address {
                            first_touch_idp_opt = note.get_id();
                            first_touch_address_opt = Some(address);
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
                                    self.to_log(board_size)
                                );
                            }
                        }
                    } else {
                        panic!(
                            "Unexpected 1st note of move(70): {}.",
                            note.to_log(board_size)
                        );
                    }
                } else {
                    // 盤上の自駒を触ったのだと確定した。
                }

                if note.get_ope().sky_turn {
                    // +。盤上の自駒が成った。
                    promotion = true;

                    // 次。
                    i += 1;
                    if i < self.notes.len() {
                        note = self.notes[i];
                    } else {
                        panic!(
                            "Unexpected 1st note of move(90): 超え。 {}/{}, {}",
                            i,
                            self.notes.len(),
                            self.to_log(board_size)
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
                note.to_log(board_size)
            );
        }

        if i + 1 < self.notes.len() {
            panic!(
                "Unexpected 1st note of move(110): 余り。 {}/{}, {}",
                i,
                self.notes.len(),
                self.to_log(board_size)
            );
        }

        /*
        for note in &self.notes {
            // 数が入っているとき。
            if let Some(address) = note.get_ope().address {
                if let Some(piece) = address.get_hand_piece() {
                    // 駒台を操作してるので　取った駒か、打った駒。
                    if !touched_source {
                        ftp_id_opt = note.get_id();
                        ftp_addr = Some(address);
                        touched_source = true;
                    }
                    drop_opt = Some(PieceType::from_piece(piece));

                // 盤上
                } else if !touched_source {
                    // 先に駒台を触るので、盤上の駒を触ったら、上書きして盤上の駒を優先します。
                    ftp_id_opt = note.get_id();
                    ftp_addr = Some(address);
                    src_opt = Some(board_size.address_to_cell(address.get_index()));
                    touched_source = true;
                } else {
                    dst_opt = Some(board_size.address_to_cell(address.get_index()));
                }
            } else if note.get_ope().sky_turn {
                // +
                promotion = true;
            } else if note.get_ope().sky_rotate {
                // -
            }
        }
        */

        let umove = if let Some(drop) = drop_opt {
            UsiMove::create_drop(dst_opt.unwrap(), drop, board_size)
        } else if let Some(dst) = dst_opt {
            UsiMove::create_walk(src_opt.unwrap(), dst, promotion, board_size)
        } else {
            panic!(
                "Unexpected dst. move.len: '{}' > 1, move: '{}'.",
                self.len(),
                self
            )
        };

        // USIの指し手が作れれば、 first touch が分からないことはないはず。
        if let Some(ftp_id) = first_touch_idp_opt {
            (umove, ftp_id, first_touch_address_opt.unwrap())
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
}
