use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_record::*;
use std::collections::HashMap;
use thought::knowledge::*;
use usi_conv::usi_move::*;

/// 駒と、手筋のペア。
/// TODO 手筋は複数。
pub struct ThreadsOfPiece {
    pub max_ply: i16,
    // pub threads: Vec<RpmRecord>,
    pub record: RpmRecord,
}

pub struct BestMovePicker {
    thread_set : HashMap<i8, ThreadsOfPiece>,
}
impl BestMovePicker {
    pub fn default() -> BestMovePicker {
        let mut instance = BestMovePicker {
            thread_set: HashMap::new(),
        };

        for id in PieceIdentify::iterator() {
            let number = id.get_number();
            let thread = ThreadsOfPiece {
                max_ply: 0,
                record: RpmRecord::default(),
            };
            instance.thread_set.insert(number, thread);
        }

        instance
    }

    /// TODO 学習ファイルをもとに動く。
    pub fn get_best_move(&mut self, comm:&Communication, _kw29_config:&KifuwarabeWcsc29Config, position:&Position) -> UsiMove {

        let know = Knowledge::new();

        // TODO 自分の駒（0～40個）の番地を調べる。
        for id in PieceIdentify::iterator() {
            let number = id.get_number();
            comm.println(&format!("id: {:?}, number: {}.", id, number));

            // RPMを検索。
            let thread = know.match_thread(position, id);

            if self.thread_set[&number].max_ply < thread.max_ply {
                // 差し替え。
                self.thread_set.insert(number, thread);
            }
        }

        // 自分の駒ごとの、現局面にマッチする最長の手筋を更新していく。

        UsiMove::create_resign()
    }
}