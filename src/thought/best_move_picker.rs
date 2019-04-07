use conf::kifuwarabe_wcsc29_config::*;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_record::*;
use thought::knowledge::*;
use usi_conv::usi_move::*;

/// 駒と、手筋のペア。
/// 手筋は複数。
pub struct ThreadsOfPiece {
    identify: PieceIdentify,
    max_ply: i16,
    threads: Vec<RpmRecord>,
}

pub struct BestMovePicker {
}
impl BestMovePicker {
    pub fn new() -> BestMovePicker {
        BestMovePicker {
        }
    }

    /// TODO 学習ファイルをもとに動く。
    pub fn get_best_move(&self, _kw29_config:&KifuwarabeWcsc29Config, position:&Position) -> UsiMove {

        let know = Knowledge::new();

        // TODO 自分の駒（0～40個）の番地を調べる。
        for id in PieceIdentify::iterator() {
            // RPMを検索。
            know.matches(position, id);
        }

        // 自分の駒ごとの、現局面にマッチする最長の手筋を更新していく。

        UsiMove::create_resign()
    }
}