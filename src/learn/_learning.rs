use learn::rpm_learn_file::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;

pub struct Learning {
    rpm_learn_file_vec: Vec<RpmLearnFile>,
}
impl Learning {
    pub fn default() -> Learning {
        let mut instance = Learning {
            rpm_learn_file_vec: Vec::new(),
        };

        for number in 0..40 {
            instance.rpm_learn_file_vec.push(RpmLearnFile::default(number as i16));
        }

        // 玉だけ動きを入れておく。
        instance.rpm_learn_file_vec[0] = RpmLearnFile {
            number: 0,
            line: "51 52".to_string(),
        };
        instance.rpm_learn_file_vec[1] = RpmLearnFile {
            number: 1,
            line: "59 58".to_string(),
        };

        instance
    }

    /// ファイルを読み込む。
    pub fn read(&mut self) {
        for number in 0..40 {
            self.rpm_learn_file_vec[number].read();
        }
    }

    pub fn save(&self) {
        // 学習用のファイルを新規作成、または上書き。
        for number in 0..40 {
            let path = format!("thought/N{:02}.rpm_learn", number);

            let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            // .append(true)
            .open(path)
            .unwrap();

            if let Err(e) = writeln!(file, "{}", self.rpm_learn_file_vec[number].line) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}