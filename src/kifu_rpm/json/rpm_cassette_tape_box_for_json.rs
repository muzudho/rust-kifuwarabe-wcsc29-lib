use kifu_rpm::json::rpm_cassette_tape_for_json::*;
//use piece_etc::*;
use serde::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// -rbox.json ファイルに対応。
#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmCassetteTapeBoxForJson {
    pub tape_box: Vec<RpmCasetteTapeForJson>,
}
impl RpmCassetteTapeBoxForJson {
    pub fn new() -> Self {
        RpmCassetteTapeBoxForJson {
            tape_box: Vec::new(),
        }
    }
    pub fn load_file(file: &str) -> Self {
        // JSONファイル。
        let path = Path::new(file);
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
            Err(err) => {
                print!("info File: {:?}", file);
                print!("info Unexpected cassette tape box: {}", err);
                RpmCassetteTapeBoxForJson::new()
            }
            // TODO Err(err) => panic!("Unexpected config: {}", err),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
