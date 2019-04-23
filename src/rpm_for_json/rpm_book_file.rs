use piece_etc::*;
use serde::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmRecordHeaderObject {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmRecordBodyObject {
    pub piece_number: Vec<i8>,
    pub operation: Vec<String>,
}
impl RpmRecordBodyObject {
    pub fn to_human_presentable(&self) -> String {
        let mut text = String::new();

        for i in 0..self.operation.len() {
            text = format!(
                "{} '{}'{}",
                text,
                if let Some(pid) = PieceIdentify::from_number(self.piece_number[i]) {
                    pid.to_human_presentable()
                } else {
                    "|".to_string()
                },
                self.operation[i]
            )
            .to_string();
        }

        text.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmRecordForJson {
    pub header: RpmRecordHeaderObject,
    pub body: RpmRecordBodyObject,
}
impl RpmRecordForJson {
    pub fn to_human_presentable(&self) -> String {
        self.body.to_human_presentable()
    }
}

/// -rpmrec.json ファイルに対応。
#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmBookFile {
    pub book: Vec<RpmRecordForJson>,
}
impl RpmBookFile {
    pub fn new() -> RpmBookFile {
        RpmBookFile { book: Vec::new() }
    }
    pub fn load(file: &str) -> RpmBookFile {
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
                RpmBookFile::new()
            }
            // TODO Err(err) => panic!("Unexpected config: {}", err),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
