use instrument::piece_etc::*;
use musician::best_move::*;
use sheet_music_format::kifu_usi::usi_move::*;
use std::fmt;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::*;
use studio::common::closed_interval::*;
use video_recorder::cassette_deck::CassetteDeck;
use video_recorder::cassette_deck::Slot;
use video_recorder::cassette_tape_box::CassetteTapeBox;

/// カセット・テープ上の閉区間（１手分）。
///
/// 右端をフェーズ・チェンジとして含む。
/// 最後のムーブのみ、フェーズチェンジを含まなくても構わない。
pub struct ShogiMove {
    // カセット・テープ上の閉区間。
    pub caret_closed_interval: ClosedInterval,
}
impl fmt::Display for ShogiMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.caret_closed_interval.to_human_presentable())
    }
}
impl ShogiMove {
    pub fn new_facing_right_move() -> Self {
        ShogiMove {
            caret_closed_interval: ClosedInterval::new_facing_right(),
        }
    }
    /// 次の1手分解析。
    ///
    /// # Arguments
    ///
    /// (parsed_note_count, move_opt)
    ///
    /// parsed_note_count は巻き戻すのに使う。
    pub fn parse_1move(
        tape_box: &CassetteTapeBox,
        caret: &mut Caret,
        _board_size: BoardSize,
        app: &Application,
    ) -> Option<ShogiMove> {
        let brandnew = ShogiMove::new_facing_right_move();

        let mut is_first = true;

        // 次のフェーズ・チェンジまで読み進める。
        'j_loop: loop {
            if tape_box.is_before_caret_overflow(&app) {
                // トラックの終わり。
                //comm.print("Break: End of track.");
                break 'j_loop;
            }

            let note = if let (_caret_number, Some(note)) =
                tape_box.go_to_next_with_othre_caret(caret, &app)
            {
                note
            } else {
                // パースできるノートが無かった。
                //comm.print("Break: None.");
                break 'j_loop;
            };

            if note.is_phase_change() && !is_first {
                // フェーズの変わり目で終わり。
                //comm.print("Break: Phase change.");
                break 'j_loop;
            }

            //comm.print(&format!("Push: {:?}.", note));
            //closed_interval.intersect_closed_interval(sub_closed_interval);

            is_first = false;
        }

        if brandnew.caret_closed_interval.is_empty() {
            None
        } else if brandnew.caret_closed_interval.len() == 1 {
            panic!("指し手が 1ノート ということは無いはず。")
        } else {
            Some(brandnew)
        }
    }

    pub fn len(&self) -> usize {
        self.caret_closed_interval.len()
    }

    pub fn is_empty(&self) -> bool {
        self.caret_closed_interval.is_empty()
    }

    /*
    /// この指し手が、どの駒が動いたものによるものなのか、またどこにあった駒なのかを返します。
    ///
    /// # Returns
    ///
    /// (どの駒を動かした一手か, どこの駒を動かした一手か, あれば取った駒，取った駒の番地)
    pub fn to_first_touch_piece_id(
        &self,
        tape_box: &CassetteTapeBox,
        board_size: BoardSize,
        app: &Application,
    ) -> (
        PieceIdentify,
        Address,
        Option<PieceIdentify>,
        Option<Address>,
    ) {
        // とりあえず USI move に変換するついでに、欲しい情報を得る。
        // (_umove, subject_pid, subject_addr, object_pid_opt, object_address_opt)
        let best_move = self.to_usi_move(&tape_box, board_size, &app);

        (
            best_move.subject_pid,
            best_move.subject_addr,
            best_move.capture_pid,
            best_move.capture_addr,
        )
    }
    */

    /// 一手。フェーズ・チェンジ・ノートや「ほこり取り」は含まない。
    ///
    /// 決まっている並びをしているものとする。
    ///
    /// # Returns
    ///
    /// (Usi move, どの駒を動かした一手か, どこの駒を動かした一手か, あれば取った駒，取った駒の番地)
    pub fn to_best_move(
        &self,
        deck: &CassetteDeck,
        slot: Slot,
        board_size: BoardSize,
        app: &Application,
    ) -> BestMove {
        if let Some(tape_box) = &deck.slots[slot as usize].tape_box {
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

            let mut caret =
                Caret::new_facing_right_caret_with_number(self.caret_closed_interval.get_start());

            let mut note = if let (_caret_number, Some(note)) =
                tape_box.go_to_next_with_othre_caret(&mut caret, &app)
            {
                note
            } else {
                panic!("Note fail.")
            };

            // 盤上の自駒 or 盤上の相手の駒 or 駒台の自駒
            if let Some(address) = note.get_ope().address {
                if let Some(piece) = address.get_hand_piece() {
                    // 駒台なら必ず自駒のドロップ。
                    subject_pid_opt = note.get_id();
                    subject_address_opt = Some(address);
                    drop_opt = Some(PieceType::from_piece(piece));

                    // 次は置くだけ。
                    note = if let (_caret_number, Some(note)) =
                        tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                    {
                        note
                    } else {
                        panic!("Note fail.")
                    };

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
                    note = if let (_caret_number, Some(note)) =
                        tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                    {
                        note
                    } else {
                        panic!("Note fail.")
                    };

                    if note.get_ope().fingertip_turn {
                        // +。駒を裏返した。自駒を成ったのか、取った成り駒を表返したのかは、まだ分からない。仮に成ったことにしておく。
                        subject_promotion = true;

                        // 次。
                        note = if let (_caret_number, Some(note)) =
                            tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                        {
                            note
                        } else {
                            panic!("Note fail.")
                        };
                    }

                    if note.get_ope().fingertip_rotate {
                        // -。向きを変えているようなら、相手の駒を取ったようだ。いろいろキャンセルする。
                        object_pid_opt = subject_pid_opt;
                        object_address_opt = subject_address_opt;
                        _object_promotion = subject_promotion;
                        subject_pid_opt = None;
                        subject_address_opt = None;
                        src_opt = None;
                        subject_promotion = false;

                        // 次。
                        note = if let (_caret_number, Some(note)) =
                            tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                        {
                            note
                        } else {
                            panic!("Note fail.")
                        };

                        // 自分の駒台に置く動き。
                        if let Some(_address) = note.get_ope().address {
                            // 次は、盤上の自駒を触る。
                            note = if let (_caret_number, Some(note)) =
                                tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                            {
                                note
                            } else {
                                panic!("Note fail.")
                            };

                            if let Some(address) = note.get_ope().address {
                                subject_pid_opt = note.get_id();
                                subject_address_opt = Some(address);
                                src_opt = Some(board_size.address_to_cell(address.get_index()));
                                // 次。
                                note = if let (_caret_number, Some(note)) =
                                    tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                                {
                                    note
                                } else {
                                    panic!("Note fail.")
                                };
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

                    if note.get_ope().fingertip_turn {
                        // +。盤上の自駒が成った。
                        subject_promotion = true;

                        // 次。
                        note = if let (_caret_number, Some(note)) =
                            tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                        {
                            note
                        } else {
                            panic!("Note fail.")
                        };
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

            /*
            if i + 1 < self.notes.len() {
                panic!(
                    "Unexpected 1st note of move(110): 余り。 {}/{}, {}",
                    i,
                    self.notes.len(),
                    self.to_human_presentable(board_size)
                );
            }
            */

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
                BestMove {
                    usi_move: umove,
                    subject_pid: subject_idp,
                    subject_addr: subject_address_opt.unwrap(),
                    capture_pid: object_pid_opt,
                    capture_addr: object_address_opt,
                }
            } else {
                panic!("Unexpected rpm move. id fail.")
            }
        } else {
            panic!("Tape box fail.")
        }
    }

    pub fn to_operation_string(
        &self,
        tape_box: &CassetteTapeBox,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        let mut caret =
            Caret::new_facing_right_caret_with_number(self.caret_closed_interval.get_start());

        let mut text = String::new();
        while caret.while_to(&self.caret_closed_interval) {
            if let (_caret_number, Some(note)) =
                tape_box.go_to_next_with_othre_caret(&mut caret, &app)
            {
                text = format!("{} {}", text, &note.get_ope().to_sign(board_size));
            } else {
                break;
            }
        }

        text
    }

    pub fn to_identify_string(&self, tape_box: &CassetteTapeBox, app: &Application) -> String {
        let mut caret =
            Caret::new_facing_right_caret_with_number(self.caret_closed_interval.get_start());

        let mut text = String::new();
        while caret.while_to(&self.caret_closed_interval) {
            if let (_caret_number, Some(note)) =
                tape_box.go_to_next_with_othre_caret(&mut caret, &app)
            {
                text = format!(
                    "{} {}",
                    text,
                    match &note.get_id() {
                        Some(pid) => pid.get_number().to_string(),
                        None => "-1".to_string(),
                    }
                );
            } else {
                break;
            }
        }

        text
    }

    /// Human presentable.
    pub fn to_human_presentable(
        &self,
        deck: &CassetteDeck,
        slot: Slot,
        board_size: BoardSize,
        app: &Application,
    ) -> String {
        if let Some(tape_box) = &deck.slots[slot as usize].tape_box {
            let mut caret =
                Caret::new_facing_right_caret_with_number(self.caret_closed_interval.get_start());

            let mut text = String::new();
            while caret.while_to(&self.caret_closed_interval) {
                if let (_caret_number, Some(note)) =
                    tape_box.go_to_next_with_othre_caret(&mut caret, &app)
                {
                    text = format!("{} {}", text, note.to_human_presentable(board_size))
                } else {
                    break;
                }
            }

            // TODO スタートが181で、エンドが1だったりするのはなんでだぜ☆（＾～＾）？
            format!(
                "{}{}",
                self.caret_closed_interval.to_human_presentable(),
                text
            )
        } else {
            panic!("None tape box.")
        }
    }
}
