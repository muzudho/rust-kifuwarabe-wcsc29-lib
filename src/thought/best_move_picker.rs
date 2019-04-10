use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use piece_etc::*;
use position::*;
use rpm_conv::thread::rpm_move::*;
use rpm_conv::rpm_operation_note::*;
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

        for id in PieceIdentify::iterator() {
            let number = id.get_number();
            let thread = ThreadsOfPiece::new();
            instance.thread_by_piece_id.insert(number, thread);
        }

        instance
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
        // RPMを検索。
        println!("#match_thread start. Dir: {}", kw29config.rpm_record);

        // TODO とりあえず -rpmrec.json ファイルを１個読む。
        'path_loop: for path in fs::read_dir(&kw29config.rpm_record).unwrap() {
            let file = path.unwrap().path().display().to_string();
            // comm.println(&format!("file: {}", file));
            let book_file = RpmBookFile::load(&file);

            // ファイルの中身をすこし見てみる。
            //comm.println(&format!("file: {}, Book len: {}.", file, book_file.book.len() ));
            if !book_file.book.is_empty() {
                //comm.println(&format!("Ope len: {}, Num len: {}.", book_file.book[0].body.operation.len(), book_file.book[0].body.piece_number.len() ));


                // レコードがいっぱいある。
                for record in book_file.book {

                    // TODO 自分の駒（0～40個）の番地を調べる。
                    'piece_loop: for id in PieceIdentify::iterator() {
                        let number = id.get_number();
                        // 現局面の駒の番地。
                        let (my_address, _hand) = position.address_of(position.get_phase(), id);
                        //comm.println(&format!("id: {:?}, number: {}, my_my_addresscell: {}, hand: {}.", id, number, my_address, hand));


                        // 自分の駒番号を検索。
                        let size = record.body.operation.len();

                        for i in 0..size {
                            let pnum = record.body.piece_number[i];
                            if id.get_number() == pnum {
                                // 番地を検索。
                                let operation_token = &record.body.operation[i];
                                //comm.println(&format!("matched pnum. operation: {}", operation_token));

                                let ope_note_opt;
                                {
                                    let mut start = 0;
                                    ope_note_opt = RpmOpeNote::parse_1note(&comm, &operation_token, &mut start, &position.get_board_size());
                                }

                                if let Some(ope_note) = ope_note_opt {
                                    if let Some(target_address) = ope_note.address {
                                        if target_address.get_index() == my_address as usize {
                                            //comm.println("matched address.");
                                            // 一致。
                                            
                                            let mut thread = ThreadsOfPiece::new();
                                            let mut rmove = RpmMove::new();
                                            {
                                                // TODO とりあえず　次のターンチェンジまで読み進める。
                                                'j_loop: for j in i..size {
                                                    let j_ope_token = &record.body.operation[j];

                                                    let j_ope_note_opt;
                                                    {
                                                        let mut start = 0;
                                                        j_ope_note_opt = RpmOpeNote::parse_1note(&comm, &j_ope_token, &mut start, &position.get_board_size());
                                                    }

                                                    if let Some(j_ope_note) = j_ope_note_opt {
                                                        if j_ope_note.is_phase_change() {
                                                            break 'j_loop;
                                                        }
                                                    }

                                                    rmove.operation_notes.push(j_ope_token.to_string());
                                                    let j_num = &record.body.piece_number[j];
                                                    rmove.piece_number_notes.push(*j_num);
                                                }
                                            }
                                            thread.rpm_move = Some(rmove);

                                            //if self.thread_by_piece_id[&number].max_ply < thread.max_ply {
                                            // 差し替え。
                                            self.thread_by_piece_id.insert(number, thread);
                                            //comm.println("Change!");
                                            //}

                                            // TODO とりあえず抜ける。
                                            break 'piece_loop;
                                        }                                    
                                    }
                                } else {
                                    // TODO 持ち駒ではないか確認。
                                }
                            }
                        }

                        // if self.thread_by_piece_id[&number].max_ply < thread.max_ply {
                        //     // 差し替え。
                        //     self.thread_by_piece_id.insert(number, thread);
                        // }
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
                    if count > 6 {
                        //println!("#Break. Exit piece count = {}.", count);
                        break 'path_loop;
                    }

                } // record_loop
            } // book
        } // path_loop

        println!("#match_thread loop end.");

        // 検索結果を見てみようぜ☆（＾～＾）
        for pid in PieceIdentify::iterator() {
            let pid_num = pid.get_number();
            let thread = &self.thread_by_piece_id[&pid_num];

            // Header.
            println!("Pid: {}.", pid_num);

            if let Some(rmove) = &thread.rpm_move {
                // Operation.
                print!("  Ope: ");
                for i in 0..rmove.len_note() {
                    let ope = &rmove.operation_notes[i];
                    print!("{} ", ope);
                }
                println!(" End.");

                // Identify.
                print!("  Num: ");
                for i in 0..rmove.len_note() {
                    let num = &rmove.piece_number_notes[i];
                    print!("{} ", num);
                }
                println!(" End.");
            }
        }

        // let thread = ThreadsOfPiece {
        //     max_ply: 0,
        //     record: RpmRecord::default(),
        // };


        // 自分の駒ごとの、現局面にマッチする最長の手筋を更新していく。

        UsiMove::create_resign()
    }
}