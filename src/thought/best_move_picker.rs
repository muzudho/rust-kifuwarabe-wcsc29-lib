use address::Address;
use application::Application;
use common::caret::*;
use communication::*;
use human::human_interface::*;
use kifu_rpm::rpm_tape::*;
use kifu_rpm::rpm_tape_box::*;
use kifu_usi::usi_move::*;
use object_rpm::cassette_deck::*;
use object_rpm::cassette_tape::*;
use object_rpm::cassette_tape_box::*;
use object_rpm::shogi_move::*;
use object_rpm::shogi_thread::*;
use piece_etc::*;
use shogi_ban::game_player::*;
use shogi_ban::position::*;
use std::collections::HashMap;
use std::fs;

pub struct BestMovePicker {
    thread_by_piece_id: HashMap<i8, ShogiThread>,
}
impl BestMovePicker {
    pub fn default() -> Self {
        let mut instance = BestMovePicker {
            thread_by_piece_id: HashMap::new(),
        };

        instance.clear();

        instance
    }

    /// 初期状態をセットします。
    pub fn clear(&mut self) {
        self.thread_by_piece_id.clear();

        for id in PieceIdentify::iterator() {
            let number = id.get_number();
            let thread = ShogiThread::new();
            self.thread_by_piece_id.insert(number, thread);
        }
    }

    pub fn get_max_note_len(&self) -> usize {
        let mut max = 0;

        for thread in self.thread_by_piece_id.values() {
            if max < thread.len() {
                max = thread.len();
            }
        }

        max
    }

    /// TODO 学習ファイルをもとに動く。
    pub fn get_mut_best_move(
        &mut self,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) -> UsiMove {
        // クリアー。
        self.clear();

        // RPMを検索。
        println!(
            "#get_mut_best_move start. Phase: {:?}",
            position.get_phase()
        );

        // TODO とりあえず -rbox.json ファイルを１個読む。
        'path_loop: for path in fs::read_dir(&app.kw29_conf.training).unwrap() {
            let file = path.unwrap().path().display().to_string();

            /*
            // 確認表示。
            {
                use piece_etc::PieceIdentify::*;
                comm.println(&format!("info file: {}, Phase: {:?}.", file, position.get_phase()));
                HumanInterface::show_position(&comm, -1, &position);
                // 先手玉の番地。
                {
                    if let Some((_idp,addr_obj)) = position.find_wild(Some(Phase::First), K00) {
                        comm.println(&format!("info First-K00: {}.", addr_obj.get_index()));
                    }
                }
                {
                    if let Some((_idp,addr_obj)) = position.find_wild(Some(Phase::First), K01) {
                        comm.println(&format!("info First-K01: {}.", addr_obj.get_index()));
                    }
                }
                // 後手玉の番地。
                {
                    if let Some((_idp,addr_obj)) = position.find_wild(Some(Phase::Second), K00) {
                        comm.println(&format!("info Second-K00: {}.", addr_obj.get_index()));
                    }
                }
                {
                    if let Some((_idp,addr_obj)) = position.find_wild(Some(Phase::Second), K01) {
                        comm.println(&format!("info Second-K01: {}.", addr_obj.get_index()));
                    }
                }
            }
            */

            let cassette_tape_box_j = RpmTapeBox::from_box_file(&file);

            // ファイルの中身をすこし見てみる。
            //comm.println(&format!("file: {}, Book len: {}.", file, cassette_tape_box_j.book.len() ));
            if !cassette_tape_box_j.tape_box.is_empty() {
                //comm.println(&format!("Ope len: {}, Num len: {}.", cassette_tape_box_j.book[0].body.operation.len(), cassette_tape_box_j.book[0].body.piece_number.len() ));

                let mut record_index = -1;

                // カセット・テープがいっぱいある。
                for cassette_tape_j in cassette_tape_box_j.tape_box {
                    record_index += 1;
                    app.comm.println(&format!(
                        "Tape index: {}. Json: {}",
                        record_index,
                        cassette_tape_j.to_human_presentable()
                    ));

                    // 駒（0～40個）の番地を全部スキャン。（駒の先後は分からない）
                    // 'piece_loop:
                    for my_piece_id in PieceIdentify::iterator() {
                        // 現局面の盤上の自駒の番地。
                        if let Some((my_idp, my_addr_obj)) =
                            position.find_wild(Some(position.get_phase()), *my_piece_id)
                        {
                            // Display.
                            HumanInterface::bo(deck, Slot::Learning, &position, &app);

                            app.comm.println(&format!(
                                "[{}] Find: {}'{}'{}.",
                                deck.get_ply(Slot::Training),
                                position.get_phase().to_log(),
                                my_idp.to_human_presentable(),
                                my_addr_obj.to_physical_sign(position.get_board_size())
                            ));

                            // ノートをスキャン。
                            // TODO 次方向と、前方向がある。
                            let mut note_caret = Caret::new_facing_right_caret();
                            let mut record_count = 0;
                            loop {
                                // 一致して続行か、一致しなくて続行か、一致せずテープの終わりだったかの３択☆（＾～＾）
                                let (rmove_opt, is_end_of_tape) = self.pattern_match_and_go(
                                    position,
                                    &cassette_tape_j,
                                    *my_piece_id,
                                    my_addr_obj,
                                    &mut note_caret,
                                    &app,
                                );

                                if is_end_of_tape {
                                    // テープの終わりなら仕方ない☆（＾～＾）終わりだぜ☆（＾～＾）
                                    break;
                                } else if let Some(rmove) = rmove_opt {
                                    // ヒットしたようだぜ☆（＾～＾）
                                    record_count += 1;
                                    app.comm.println(&format!(
                                        "{} hit! Rmove: {}.",
                                        record_count,
                                        rmove.to_human_presentable(position.get_board_size())
                                    ));
                                    // thread.push_move(rmove);
                                    // とりあえず抜ける☆（＾～＾）
                                    break;
                                } else {
                                    // 一致しなかった☆（＾～＾）
                                    // 見つかるか、テープの終わりまで、続行して探せだぜ☆（＾～＾）
                                }
                            }

                            // let mut thread = ShogiThread::new();
                            // let thread_len = thread.len() as i16;
                            // let thread_to_human_presentable =
                            //    thread.to_human_presentable(position.get_board_size());
                            // self.thread_by_piece_id
                            //    .insert(my_piece_id.get_number(), thread);

                            // 指した手数分、後ろ向きに読み進めながら記録しろだぜ☆（＾～＾）
                            // それを逆順にすれば　指し手だぜ☆（＾～＾）
                            deck.turn_caret_to_opponent(Slot::Learning);
                            app.comm
                                .println(&format!("Tried, go opponent {} move!", record_count,));
                            {
                                let learning_slot = &mut deck.slots[Slot::Learning as usize];
                                if let Some(ref mut tape_box) = learning_slot.tape_box {
                                    GamePlayer::read_tape_for_n_moves_forcely(
                                        tape_box,
                                        record_count,
                                        position,
                                        learning_slot.ply,
                                        &app,
                                    );
                                } else {
                                    panic!("Tape box none.");
                                }
                            }
                            deck.turn_caret_to_opponent(Slot::Learning);
                            app.comm.println("Backed.");
                        }
                    }

                    // いくつか読み取れれば打ち止め。
                    if self.get_max_note_len() > 4 {
                        println!("#Break. Exit piece count = {}.", self.get_max_note_len());
                        break 'path_loop;
                    }
                } // record_loop
            } // book
        } // path_loop

        //println!("#match_thread loop end.");

        let mut best_rpm_move_opt = None;

        // １つチョイスしようぜ☆（*＾～＾*）
        'choice: for pid in PieceIdentify::iterator() {
            let pid_num = pid.get_number();
            let thread = &self.thread_by_piece_id[&pid_num];

            // Header.
            // println!("Pid: {}.", pid_num);

            // とりあえず１つチョイス☆（＾～＾）
            if !thread.is_empty() {
                best_rpm_move_opt = Some(&thread.moves[0]);

                // 検索結果を見てみようぜ☆（＾～＾）
                // Operation.
                // println!("  Ope: {} End.", rmove.to_operation_string(position.get_board_size()));

                // Identify.
                // println!("  Num: {} End.", rmove.to_identify_string());
                break 'choice;
            }
        }

        // let thread = ThreadsOfPiece {
        //     max_ply: 0,
        //     record: RpmRecord::default(),
        // };

        // 自分の駒ごとの、現局面にマッチする最長の手筋を更新していく。

        if let Some(best_rpm_move) = best_rpm_move_opt {
            let (umove, _subject_pid, _subject_address, _object_pid_opt, _object_address_opt) =
                best_rpm_move.to_usi_move(position.get_board_size());
            umove
        } else {
            UsiMove::create_resign()
        }
    }

    /// 現局面が一致しているか判定。
    pub fn position_match(
        &mut self,
        _comm: &Communication,
        position: &mut Position,
        my_piece_id: PieceIdentify,
        my_addr_obj: Address,
        rmove: &ShogiMove,
        subject_pid: PieceIdentify,
        subject_address: Address,
        object_address_opt: Option<Address>,
    ) -> bool {
        // パターンマッチから外れたら抜けていく。
        if my_piece_id.get_number() != subject_pid.get_number()
            || subject_address.get_index() != my_addr_obj.get_index() as usize
        {
            // No match. 背番号と、アドレスが不一致なら何もしない。
            /*
            comm.println(
                "#No-match. 背番号と、アドレスが不一致なら、何もせずループを続行。",
            );
            */
            return false; // continue 'track_scan;
        }

        // TODO 番地を指定して、そこにある駒が　相手の駒か判定。合法手だけを残す。
        if let Some(addr) = object_address_opt {
            if let Some(cell) = addr.to_cell(position.get_board_size()) {
                if let Some(idp) = position.get_id_piece(cell) {
                    if let Some(_is_opponent) = idp.is_opponent(position) {
                        // 相手の駒を取った合法手。
                    } else {
                        /*
                        comm.println(&format!(
                            "#IL-味方の駒を取ってしまうなら、何もせずループを続行。{}",
                            rmove.to_human_presentable(
                                position.get_board_size()
                            )
                        ));
                            */
                        return false; // continue 'track_scan;
                    }
                } else {
                    // 現局面では、取ろうとした駒がなかった。
                    return false; // continue 'track_scan;
                }
            } else {
                // プログラムの不具合。
                panic!(
                    "#IL-盤上以外の駒を取った(1)。{}",
                    rmove.to_human_presentable(position.get_board_size())
                );
            }
        } else {
            // 駒を取らなかった合法手。
        };

        // パターンがマッチした。
        //comm.println(&format!("matched address. address={}.", my_addr_obj.get_index()));
        true
    }

    /// 指し手単位での、パターン・マッチ。
    /// 一致したか、一致しなかったか、一致せずテープの終わりだったかの３択☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (move_opt, is_end_of_tape)
    pub fn pattern_match_and_go(
        &mut self,
        position: &mut Position,
        cassette_tape_j: &RpmTape,
        my_piece_id: PieceIdentify,
        my_addr_obj: Address,
        note_caret: &mut Caret,
        app: &Application,
    ) -> (Option<ShogiMove>, bool) {
        /*
        comm.println(&format!(
            "#>{} note.",
            note_caret.to_human_presentable()
        ));
        */
        // とりあえず 1手分をパースします。
        if let (_parsed_note_count, Some(rmove)) = ShogiMove::parse_1move(
            &app.comm,
            &cassette_tape_j,
            note_caret,
            position.get_board_size(),
        ) {
            // どの駒が動いた１手なのか、またその番地。
            // 取った駒があるのなら、それも欲しい。
            let (subject_pid, subject_address, opject_pid_opt, object_address_opt) =
                rmove.to_first_touch_piece_id(position.get_board_size());

            app.comm.println(&format!(
                "#{}Rmove:{}. subject('{}'{}){}",
                note_caret.to_human_presentable(),
                rmove.to_human_presentable(position.get_board_size()),
                subject_pid.to_human_presentable(),
                subject_address.to_human_presentable(position.get_board_size()),
                if let Some(object_pid) = opject_pid_opt {
                    format!(
                        " object('{}'{})",
                        object_pid.to_human_presentable(),
                        object_address_opt
                            .unwrap()
                            .to_human_presentable(position.get_board_size())
                    )
                    .to_string()
                } else {
                    "".to_string()
                }
            ));

            if self.position_match(
                &app.comm,
                position,
                my_piece_id,
                my_addr_obj,
                &rmove,
                subject_pid,
                subject_address,
                object_address_opt,
            ) {
                // 局面と一致。
                // TODO 現局面で この手を指せるか試してみる。
                // 例えば 味方の駒の上に駒を動かすような動きは イリーガル・タッチ として弾く。

                // 新規に テープを作る。ムーブ１つだけ。
                //let mut recorder = CassetteTapeEditor::new_cassette_tape_editor();
                //recorder.put_1note(&rmove, comm);
                //recorder.reset_caret();
                let mut ply_2 = 1;
                let mut cassette_tape_box_2 = CassetteTapeBox::new_empty(&app);
                {
                    let mut cassette_tape_2 = CassetteTape::from_1_move(&rmove, &app);
                    cassette_tape_box_2.change_with_training_tape(cassette_tape_2);
                }
                /*
                println!(
                    "BMP: This move rtape: {}.",
                    recorder.to_human_presentable(position.get_board_size())
                );
                 */

                // 試しに1手進めます。（非合法タッチは自動で戻します）
                if GamePlayer::try_read_tape_for_1move(
                    &mut cassette_tape_box_2,
                    position,
                    ply_2,
                    &app,
                ) {
                    // 合法タッチ。戻さず抜けます。
                    app.comm.println(&format!(
                        "Hit and go! ({}) {}",
                        subject_pid.to_human_presentable(),
                        &rmove.to_human_presentable(position.get_board_size())
                    ));
                    HumanInterface::bo_with_tape(&cassette_tape_box_2, ply_2, &position, &app);
                    (Some(rmove), false)
                } else {
                    // 非合法タッチ。（自動で戻されています）
                    app.comm.println(&format!(
                        "Canceled: {}.",
                        rmove.to_human_presentable(position.get_board_size())
                    ));
                    HumanInterface::bo_with_tape(&cassette_tape_box_2, ply_2, &position, &app);
                    (None, false)
                }
            } else {
                // パターン不一致。
                (None, false)
            }
        } else {
            // テープの終わり。
            app.comm.println("Break: End of tape.");
            (None, true)
        }
    }
}
