/// # 設定ファイル
// use serde_json::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use conf::kifuwarabe_wcsc29_lib_config::*;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct Kw29Directories {
	pub go : String,
	pub went : String,
	pub output : String,
}

/// 統一設定ファイル。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KifuwarabeWcsc29Config {
	pub expansion : Kw29Directories,
	pub formation : Kw29Directories,
	pub eating : Kw29Directories,
    pub learning : String,
    pub rpm_record : String,
}
impl KifuwarabeWcsc29Config {

    /// 設定ファイル読込。
    pub fn load(my_confing:&KifuwarabeWcsc29LibConfig) -> KifuwarabeWcsc29Config {
        let path = &my_confing.kifuwarabe_wcsc29_config_path;
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
            Err(err) => panic!("Unexpected config: {}", err),
        }
    }
}