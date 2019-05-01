use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::piece_etc::*;
use instrument::position::*;
use musician::best_move::BestMove;
use musician::best_thread::*;
use musician::best_thread_buffer::*;
use sheet_music_format::kifu_usi::usi_move::*;
use sound::shogi_move::ShogiMove;
use std::collections::HashMap;
use std::fs;
use studio::address::Address;
use studio::application::Application;
use studio::board_size::BoardSize;

pub struct BestMovePicker {
    // 確定した手筋だぜ☆（＾～＾）
    best_thread_map: HashMap<i8, BestThread>,

    // ここに 手筋 を追加していけだぜ☆（＾～＾）
    best_thread_buffer: BestThreadBuffer,
}
impl BestMovePicker {
    pub fn default() -> Self {
        let mut instance = BestMovePicker {
            best_thread_map: HashMap::new(),
            best_thread_buffer: BestThreadBuffer::new(),
        };

        instance.init_state();

        instance
    }

    /// 現在の内容を破棄し、初期位置に設定します。
    pub fn init_state(&mut self) {
        // マッピングの再設定。
        self.best_thread_map.clear();
        for id in PieceIdentify::iterator() {
            let number = id.get_number();
            let best_thread = BestThread::new();
            self.best_thread_map.insert(number, best_thread);
        }

        // 現在の内容を破棄☆（＾～＾）
        self.best_thread_buffer.clear();
    }

    /// 現在の内容を確定し、次の手筋にチェンジするぜ☆（*＾～＾*）
    pub fn change_thread(&mut self, subject_piece_id: PieceIdentify, app: &Application) {
        if !self.best_thread_buffer.is_empty() {
            app.comm.println(&format!(
                "[Change thread: subject_piece_id: {}, not empty]",
                subject_piece_id.to_human_presentable(),
            ));
            // 中身が残っていれば、まず確定☆（＾～＾）
            self.best_thread_map.insert(
                subject_piece_id.get_number(),
                self.best_thread_buffer.to_object(),
            );

            // 現在の内容を破棄☆（＾～＾）
            self.best_thread_buffer.clear();
        } else {
            app.comm.println(&format!(
                "[Change thread: subject_piece_id: {}, is empty]",
                subject_piece_id.to_human_presentable(),
            ));
        }
    }

    pub fn get_max_note_len(&self) -> usize {
        let mut max = 0;

        for best_thread in self.best_thread_map.values() {
            if max < best_thread.len() {
                max = best_thread.len();
            }
        }

        max
    }

    /// 最善手を返す。
    pub fn get_mut_best_move(
        &mut self,
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) -> UsiMove {
        // 状態を初期位置に設定します。
        self.init_state();

        // RPMを検索。
        println!(
            "#get_mut_best_move start. Phase: {:?}",
            position.get_phase()
        );

        // TODO とりあえず rbox.json ファイルを１個読む。
        'path_loop: for tape_box_file in fs::read_dir(&app.kw29_conf.training).unwrap() {
            // JSONファイルを元にオブジェクト化☆（＾～＾）
            deck.change_with_tape_box_file(
                &tape_box_file.unwrap().path().display().to_string(),
                position.get_board_size(),
                &app,
            );

            // トレーニング・テープ・ボックスを１箱選択。
            app.comm.println(&format!(
                "#Tape-box: {}. Phase: {:?}.",
                deck.to_human_presentable_of_tape_box(Slot::Training),
                position.get_phase()
            ));

            /*
            // 確認表示。
            {
                use piece_etc::PieceIdentify::*;
                HumanInterface::show_position(&comm, -1, &position);
                // 先手玉の番地。
                {
                    if let Some((_idp,addr_obj)) = position.scan_wild(Some(Phase::First), K00) {
                        comm.println(&format!("info First-K00: {}.", addr_obj.get_index()));
                    }
                }
                {
                    if let Some((_idp,addr_obj)) = position.scan_wild(Some(Phase::First), K01) {
                        comm.println(&format!("info First-K01: {}.", addr_obj.get_index()));
                    }
                }
                // 後手玉の番地。
                {
                    if let Some((_idp,addr_obj)) = position.scan_wild(Some(Phase::Second), K00) {
                        comm.println(&format!("info Second-K00: {}.", addr_obj.get_index()));
                    }
                }
                {
                    if let Some((_idp,addr_obj)) = position.scan_wild(Some(Phase::Second), K01) {
                        comm.println(&format!("info Second-K01: {}.", addr_obj.get_index()));
                    }
                }
            }
            */

            let mut debug_tape_count = -1;
            // テープをセット☆（＾～＾）
            while deck.change_next_if_training_tape_exists(&app) {
                debug_tape_count += 1;
                if 0 <= debug_tape_count && debug_tape_count <= 0 {
                    // ここだけテストするぜ☆（＾～＾）
                } else {
                    // それ以外は無視。
                    app.comm.println("デバッグ中☆（＾～＾）テープを中断。");
                    continue;
                }

                // テープを１本選択☆（＾～＾）
                app.comm.println(&format!(
                    "#Tape: {}",
                    deck.to_human_presentable_of_current_tape_of_training_box(
                        position.get_board_size(),
                        &app
                    )
                ));

                // 駒（0～40個）の番地を全部スキャン。（駒の先後は分からない）
                // 'piece_loop:
                for subject_piece_id in PieceIdentify::iterator() {
                    if 29 <= subject_piece_id.get_number() && subject_piece_id.get_number() <= 29 {
                        // ここだけテストするぜ☆（＾～＾）
                    } else {
                        // それ以外は無視。
                        app.comm.println("デバッグ中☆（＾～＾）ループを中断。");
                        continue;
                    }

                    // 駒を１つ選択☆（＾～＾）
                    app.comm.println(&format!(
                        "\n----------------------------------------------------------------------------------------------------------------------------------------------------------------#Subject piece: {}",
                        subject_piece_id.to_human_presentable()
                    ));

                    // 現局面の盤上の自駒の番地。
                    if let Some((my_idp, my_addr_obj)) =
                        position.scan_wild(Some(position.get_phase()), *subject_piece_id)
                    {
                        // Display.
                        app.comm.println(&format!(
                            "[{}] Recording thread by piece: {}'{}'{}.",
                            deck.get_ply(Slot::Training),
                            position.get_phase().to_log(),
                            my_idp.to_human_presentable(),
                            my_addr_obj.to_physical_sign(position.get_board_size())
                        ));
                        HumanInterface::bo(deck, &position, &app);

                        // １手ずつ、テープを最後尾に向かってスキャン。
                        // TODO 次方向と、前方向の両方へスキャンしたい。
                        // あとで巻き戻すために、進めた数を覚えておくぜ☆（＾～＾）
                        let mut forwarding_note_count: usize = 0;
                        'sequence_moves: loop {
                            'sequence_thread: loop {
                                app.comm.println(&format!(
                                    "\n--------------------------------------------------------------------------------#Note scan: {}th note of {} move of a thread. [Before pattern match: Caret: {}]",
                                    forwarding_note_count,
                                    self.best_thread_buffer.len(),
                                    deck.to_human_presentable_of_caret_of_current_tape(
                                        Slot::Training, &app
                                    ),
                                ));

                                // 以下の４択☆（＾～＾）
                                // （１）最後の１手分。局面もキャレットも進んでいる。
                                // （２）最後ではない１手分。局面もキャレットも進んでいる。
                                // （３）テープ終わっていた。キャレットを戻す。
                                // （４）実現しない操作だった。局面とキャレットを戻す。
                                match deck.try_read_tape_for_1move(Slot::Training, position, &app) {
                                    (is_end_of_tape, Some(rmove)) => {
                                        // この手は、タッチはできるみたいだな☆（＾～＾）

                                        // パターンにマッチしていようが、していまいが、タッチを確定したなら、キャレットは動かしてしまった☆（＾～＾）
                                        forwarding_note_count += rmove.len();

                                        // ベストムーブを作ろうぜ☆（＾～＾）
                                        let best_move = rmove.to_best_move(
                                            deck,
                                            Slot::Training,
                                            position.get_board_size(),
                                            &app,
                                        );

                                        // パターンマッチには２種類ある☆（＾～＾）
                                        // 主体となる駒まで指定する場合と、主体となる駒を指定しない場合だぜ☆（＾～＾）
                                        // 手筋の各1ムーブ目は、主体となるピースのものであるか判定する☆（＾～＾）
                                        if self.best_thread_buffer.is_empty() {
                                            if !self.match_subject_piece(
                                                *subject_piece_id,
                                                my_addr_obj,
                                                &best_move,
                                                position.get_board_size(),
                                                &app,
                                            ) {
                                                // 手筋の１個めが、主体となる駒で始まっていない☆（＾～＾）
                                                // 抜ける☆（＾～＾）
                                                app.comm.println(
                                                    "[主体となる駒のものではないぜ☆（＾～＾）]",
                                                );

                                                // これは、主体の駒の手筋にならない☆（＾～＾）抜けるぜ☆（＾～＾）
                                                break 'sequence_thread;
                                            }
                                        }

                                        if !self.match_object_piece(
                                            deck,
                                            Slot::Training,
                                            position,
                                            my_addr_obj,
                                            &rmove,
                                            &best_move,
                                            &app,
                                        ) {
                                            // 竹の節の境目たぜ☆（＾～＾）
                                            // この手の途中で止まっているキャレットを　ごそっと　次の１手まで進め、現在の手筋を確定しろだぜ☆（＾～＾）
                                            // ノートのループは続行する☆（＾～＾）
                                            app.comm.println("[途切れたぜ☆（＾～＾）]");
                                            deck.seek_1move(Slot::Training, &app);
                                            HumanInterface::bo(deck, &position, &app);
                                            self.change_thread(*subject_piece_id, &app);
                                        }

                                        // 今探している駒の指し手のような感じはするみたいだな☆（＾～＾）
                                        app.comm.println(&format!(
                                            "\n----------------------------------------[Hit {}th note! is_end_of_tape: {}, Move {} --> Best move: {}. Caret: {}]",
                                            forwarding_note_count,
                                            rmove.to_human_presentable(
                                                deck,
                                                Slot::Training,
                                                position.get_board_size(),
                                                &app),
                                            is_end_of_tape,
                                            best_move.to_human_presentable(position.get_board_size(), &app),
                                            deck.to_human_presentable_of_caret_of_current_tape(
                                                Slot::Training, &app
                                            ),
                                        ));

                                        self.best_thread_buffer.push_move(best_move);
                                        // 手筋のループは続行だぜ☆（＾～＾）
                                    }
                                    (true, None) => {
                                        // テープの終わりなら仕方ない☆（＾～＾）手筋は終わりだぜ☆（＾～＾）
                                        app.comm.println(&format!(
                                            "[End of tape of Piece loop: Caret: {}]",
                                            deck.to_human_presentable_of_caret_of_current_tape(
                                                Slot::Training,
                                                &app
                                            ),
                                        ));
                                        break 'sequence_thread;
                                    }
                                    (false, None) => {
                                        // このタッチは実現できなかった☆（＾～＾）手筋はここまで☆（＾～＾）
                                        break 'sequence_thread;
                                    }
                                }
                            } // Sequence thread.

                            // スレッドを差し替えろだぜ☆（＾～＾）
                            app.comm.println("[Next thread scan]");
                            self.change_thread(*subject_piece_id, &app);

                            // 無限ループしないように、１手進めだぜ☆（＾～＾）
                            match deck.seek_1move(Slot::Training, &app) {
                                (true, _, _) => {
                                    break 'sequence_moves;
                                }
                                _ => {}
                            }
                            HumanInterface::bo(deck, &position, &app);
                        } // Sequence moves.

                        // ケツ☆（*＾～＾*） 余ってるかも知れないぜ☆（*＾～＾*）次の手筋探しにチェンジするぜ☆（*＾～＾*）
                        self.change_thread(*subject_piece_id, &app);

                        // 指した手数分、後ろ向きに読み進めながら記録しろだぜ☆（＾～＾）
                        // TODO それを逆順にすれば　指し手だぜ☆（＾～＾）
                        app.comm.println(&format!(
                            "Tried! Then go to opponent {}th note of move! Training deck box: {}. Deck: {}.",
                            forwarding_note_count,
                            deck.to_human_presentable_of_tape_box(Slot::Training),
                            deck.to_human_presentable()
                        ));
                        // TODO ここでテープボックスが無くなっているのは　なぜなのか☆（＾～＾）？
                        deck.look_back_caret_to_opponent(Slot::Training, &app);
                        {
                            deck.go_n_notes_permissive(
                                Slot::Training,
                                forwarding_note_count,
                                position,
                                &app,
                            );
                        }
                        deck.look_back_caret_to_opponent(Slot::Training, &app);
                        HumanInterface::bo(deck, &position, &app);
                        app.comm.println("Backed.");
                    } // if
                } // for

                // いくつか読み取れれば打ち止め。
                if self.get_max_note_len() > 4 {
                    println!("#Break. Exit piece count = {}.", self.get_max_note_len());
                    break 'path_loop;
                }
            } // テープのループ。
        } // トレーニング・ディレクトリー内のループ。

        //println!("#match_thread loop end.");

        // デバッグ表示☆（*＾～＾*）
        {
            for pid in PieceIdentify::iterator() {
                let pid_num = pid.get_number();
                let best_thread = &self.best_thread_map[&pid_num];

                app.comm.println(&format!(
                    "[Best: Pid: {}, Thr-Len: {}. {}]",
                    pid_num,
                    best_thread.len(),
                    best_thread.to_human_presentable(position.get_board_size(), &app)
                ));
            }
        }

        let mut best_move_opt = None;

        // １つチョイスしようぜ☆（*＾～＾*）
        'choice: for pid in PieceIdentify::iterator() {
            let pid_num = pid.get_number();
            let best_thread = &self.best_thread_map[&pid_num];

            // Header.
            // println!("Pid: {}.", pid_num);

            // とりあえず１つチョイス☆（＾～＾）
            if !best_thread.is_empty() {
                best_move_opt = Some(&best_thread.moves[0]);

                // 検索結果を見てみようぜ☆（＾～＾）
                // Operation.
                // println!("  Ope: {} End.", rmove.to_operation_string(position.get_board_size()));

                // Identify.
                // println!("  Num: {} End.", rmove.to_identify_string());
                break 'choice;
            }
        }

        // let best_thread = ThreadsOfPiece {
        //     max_ply: 0,
        //     record: RpmRecord::default(),
        // };

        // 自分の駒ごとの、現局面にマッチする最長の手筋を更新していく。

        if let Some(best_move) = best_move_opt {
            best_move.usi_move
        } else {
            UsiMove::create_resign()
        }
    }

    /// この指し手が、今探している駒の指し手のものであるのか判定。
    pub fn match_subject_piece(
        &mut self,
        subject_piece_id: PieceIdentify,
        my_addr_obj: Address,
        bmove: &BestMove,
        board_size: BoardSize,
        app: &Application,
    ) -> bool {
        if subject_piece_id.get_number() != bmove.subject_pid.get_number()
            || bmove.subject_addr.get_index() != my_addr_obj.get_index() as usize
        {
            /*
            // パターンマッチから外れたら抜けていく。
            app.comm.println(&format!(
                "#[No-subject: これは手筋の主体ではありません。 {} != {} || {} != {}. subject_piece_id: '{}', bmove.subject_pid: '{}', bmove.subject_addr: '{}', my_addr_obj: '{}']",
                subject_piece_id.get_number(),
                bmove.subject_pid.get_number(),
                bmove.subject_addr.get_index(),
                my_addr_obj.get_index() as usize,
                subject_piece_id.to_human_presentable(),
                bmove.subject_pid.to_human_presentable(),
                bmove.subject_addr.to_human_presentable(board_size),
                my_addr_obj.to_human_presentable(board_size)
            ));
            */
            return false;
        }

        // パターンがマッチした。
        app.comm.println(&format!(
                "#[Match-subject: 手筋の主体です。 {} != {} || {} != {}. subject_piece_id: '{}', bmove.subject_pid: '{}', bmove.subject_addr: '{}', my_addr_obj: '{}']",
                subject_piece_id.get_number(),
                bmove.subject_pid.get_number(),
                bmove.subject_addr.get_index(),
                my_addr_obj.get_index() as usize,
                subject_piece_id.to_human_presentable(),
                bmove.subject_pid.to_human_presentable(),
                bmove.subject_addr.to_human_presentable(board_size),
                my_addr_obj.to_human_presentable(board_size)
            ));
        true
    }

    /// この指し手の、取られた駒などが一致しているかの判定。
    pub fn match_object_piece(
        &mut self,
        deck: &mut CassetteDeck,
        slot: Slot,
        position: &mut Position,
        my_addr_obj: Address,
        rmove: &ShogiMove,
        bmove: &BestMove,
        app: &Application,
    ) -> bool {
        // 番地を指定して、そこにある駒が　相手の駒か判定。合法手だけを残す。
        if let Some(addr) = bmove.capture_addr {
            if let Some(cell) = addr.to_cell(position.get_board_size()) {
                if let Some(idp) = position.get_id_piece(cell) {
                    if let Some(_is_opponent) = idp.is_opponent(position) {
                        // 相手の駒を取った合法手。
                    } else {
                        app.comm.println(&format!(
                            "#[Illegal: 味方の駒を取ってしまう。{}]",
                            rmove.to_human_presentable(deck, slot, position.get_board_size(), &app)
                        ));
                        return false;
                    }
                } else {
                    // 現局面では、取ろうとした駒がなかった。
                    app.comm.println(&format!(
                        "#[No-match: 現局面では、取ろうとした駒がなかった。{}]",
                        rmove.to_human_presentable(deck, slot, position.get_board_size(), &app)
                    ));
                    return false;
                }
            } else {
                // プログラムの不具合。
                panic!(
                    "#[IL-盤上以外の駒を取った(1)。{}]",
                    rmove.to_human_presentable(deck, slot, position.get_board_size(), &app)
                );
            }
        } else {
            // 駒を取らなかった合法手。
        };

        // パターンがマッチした。
        app.comm
            .println(&format!("#[Matched: address={}]", my_addr_obj.get_index()));
        true
    }

    /*
    /// 指し手単位での、パターン・マッチ。
    ///
    /// # Returns
    ///
    /// (is_end_of_tape, move_opt)
    pub fn try_read_training_tape_for_1move(
        &mut self,
        deck: &mut CassetteDeck,
        position: &mut Position,
        ply: i16,
        subject_piece_id: PieceIdentify,
        my_addr_obj: Address,
        app: &Application,
    ) -> (bool, Option<ShogiMove>) {
        /*
        comm.println(&format!(
            "#>{} note.",
            note_caret.to_human_presentable()
        ));
        */
        // とりあえず 1手分ごそっと動かそうぜ☆（＾～＾）
        /// 結果は次の４つだぜ☆（＾～＾）
        /// （１）最後の１手分。局面もキャレットも進んでいる。
        /// （２）最後ではない１手分。局面もキャレットも進んでいる。
        /// （３）テープ終わっていた。キャレットを戻す。
        /// （４）実現しない操作だった。局面とキャレットを戻す。
        match GamePlayer::try_read_tape_for_1move(deck, Slot::Training, position, ply, &app) {
            (is_end_of_tape, Some(rmove)) => {
                // テープの通りに、局面をタッチしてみて１手分は　なるほど進んだようだぜ☆（＾～＾）
                // USI に変換してみようぜ☆（＾～＾）
                let bmove =
                    rmove.to_best_move(deck, Slot::Training, position.get_board_size(), &app);

                app.comm.println(&format!(
                    "#{}Rmove:{}. subject('{}'{}){}",
                    deck.to_human_presentable_of_caret_of_current_tape_of_training_box(&app),
                    rmove.to_human_presentable(
                        deck,
                        Slot::Training,
                        position.get_board_size(),
                        &app
                    ),
                    bmove.subject_pid.to_human_presentable(),
                    bmove
                        .subject_addr
                        .to_human_presentable(position.get_board_size()),
                    if let Some(cap_pid) = bmove.capture_pid {
                        format!(
                            " object('{}'{})",
                            cap_pid.to_human_presentable(),
                            bmove
                                .capture_addr
                                .unwrap()
                                .to_human_presentable(position.get_board_size())
                        )
                        .to_string()
                    } else {
                        "".to_string()
                    }
                ));

                if self.position_match(
                    deck,
                    Slot::Training,
                    position,
                    subject_piece_id,
                    my_addr_obj,
                    &rmove,
                    &bmove,
                    &app,
                ) {
                    // 局面と一致。
                    // TODO 現局面で この手を指せるか試してみる。
                    // 例えば 味方の駒の上に駒を動かすような動きは イリーガル・タッチ として弾く。

                    // 新規に テープを作る。ムーブ１つだけ。
                    //let mut recorder = CassetteTapeEditor::new_cassette_tape_editor();
                    //recorder.put_1note(&rmove, comm);
                    //recorder.reset_caret();
                    /*
                    let mut ply_2 = 1;
                    let mut cassette_tape_box_2 = CassetteTapeBox::new_empty(&app);
                    {
                        let mut cassette_tape_2 = CassetteTape::from_1_move(&rmove, &app);
                        cassette_tape_box_2.change_with_tape(cassette_tape_2);
                    }
                    */
                    /*
                    println!(
                        "BMP: This move rtape: {}.",
                        recorder.to_human_presentable(position.get_board_size())
                    );
                     */

                    // TODO 同じことを２回している？
                    /// 結果は次の４つだぜ☆（＾～＾）
                    /// （１）最後の１手分。局面もキャレットも進んでいる。
                    /// （２）最後ではない１手分。局面もキャレットも進んでいる。
                    /// （３）テープ終わっていた。キャレットを戻す。
                    /// （４）実現しない操作だった。局面とキャレットを戻す。
                    match GamePlayer::try_read_tape_for_1move(
                        deck,
                        Slot::Training,
                        position,
                        ply, //ply_2,
                        &app,
                    ) {
                        (true, _) => {
                            // テープの終わり
                            (true, None)
                        }
                        (false, Some(rmove)) => {
                            // 合法タッチ。戻さず抜けます。
                            app.comm.println(&format!(
                                "Hit and go! ({}) {}",
                                bmove.subject_pid.to_human_presentable(),
                                &rmove.to_human_presentable(
                                    deck,
                                    Slot::Training,
                                    position.get_board_size(),
                                    &app
                                )
                            ));
                            HumanInterface::bo_with_tape(
                                deck,
                                Slot::Training,
                                ply, //ply_2,
                                &position,
                                &app,
                            );
                            (false, Some(rmove))
                        }
                        (false, None) => {
                            // 非合法タッチ。（自動で戻されています）
                            app.comm.println(&format!(
                                "Canceled: {}.",
                                rmove.to_human_presentable(
                                    deck,
                                    Slot::Training,
                                    position.get_board_size(),
                                    &app
                                )
                            ));
                            HumanInterface::bo_with_tape(
                                deck,
                                Slot::Training,
                                ply, // ply_2,
                                &position,
                                &app,
                            );
                            (false, None)
                        }
                    }
                } else {
                    // パターン不一致。
                    app.comm.println("[No match.]");
                    (false, None)
                }
            }
            (is_end_of_tape, None) => {
                // パターン不一致。
                app.comm.println("[No match.]");
                (is_end_of_tape, None)
            }
        }
    }
                */
}
