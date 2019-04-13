use common_operation::*;
use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_move::*;
use rpm_conv::thread::rpm_operation_note::*;
use rpm_model::rpm_book_file::*;
use std::collections::HashMap;
use std::fs;
use usi_conv::usi_move::*;

/// 駒と、手筋のペア。
/// TODO 手筋は複数。
pub struct ThreadsOfPiece {
    // 一手分。
    pub rpm_move: Option<RpmMove>,
}
impl ThreadsOfPiece {
    pub fn new() -> ThreadsOfPiece {
        ThreadsOfPiece {
            rpm_move: None,
        }
    }

    pub fn len_move(&self) -> usize {
        match &self.rpm_move {
            Some(_x) => 1,
            None => 0,
        }
    }

    pub fn is_empty_move(&self) -> bool {
        match &self.rpm_move {
            Some(_x) => false,
            None => true,
        }
    }
}

pub struct BestMovePicker {
    thread_by_piece_id : HashMap<i8, ThreadsOfPiece>,
}
impl BestMovePicker {
    pub fn default() -> BestMovePicker {
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
            let thread = ThreadsOfPiece::new();
            self.thread_by_piece_id.insert(number, thread);
        }
    }

    pub fn get_len_note(&self, i:i8) -> usize {
        let thread = &self.thread_by_piece_id[&i];
        if thread.is_empty_move() {
            0
        } else {
            thread.len_move()
        }
    }

    /// TODO 学習ファイルをもとに動く。
    pub fn get_best_move(&mut self, comm:&Communication, kw29config:&KifuwarabeWcsc29Config, position:&Position) -> UsiMove {

        // クリアー。
        self.clear();

        // RPMを検索。
        // println!("#get_best_move start. Phase: {:?}", position.get_phase());

        // TODO とりあえず -rpmrec.json ファイルを１個読む。
        'path_loop: for path in fs::read_dir(&kw29config.rpm_record).unwrap() {
            let file = path.unwrap().path().display().to_string();

            // 確認表示。
            {
                use piece_etc::PieceIdentify::*;
                comm.println(&format!("info file: {}, Phase: {:?}.", file, position.get_phase()));
                CommonOperation::show_position(&comm, -1, &position);
                // 先手玉の番地。
                {
                    let (address_opt, _hand) = position.address_of(Some(Phase::First), K00);
                    comm.println(&format!("info First-K00: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
                {
                    let (address_opt, _hand) = position.address_of(Some(Phase::First), K01);
                    comm.println(&format!("info First-K01: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
                // 後手玉の番地。
                {
                    let (address_opt, _hand) = position.address_of(Some(Phase::Second), K00);
                    comm.println(&format!("info Second-K00: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
                {
                    let (address_opt, _hand) = position.address_of(Some(Phase::Second), K01);
                    comm.println(&format!("info Second-K01: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
            }

            let book_file = RpmBookFile::load(&file);
            
            // ファイルの中身をすこし見てみる。
            //comm.println(&format!("file: {}, Book len: {}.", file, book_file.book.len() ));
            if !book_file.book.is_empty() {
                //comm.println(&format!("Ope len: {}, Num len: {}.", book_file.book[0].body.operation.len(), book_file.book[0].body.piece_number.len() ));

                let mut _record_index = -1;

                // レコードがいっぱいある。
                for record_for_json in book_file.book {
                    _record_index += 1;
                    // comm.println(&format!("Record index: {}, Phase: {:?}.", record_index, position.get_phase()));

                    // 駒（0～40個）の番地を全部スキャン。（駒の先後は分からない）
                    'piece_loop: for my_piece_id in PieceIdentify::iterator() {
                        let number = my_piece_id.get_number();

                        // 現局面の駒の番地。
                        let (piece_address_at_cur_pos_opt, _hand) = position.address_of(Some(position.get_phase()), *my_piece_id);
                        if let Some(piece_address_at_cur_pos) = piece_address_at_cur_pos_opt {
                            comm.println(&format!("Phase: {:?}, id: {:?}, number: {}, piece_address_at_cur_pos: {}, hand: {}.", position.get_phase(), my_piece_id, number, piece_address_at_cur_pos, _hand));

                            // 自分の駒番号を検索。
                            let size = record_for_json.body.operation.len();
                            let mut skip_until_phase_change = false;

                            // 駒番号トラックをスキャン。
                            // 動かす駒番号を調べるので、フェーズの変わり目は -1 なので、 -1 の次に現れた駒番号を調べればよい。
                            for row_idx in 0..size {
                                // 駒番号トラックの数。
                                let idtr_num = record_for_json.body.piece_number[row_idx];

                                // スキップ。
                                if skip_until_phase_change {
                                    if idtr_num == -1 {
                                        comm.println(&format!("[{:>3}] --- {:>3} // Stop.", row_idx, idtr_num));
                                        skip_until_phase_change = false;
                                    } else {
                                        comm.println(&format!("[{:>3}] --- {:>3} // Skip.", row_idx, idtr_num));
                                    }

                                    continue;
                                }

                                // 指し手の1文字目以降は読み飛ばす。
                                skip_until_phase_change = true;

                                if my_piece_id.get_number() == idtr_num {
                                    // 一致。

                                    // 番地を検索。
                                    let optr_note = &record_for_json.body.operation[row_idx];
                                    comm.println(&format!("[{:>3}] {} {:>3} // Pick.", row_idx, optr_note, idtr_num));

                                    let ope_note_opt;
                                    {
                                        let mut start = 0;
                                        ope_note_opt = RpmOpeNote::parse_1note(&comm, &optr_note, &mut start, position.get_board_size());
                                    }

                                    if let Some(ope_note) = ope_note_opt {
                                        if let Some(target_address) = ope_note.address {
                                            if target_address.get_index() == piece_address_at_cur_pos as usize {
                                                comm.println(&format!("matched address. address={}.", piece_address_at_cur_pos));
                                                // 一致。
                                                
                                                // １手分読み進める。
                                                if let Some(rmove) = RpmMove::parse_1move(comm, &record_for_json, row_idx, position.get_board_size()) {
                                                    // TODO この手は、現在の盤上で指せるのか検証したい。
                                                    // 例えば 味方の駒の上に駒を動かさないだろうか？

                                                    comm.println(&format!("Rmove: {:?}.", rmove));

                                                    let mut thread = ThreadsOfPiece::new();
                                                    thread.rpm_move = Some(rmove);

                                                    //if self.thread_by_piece_id[&number].max_ply < thread.max_ply {
                                                    // 差し替え。
                                                    self.thread_by_piece_id.insert(number, thread);
                                                    comm.println("Change!");
                                                    //}

                                                    // TODO とりあえず抜ける。
                                                    break 'piece_loop;
                                                } else {
                                                    panic!("Unexpected move in record.")
                                                }
                                            }                                    
                                        }
                                    } else {
                                        // TODO 持ち駒ではないか確認。
                                    }
                                } else {
                                    // 一致しなかったら何もしない。
                                    // No match.
                                    comm.println(&format!("[{:>3}] --- {:>3} // -", row_idx, idtr_num));
                                }
                            }
                        } // piece_address_at_cur_pos
                    }

                    // 手筋の長さが０でない駒の数。
                    let mut count = 0;
                    for pid in PieceIdentify::iterator() {
                        let pid_num = pid.get_number();
                        if 0 < self.get_len_note(pid_num) {
                            count += 1;
                        }
                    }

                    // いくつか読み取れれば打ち止め。
                    if count > 0 {
                        println!("#Break. Exit piece count = {}.", count);
                        break 'path_loop;
                    }

                } // record_loop
            } // book
        } // path_loop

        //println!("#match_thread loop end.");

        let mut best_rpm_move_opt = None;

        // １つチョイスしようぜ☆（*＾～＾*）
        for pid in PieceIdentify::iterator() {
            let pid_num = pid.get_number();
            let thread = &self.thread_by_piece_id[&pid_num];

            // Header.
            // println!("Pid: {}.", pid_num);

            if let Some(rmove) = &thread.rpm_move {
                best_rpm_move_opt = Some(rmove);

                // 検索結果を見てみようぜ☆（＾～＾）
                // Operation.
                // println!("  Ope: {} End.", rmove.to_operation_string(position.get_board_size()));

                // Identify.
                // println!("  Num: {} End.", rmove.to_identify_string());
            }
        }

        // let thread = ThreadsOfPiece {
        //     max_ply: 0,
        //     record: RpmRecord::default(),
        // };


        // 自分の駒ごとの、現局面にマッチする最長の手筋を更新していく。

        if let Some(best_rpm_move) = best_rpm_move_opt {
            best_rpm_move.to_usi_sign(position.get_board_size())
        } else {
            UsiMove::create_resign()
        }
    }
}