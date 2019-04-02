/// # 設定ファイル
///
/// 参考: [シリアライズ、デシリアライズ](https://github.com/serde-rs/json)
extern crate serde_json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;

pub struct Config {
    /// 自分の戦いを記録するディレクトリ。
    my_record_directory: String,

    /// 参考にした CSA形式棋譜の置いてあるディレクトリ。
    reference_csa_record_directory: String,

    /// 参考にした USI形式棋譜の置いてあるディレクトリ。
    reference_usi_record_directory: String,
}
impl Config {
    pub fn get_my_record_directory(self) -> String {
        self.my_record_directory
    }

    /// 設定ファイル読込。
    pub fn load() -> Config {
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

        // https://docs.serde.rs/serde_json/value/enum.Value.html
        let document: Value = match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        Config {
            my_record_directory: if let Some(x) = document["my_record_directory"].as_str() {
                x.to_string()
            } else {
                "".to_string()
            },

            reference_csa_record_directory: if let Some(x) = document["my_rereference_csa_record_directorycord_directory"].as_str() {
                x.to_string()
            } else {
                "".to_string()
            },

            reference_usi_record_directory: if let Some(x) = document["my_rereference_usi_record_directorycord_directory"].as_str() {
                x.to_string()
            } else {
                "".to_string()
            },
        }
    }
}