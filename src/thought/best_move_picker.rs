use address::Address;
use common::caret::*;
use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use human::human_interface::*;
use kifu_rpm::json::rpm_cassette_tape_box_for_json::*;
use kifu_rpm::json::rpm_cassette_tape_for_json::*;
use kifu_rpm::play::rpm_move_player::*;
use kifu_rpm::play::rpm_thread_player::*;
use kifu_rpm::rpm_cassette_tape_recorder::*;
use kifu_rpm::thread::rpm_move::*;
use kifu_rpm::thread::rpm_thread::*;
use piece_etc::*;
use position::*;
use std::collections::HashMap;
use std::fs;
use usi_conv::usi_move::*;

pub struct BestMovePicker {
    thread_by_piece_id: HashMap<i8, RpmThread>,
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
            let thread = RpmThread::new();
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
    pub fn get_best_move(
        &mut self,
        comm: &Communication,
        kw29config: &KifuwarabeWcsc29Config,
        recorder: &mut RpmCassetteTapeRecorder,
        position: &mut Position,
    ) -> UsiMove {
        // クリアー。
        self.clear();

        // RPMを検索。
        println!("#get_best_move start. Phase: {:?}", position.get_phase());

        // TODO とりあえず -rpmrec.json ファイルを１個読む。
        'path_loop: for path in fs::read_dir(&kw29config.rpm_record).unwrap() {
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

            let book_file = RpmCassetteTapeBoxFile::load(&file);

            // ファイルの中身をすこし見てみる。
            //comm.println(&format!("file: {}, Book len: {}.", file, book_file.book.len() ));
            if !book_file.book.is_empty() {
                //comm.println(&format!("Ope len: {}, Num len: {}.", book_file.book[0].body.operation.len(), book_file.book[0].body.piece_number.len() ));

                let mut record_index = -1;

                // レコードがいっぱいある。
                for record_for_json in book_file.book {
                    record_index += 1;
                    comm.println(&format!(
                        "Record index: {}. Json: {}",
                        record_index,
                        record_for_json.to_human_presentable()
                    ));

                    // 駒（0～40個）の番地を全部スキャン。（駒の先後は分からない）
                    // 'piece_loop:
                    for my_piece_id in PieceIdentify::iterator() {
                        // 現局面の盤上の自駒の番地。
                        if let Some((my_idp, my_addr_obj)) =
                            position.find_wild(Some(position.get_phase()), *my_piece_id)
                        {
                            // Display.
                            HumanInterface::bo(
                                &comm,
                                &recorder.cassette_tape,
                                recorder.ply,
                                &position,
                            );
                            comm.println(&format!(
                                "[{}] Find: {}'{}'{}.",
                                recorder.ply,
                                position.get_phase().to_log(),
                                my_idp.to_human_presentable(),
                                my_addr_obj.to_physical_sign(position.get_board_size())
                            ));

                            // ノートをスキャン。
                            // TODO 次方向と、前方向がある。
                            let mut note_caret = Caret::new_next_caret();
                            let mut thread = RpmThread::new();
                            //'track_scan:
                            loop {
                                let rmove_opt = self.go_pattern_match_move(
                                    &comm,
                                    position,
                                    &record_for_json,
                                    *my_piece_id,
                                    my_addr_obj,
                                    &mut note_caret,
                                );

                                if let Some(rmove) = rmove_opt {
                                    comm.println(&format!(
                                        "Push move! {}",
                                        rmove.to_human_presentable(position.get_board_size())
                                    ));
                                    thread.push_move(rmove);
                                } else {
                                    break;
                                }
                            }
                            let thread_len = thread.len() as i16;
                            let thread_to_human_presentable =
                                thread.to_human_presentable(position.get_board_size());
                            self.thread_by_piece_id
                                .insert(my_piece_id.get_number(), thread);

                            // 局面を動かしてしまったので戻す。
                            recorder.cassette_tape.caret.turn_to_opponent();
                            comm.println(&format!(
                                "Legal, go back! thread_len: {}, thread: {}",
                                thread_len, thread_to_human_presentable,
                            ));
                            RpmThreadPlayer::get_n_move_and_go(
                                thread_len, recorder, position, comm,
                            );
                            recorder.cassette_tape.caret.turn_to_opponent();
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
        rmove: &RpmMove,
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

    /// 次方向へのパターン・マッチ。
    /// 正常終了すると、トラックの検索を続行します。
    ///
    /// # Returns
    ///
    /// (move_opt)
    pub fn go_pattern_match_move(
        &mut self,
        comm: &Communication,
        position: &mut Position,
        record_for_json: &RpmCasetteTapeForJson,
        my_piece_id: PieceIdentify,
        my_addr_obj: Address,
        note_caret: &mut Caret,
    ) -> Option<RpmMove> {
        /*
        comm.println(&format!(
            "#>{} note.",
            note_caret.to_human_presentable()
        ));
        */
        // とりあえず 1手分をパースします。
        if let (_parsed_note_count, Some(rmove)) = RpmMove::parse_1move(
            comm,
            &record_for_json,
            note_caret,
            position.get_board_size(),
        ) {
            // どの駒が動いた１手なのか、またその番地。
            // 取った駒があるのなら、それも欲しい。
            let (subject_pid, subject_address, opject_pid_opt, object_address_opt) =
                rmove.to_first_touch_piece_id(position.get_board_size());

            comm.println(&format!(
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
                comm,
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
                let mut recorder = RpmCassetteTapeRecorder::default();
                recorder.record_move(&rmove, comm);
                recorder.reset_caret();
                println!(
                    "BMP: This move rtape: {}.",
                    recorder.to_human_presentable(position.get_board_size())
                );

                // 試しに1手進めます。（非合法タッチは自動で戻します）
                if RpmMovePlayer::get_1move_and_go(
                    &mut recorder.cassette_tape,
                    position,
                    recorder.ply,
                    true,
                    &comm,
                ) {
                    // 合法タッチ。
                    // TODO とりあえず抜けて次の駒へ。
                    comm.println(&format!(
                        "Hit and break! ({}) {}",
                        subject_pid.to_human_presentable(),
                        &rmove.to_human_presentable(position.get_board_size())
                    ));
                    HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
                    Some(rmove)
                } else {
                    // 非合法タッチ。（自動で戻されています）
                    comm.println(&format!(
                        "Canceled: {}.",
                        rmove.to_human_presentable(position.get_board_size())
                    ));
                    HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
                    None
                }
            } else {
                // パターン不一致。
                None
            }
        } else {
            // トラックの終わり。
            comm.println("Break: End of track.");
            None
        }
    }
}
