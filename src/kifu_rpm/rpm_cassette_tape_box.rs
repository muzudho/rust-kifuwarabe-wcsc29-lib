use kifu_rpm::rpm_tape::*;
use serde::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// -rbox.json ファイルに対応。
#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct CassetteTapeBoxForJson {
    pub tape_box: Vec<RpmCasetteTapeForJson>,
}
impl CassetteTapeBoxForJson {
    pub fn new() -> Self {
        CassetteTapeBoxForJson {
            tape_box: Vec::new(),
        }
    }
    pub fn load_tape_box_by_file(file: &str) -> Self {
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
                CassetteTapeBoxForJson::new()
            }
            // TODO Err(err) => panic!("Unexpected config: {}", err),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
