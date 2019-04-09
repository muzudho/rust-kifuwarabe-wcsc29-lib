use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_record::*;
use rpm_model::rpm_book_file::*;
use std::collections::HashMap;
use std::fs;
use thought::knowledge::*;
use usi_conv::usi_move::*;

/// 駒と、手筋のペア。
/// TODO 手筋は複数。
pub struct ThreadsOfPiece {
    pub operation_notes: Vec<String>,
    pub piece_number_notes: Vec<i8>,
}
impl ThreadsOfPiece {
    pub fn new() -> ThreadsOfPiece {
        ThreadsOfPiece {
            operation_notes: Vec::new(),
            piece_number_notes: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.operation_notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.operation_notes.is_empty()
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

    pub fn get_len(&self, i:i8) -> usize {
        let thread = &self.thread_by_piece_id[&i];
        thread.len()
    }

    /// TODO 学習ファイルをもとに動く。
    pub fn get_best_move(&mut self, comm:&Communication, kw29config:&KifuwarabeWcsc29Config, position:&Position) -> UsiMove {

        let know = Knowledge::new();


        // RPMを検索。
        println!("#match_thread start. Dir: {}", kw29config.rpm_record);

        // TODO とりあえず -rpmrec.json ファイルを１個読む。
        for path in fs::read_dir(&kw29config.rpm_record).unwrap() {
            let file = path.unwrap().path().display().to_string();
            // comm.println(&format!("file: {}", file));
            let book_file = RpmBookFile::load(&file);

            // ファイルの中身をすこし見てみる。
            comm.println(&format!("file: {}, Book len: {}.", file, book_file.book.len() ));
            if !book_file.book.is_empty() {
                comm.println(&format!("Ope len: {}, Num len: {}.", book_file.book[0].body.operation.len(), book_file.book[0].body.piece_number.len() ));


                // レコードがいっぱいある。
                'outer: for record in book_file.book {

                    // TODO 自分の駒（0～40個）の番地を調べる。
                    for id in PieceIdentify::iterator() {
                        let number = id.get_number();
                        // 現局面の駒の番地。
                        let (board_index, hand) = position.board_index_of(position.get_phase(), id);
                        comm.println(&format!("id: {:?}, number: {}, board_index: {}, hand: {}.", id, number, board_index, hand));


                        // 自分の駒番号を検索。
                        let size = record.body.operation.len();
                        for i in 0..size {
                            let num = record.body.piece_number[i];
                            if id.get_number() == num {
                                // 番地を検索。
                                let operation = &record.body.operation[i];

                                match operation.parse::<i8>() {
                                    Ok(target_board_index) => {
                                        if target_board_index > 0 && target_board_index == board_index {
                                            // 一致。とりあえず　ここから 数ノートを選んでおく。

                                            let mut thread = ThreadsOfPiece::new();
                                            for j in i..size {
                                                let j_ope = &record.body.operation[j];
                                                thread.operation_notes.push(j_ope.to_string());

                                                let j_num = &record.body.piece_number[j];
                                                thread.piece_number_notes.push(*j_num);
                                            }

                                            //if self.thread_by_piece_id[&number].max_ply < thread.max_ply {
                                            // 差し替え。
                                            self.thread_by_piece_id.insert(number, thread);
                                            //}

                                            // TODO とりあえず抜ける。
                                            break 'outer;
                                        }
                                    },
                                    Err(_e) => {
                                        // TODO 持ち駒ではないか確認。
                                    },
                                }
                            }
                        }

                        // if self.thread_by_piece_id[&number].max_ply < thread.max_ply {
                        //     // 差し替え。
                        //     self.thread_by_piece_id.insert(number, thread);
                        // }
                    }

                } // record

            }
        }

        // 手筋の長さが０でない駒の数。
        let mut count = 0;
        for pid in PieceIdentify::iterator() {
            let pid_num = pid.get_number();
            if 0 < self.get_len(pid_num) {
                count += 1;
            }
        }

        println!("#match_thread loop end. exist piece count = {}.", count);

        // let thread = ThreadsOfPiece {
        //     max_ply: 0,
        //     record: RpmRecord::default(),
        // };


        // 自分の駒ごとの、現局面にマッチする最長の手筋を更新していく。

        UsiMove::create_resign()
    }
}