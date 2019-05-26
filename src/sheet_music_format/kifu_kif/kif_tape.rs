use sheet_music_format::kifu_kif::kif_move::*;
use sheet_music_format::kifu_kif::version::kaki189::*;
use std::*;
use studio::application::Application;

#[derive(Default)]
pub struct KifTape {
    pub moves: Vec<KifMove>,
    game_date: String,
}
impl KifTape {
    pub fn new() -> KifTape {
        KifTape {
            moves: Vec::new(),
            game_date: String::new(),
        }
    }

    // #####
    // # F #
    // #####

    /// ファイル読取。
    pub fn from_file(file: &str, app: &Application) -> KifTape {
        // .kif形式には、バージョンがいろいろあるようだ。
        Kaki189::from_file(file, &app)
    }

    // #####
    // # G #
    // #####

    // 対局日。
    pub fn get_game_date(&self) -> String {
        self.game_date.to_string()
    }

    // #####
    // # P #
    // #####

    /// 指し手を追加。
    pub fn push_move(&mut self, mov: KifMove) {
        self.moves.push(mov);
    }

    // #####
    // # S #
    // #####

    /// 対局日を記述。
    pub fn set_game_date(&mut self, date: &str) {
        self.game_date = date.to_string();
    }
}
