use kifu_rpm::json::rpm_cassette_tape_for_json::*;
//use piece_etc::*;
use serde::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// -rpmrec.json ファイルに対応。
#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmCassetteTapeBoxFile {
    pub book: Vec<RpmCasetteTapeForJson>,
}
impl RpmCassetteTapeBoxFile {
    pub fn new() -> RpmCassetteTapeBoxFile {
        RpmCassetteTapeBoxFile { book: Vec::new() }
    }
    pub fn load(file: &str) -> RpmCassetteTapeBoxFile {
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
                print!("info Unexpected config: {}", err);
                RpmCassetteTapeBoxFile::new()
            }
            // TODO Err(err) => panic!("Unexpected config: {}", err),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
