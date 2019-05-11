/// # 設定ファイル
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct Logging {
    // ログ・ファイルを入れておくディレクトリー。
    pub directory: String,
    pub file_base_name: String,
    pub file_extension: String,
}

/// このライブラリの設定。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KifuwarabeWcsc29AppConfig {
    // 統一設定ファイルへのパス。
    pub kifuwarabe_wcsc29_master_config_path: String,

    // ログ・ファイル。
    pub logging: Logging,
}
impl KifuwarabeWcsc29AppConfig {
    /// 設定ファイル読込。
    pub fn load() -> KifuwarabeWcsc29AppConfig {
        let path = Path::new("./kifuwarabe-wcsc29-app-config.json");

        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err), // ログ取らない。
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err), // ログ取らない。
        };

        match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => panic!("Unexpected wcsc29 lib config: {}", err), // ログ取らない。
        }
    }
}
