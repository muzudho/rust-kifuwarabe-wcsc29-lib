use serde::*;

/// 汎用テープ・ラベル。
/// 元データをそのまま抜き出し、ぴったりした項目がなければ、どこかに入れる。
/// 内容の順序は、
/// * 一意性
///   * テープ名
///   * 時
///     対局年月日、開始時刻、終了時刻。
///   * 場所
///   * 催し物名
///   * 先手
///   * 後手
/// * ルール
///   * 持ち時間制
///   * ハンディキャップ
/// * ゲーム内容分析情報
///   * 戦型
/// * ツール付加情報
///   * 棋譜の保存形式
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct TapeLabel {
    // テープ名。一意性があると良い。
    name: String,

    // 対局日。
    game_date: String,

    // 開始時刻。
    start_time: String,

    // 終了時刻。
    end_time: String,

    // 対局場。
    place: String,

    // イベント名。
    event: String,

    // 先手名。
    player1: String,

    // 後手名。
    player2: String,

    // 持ち時間制。
    time_system: String,

    // ハンディキャップ。
    handicap: String,

    // 戦型
    battle_type: String,

    // 棋譜の保存形式。
    format: String,
}
impl TapeLabel {
    pub fn new() -> Self {
        Self {
            format: String::new(),
            name: String::new(),
            game_date: String::new(),
            start_time: String::new(),
            end_time: String::new(),
            place: String::new(),
            event: String::new(),
            player1: String::new(),
            player2: String::new(),
            time_system: String::new(),
            handicap: String::new(),
            battle_type: String::new(),
        }
    }

    // #####
    // # G #
    // #####

    /// 棋譜の保存形式。
    pub fn get_format(&self) -> String {
        self.format.to_string()
    }

    /// テープ名。
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// 対局日。
    pub fn get_game_date(&self) -> String {
        self.game_date.to_string()
    }

    /// 開始時間。
    pub fn get_start_time(&self) -> String {
        self.start_time.to_string()
    }

    /// 終了時間。
    pub fn get_end_time(&self) -> String {
        self.end_time.to_string()
    }

    /// 対局場。
    pub fn get_place(&self) -> String {
        self.place.to_string()
    }

    /// イベント名。
    pub fn get_event(&self) -> String {
        self.event.to_string()
    }

    /// 先手名。
    pub fn get_player1(&self) -> String {
        self.player1.to_string()
    }

    /// 後手名。
    pub fn get_player2(&self) -> String {
        self.player2.to_string()
    }

    /// 持ち時間制。
    pub fn get_time_system(&self) -> String {
        self.time_system.to_string()
    }

    /// ハンディキャップ。
    pub fn get_handicap(&self) -> String {
        self.handicap.to_string()
    }

    /// 戦型。
    pub fn get_battle_type(&self) -> String {
        self.battle_type.to_string()
    }

    // #####
    // # S #
    // #####

    // 棋譜の保存形式を書く。
    pub fn set_format(&mut self, format_text: &str) {
        self.format = format_text.to_string();
    }

    // テープ名を書く。
    pub fn set_name(&mut self, name_text: &str) {
        self.name = name_text.to_string();
    }

    /// 対局日を書く。
    pub fn set_game_date(&mut self, date: &str) {
        self.game_date = date.to_string();
    }

    /// 開始時間を書く。
    pub fn set_start_time(&mut self, time: &str) {
        self.start_time = time.to_string();
    }

    /// 終了時間を書く。
    pub fn set_end_time(&mut self, time: &str) {
        self.end_time = time.to_string();
    }

    /// 対局場を書く。
    pub fn set_place(&mut self, date: &str) {
        self.place = date.to_string();
    }

    // イベント名を書く。
    pub fn set_event(&mut self, event_text: &str) {
        self.event = event_text.to_string();
    }

    // 先手名を書く。
    pub fn set_player1(&mut self, player1_text: &str) {
        self.player1 = player1_text.to_string();
    }

    // 後手名を書く。
    pub fn set_player2(&mut self, player2_text: &str) {
        self.player2 = player2_text.to_string();
    }

    // 持ち時間制を書く。
    pub fn set_time_system(&mut self, time_system_text: &str) {
        self.time_system = time_system_text.to_string();
    }

    // ハンディキャップを書く。
    pub fn set_handicap(&mut self, handicap_text: &str) {
        self.handicap = handicap_text.to_string();
    }

    // 戦型を書く。
    pub fn set_battle_type(&mut self, battle_type_text: &str) {
        self.battle_type = battle_type_text.to_string();
    }
}
