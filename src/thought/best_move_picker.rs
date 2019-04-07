use position::*;
use rpm_conv::rpm_operation_track::*;
use usi_conv::usi_move::*;
use conf::kifuwarabe_wcsc29_config::*;

pub struct BestMovePicker {
}
impl BestMovePicker {
    pub fn new() -> BestMovePicker {
        BestMovePicker {
        }
    }

    /// TODO 学習ファイルをもとに動く。
    pub fn get_best_move(&self, kw29_config:&KifuwarabeWcsc29Config, _position:&Position) -> UsiMove {
        UsiMove::create_resign()
    }
}