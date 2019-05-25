use conf::kifuwarabe_wcsc29_app_config::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

/// 統一設定ファイル。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KifuwarabeWcsc29MasterConfig {
    pub converter_input: String,
    pub converter_expand: String,
    pub converter_working: String,
    pub converter_output: String,
    pub converter_error: String,
    pub training: String,
    pub learning: String,
    pub book: String,
    pub tapes_fragments: String,
    pub kifuwarabe_wcsc29_exe_path_for_read_kifu: String,
}
impl KifuwarabeWcsc29MasterConfig {
    /// 設定ファイル読込。
    pub fn load(my_app_conf: &KifuwarabeWcsc29AppConfig) -> KifuwarabeWcsc29MasterConfig {
        let kw29_path = &my_app_conf.kifuwarabe_wcsc29_master_config_path;
        let mut kw29_file = match File::open(kw29_path) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err), // ログ取らない。
        };

        let mut contents = String::new();
        match kw29_file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err), // ログ取らない。
        };

        match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => panic!("Unexpected wcsc29 config: {}", err), // ログ取らない。
        }
    }
}
