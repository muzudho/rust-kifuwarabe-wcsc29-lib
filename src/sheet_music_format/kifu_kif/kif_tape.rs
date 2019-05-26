use sheet_music_format::kifu_kif::kif_move::*;
use sheet_music_format::kifu_kif::version::kaki189::*;
use std::*;
use studio::application::Application;

#[derive(Default)]
pub struct KifTape {
    // 指し手。
    pub moves: Vec<KifMove>,

    // テープ名。
    name: String,

    // 対局日。
    game_date: String,

    // イベント名。
    event: String,

    // 先手名。
    player1: String,

    // 後手名。
    player2: String,
}
impl KifTape {
    pub fn new() -> KifTape {
        KifTape {
            moves: Vec::new(),
            name: String::new(),
            game_date: String::new(),
            event: String::new(),
            player1: String::new(),
            player2: String::new(),
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

    // テープ名。
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    // 対局日。
    pub fn get_game_date(&self) -> String {
        self.game_date.to_string()
    }

    // イベント名。
    pub fn get_event(&self) -> String {
        self.event.to_string()
    }

    // 先手名。
    pub fn get_player1(&self) -> String {
        self.player1.to_string()
    }

    // 後手名。
    pub fn get_player2(&self) -> String {
        self.player2.to_string()
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

    // テープ名を書く。
    pub fn set_name(&mut self, name_text: &str) {
        self.name = name_text.to_string();
    }

    /// 対局日を書く。
    pub fn set_game_date(&mut self, date: &str) {
        self.game_date = date.to_string();
    }

    // イベント名を書く。
    pub fn set_event(&mut self, event_text: &str) {
        self.event = event_text.to_string();
    }

    // 先手名。
    pub fn set_player1(&mut self, player1_text: &str) {
        self.player1 = player1_text.to_string();
    }

    // 後手名。
    pub fn set_player2(&mut self, player2_text: &str) {
        self.player2 = player2_text.to_string();
    }
}
