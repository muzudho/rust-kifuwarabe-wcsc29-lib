use learn::rpmove_file::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct Learning {
    rpmove_file_vec: Vec<RpmoveFile>,
}
impl Learning {
    pub fn default() -> Learning {
        let mut instance = Learning {
            rpmove_file_vec: Vec::new(),
        };

        for number in 0..40 {
            instance.rpmove_file_vec[number] = RpmoveFile::default();
        }

        // 玉だけ動きを入れておく。
        instance.rpmove_file_vec[0] = RpmoveFile {
            line: "51 52".to_string(),
        };
        instance.rpmove_file_vec[1] = RpmoveFile {
            line: "59 58".to_string(),
        };

        instance
    }

    /// TODO ファイルを読み込む。
    pub fn read(&self) {
    }

    pub fn save(&self) {
        // 学習用のファイルを新規作成、または上書き。
        for number in 0..40 {
            let path = format!("thought/N{:02}.rpmove", number);

            let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            // .append(true)
            .open(path)
            .unwrap();

            if let Err(e) = writeln!(file, "{}", self.rpmove_file_vec[number].line) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}