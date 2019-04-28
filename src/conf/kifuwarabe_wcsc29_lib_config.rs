/// # 設定ファイル
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// このライブラリの設定。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KifuwarabeWcsc29LibConfig {
    /// 統一設定ファイルへのパス。
    pub kifuwarabe_wcsc29_config_path: String,
}
impl KifuwarabeWcsc29LibConfig {
    /// 設定ファイル読込。
    pub fn load() -> KifuwarabeWcsc29LibConfig {
        let path = Path::new("./kifuwarabe-wcsc29-lib-config.json");

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
            Err(err) => panic!("Unexpected wcsc29 lib config: {}", err),
        }
    }
}
