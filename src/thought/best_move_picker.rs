use common_operation::*;
use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_move::*;
// use rpm_conv::thread::rpm_note_operation::*;
use rpm_for_json::rpm_book_file::*;
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

            /*
            // 確認表示。
            {
                use piece_etc::PieceIdentify::*;
                comm.println(&format!("info file: {}, Phase: {:?}.", file, position.get_phase()));
                CommonOperation::show_position(&comm, -1, &position);
                // 先手玉の番地。
                {
                    let (address_opt, _hand) = position.address_number_of(Some(Phase::First), K00);
                    comm.println(&format!("info First-K00: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
                {
                    let (address_opt, _hand) = position.address_number_of(Some(Phase::First), K01);
                    comm.println(&format!("info First-K01: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
                // 後手玉の番地。
                {
                    let (address_opt, _hand) = position.address_number_of(Some(Phase::Second), K00);
                    comm.println(&format!("info Second-K00: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
                {
                    let (address_opt, _hand) = position.address_number_of(Some(Phase::Second), K01);
                    comm.println(&format!("info Second-K01: {}.", if let Some(address) = address_opt {address.to_string()}else{"".to_string()}));
                }
            }
            */

            let book_file = RpmBookFile::load(&file);
            
            // ファイルの中身をすこし見てみる。
            //comm.println(&format!("file: {}, Book len: {}.", file, book_file.book.len() ));
            if !book_file.book.is_empty() {
                //comm.println(&format!("Ope len: {}, Num len: {}.", book_file.book[0].body.operation.len(), book_file.book[0].body.piece_number.len() ));

                let mut _record_index = -1;

                // レコードがいっぱいある。
                for record_for_json in book_file.book {
                    _record_index += 1;
                    //comm.println(&format!("Record index: {}, Phase: {:?}.", record_index, position.get_phase()));

                    // 駒（0～40個）の番地を全部スキャン。（駒の先後は分からない）
                    'piece_loop: for my_piece_id in PieceIdentify::iterator() {

                        // 現局面の盤上の自駒の番地。
                        // TODO 駒台
                        let (my_piece_addr_num_opt, hand) = position.address_number_of(Some(position.get_phase()), *my_piece_id);
                        if let Some(my_piece_addr_num) = my_piece_addr_num_opt {
                            //comm.println(&format!("My piece on board: Phase: {:?}, id: {:?}, number: {}, my_piece_addr: {}, hand: {}.", position.get_phase(), my_piece_id, my_piece_id.get_number(), my_piece_addr_num, hand));

                            // トラックをスキャン。
                            let mut note_idx = 0;
                            'track_scan: loop {

                                // とりあえず 1手分をパースします。
                                if let Some(rmove) = RpmMove::parse_1move(comm, &record_for_json, &mut note_idx, position.get_board_size()) {
                                    //comm.println("Scanning.");
                                    // どの駒が動いた１手なのか、またその番地。
                                    let (ftp_id, ftp_address) = rmove.to_first_touch_piece_id(position.get_board_size());

                                    // 背番号と、アドレスの一致。
                                    if my_piece_id.get_number() == ftp_id.get_number() &&
                                        ftp_address.get_index() == my_piece_addr_num as usize {
                                        // 一致。

                                        //comm.println(&format!("matched address. address={}.", my_piece_addr_num));
                                        
                                        // TODO この手は、現在の盤上で指せるのか検証したい。
                                        // 例えば 味方の駒の上に駒を動かさないだろうか？

                                        //comm.println(&format!("Rmove: {:?}.", rmove));

                                        let mut thread = ThreadsOfPiece::new();
                                        thread.rpm_move = Some(rmove);

                                        //if self.thread_by_piece_id[&my_piece_id.get_number()].max_ply < thread.max_ply {
                                        // 差し替え。
                                        self.thread_by_piece_id.insert(my_piece_id.get_number(), thread);
                                        //comm.println("Change!");
                                        //}

                                        // TODO とりあえず抜ける。
                                        break 'piece_loop;
                                    } else {
                                        // 一致しなかったら何もしない。
                                        // No match.
                                        //comm.println(&format!("[{:>3}] --- {:>3} // -", note_idx, ftp_id.get_number()));
                                    }
                                } else {
                                    // トラックの終わり。
                                    //comm.println("Break: End of track.");
                                    break 'track_scan;
                                }
                            }
                        } else if hand {
                            // TODO 駒台。
                            //comm.println(&format!("My piece in hand: Phase: {:?}, id: {:?}, number: {}.", position.get_phase(), my_piece_id, my_piece_id.get_number()));

                        } else {
                            // その他。
                            //comm.println(&format!("Not my piece: Phase: {:?}, id: {:?}, number: {}.", position.get_phase(), my_piece_id, my_piece_id.get_number()));
                        }
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
                        //println!("#Break. Exit piece count = {}.", count);
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
            let (umove, _id, _address) = best_rpm_move.to_usi_move(position.get_board_size());
            umove
        } else {
            UsiMove::create_resign()
        }
    }
}