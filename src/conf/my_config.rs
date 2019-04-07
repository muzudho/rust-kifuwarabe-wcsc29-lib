/// # 設定ファイル
use serde_json::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

/// この実行ファイルの設定。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct MyConfig {
    /// 統一設定ファイルへのパス。
    pub kifuwarabe_wcsc29_config_path: String,
}
impl MyConfig {

    /// 設定ファイル読込。
    pub fn load() -> MyConfig {
        let path = "./config.json";

        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => panic!("Unexpected config: {}", path),
        }
    }
}