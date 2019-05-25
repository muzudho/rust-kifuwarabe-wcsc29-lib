use conf::kifuwarabe_wcsc29_exe_config::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

/// 統一設定ファイル。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KifuwarabeWcsc29ConverterMasterConfig {
    pub converter_input: String,
    pub converter_expanded: String,
    pub converter_encoded: String,
    pub converter_converted: String,
    pub converter_jammed: String,
    pub converter_error: String,
    pub kifuwarabe_wcsc29_exe_path_for_read_kifu: String,
}
impl KifuwarabeWcsc29ConverterMasterConfig {
    /// 設定ファイル読込。
    pub fn load(my_app_conf: &KifuwarabeWcsc29ExeConfig) -> KifuwarabeWcsc29ConverterMasterConfig {
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
