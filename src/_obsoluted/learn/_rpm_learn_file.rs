use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;

pub struct RpmLearnFile {
    pub number: i16,
    pub line: String,
}
impl RpmLearnFile {
    pub fn default(piece_number:i16) -> RpmLearnFile {
        RpmLearnFile {
            number: piece_number,
            line: String::new(),
        }
    }

    /// TODO ファイルを読み込む。
    pub fn read(&mut self) {
        let path = format!("musician/N{:02}.rpm_learn", self.number);
        let mut file = match File::open(path) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        self.line = contents;
    }
}