use serde::Deserialize;

use communication::*;
use position::*;
use rpm_conv::rpm_record::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmRecordHeaderObject {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmRecordBodyObject {
    pub operation: Vec<String>,
    pub piece_number: Vec<i8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmRecordObject {
    pub header: RpmRecordHeaderObject,
    pub body: RpmRecordBodyObject,
}

/// -rpmrec.json ファイルに対応。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmBookFile {
    pub book: Vec<RpmRecordObject>,
}
impl RpmBookFile {
    pub fn new()->RpmBookFile{
        RpmBookFile{
            book: Vec::new(),
        }
    }
    pub fn load(file:&str) -> RpmBookFile {
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
            Err(err) =>
            {
                print!("info File: {:?}", file);
                print!("info Unexpected config: {}", err);
                RpmBookFile::new()
            },
            // TODO Err(err) => panic!("Unexpected config: {}", err),
        }
    }
}