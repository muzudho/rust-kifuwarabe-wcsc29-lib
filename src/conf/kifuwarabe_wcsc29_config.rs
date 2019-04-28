use conf::kifuwarabe_wcsc29_lib_config::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct Kw29Directories {
    pub go: String,
    pub went: String,
    pub output: String,
}

/// 統一設定ファイル。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KifuwarabeWcsc29Config {
    pub expansion: Kw29Directories,
    pub formation: Kw29Directories,
    pub eating: Kw29Directories,
    pub training: String,
    pub learning: String,
    pub tape_fragments: String,
    pub kifuwarabe_wcsc29_exe_path_for_read_kifu: String,
}
impl KifuwarabeWcsc29Config {
    /// 設定ファイル読込。
    pub fn load(my_app_conf: &KifuwarabeWcsc29LibConfig) -> KifuwarabeWcsc29Config {
        let kw29_path = &my_app_conf.kifuwarabe_wcsc29_config_path;
        let mut kw29_file = match File::open(kw29_path) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match kw29_file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => panic!("Unexpected wcsc29 config: {}", err),
        }
    }
}
