use learn::rpmove_file::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct Learning {
    rpmove_files: [Option<RpmoveFile>; 40],
}
impl Learning {
    pub fn new() -> Learning {
        let mut instance = Learning {
            rpmove_files: [
                None, None, None, None, None, None, None, None, None, None, 
                None, None, None, None, None, None, None, None, None, None, 
                None, None, None, None, None, None, None, None, None, None, 
                None, None, None, None, None, None, None, None, None, None, 
            ]
        };

        for number in 0..40 {
            instance.rpmove_files[number] = Some(RpmoveFile::new());
        }

        // 玉だけ動きを入れておく。
        instance.rpmove_files[0] = Some(RpmoveFile {
            line: "51 52".to_string(),
        });
        instance.rpmove_files[1] = Some(RpmoveFile {
            line: "59 58".to_string(),
        });

        instance
    }

    pub fn read(&self) {

    }

    pub fn save(&self) {
        // 学習用のファイルを新規作成、または上書き。
        for number in 0..40 {
            let path = format!("thought/N{:02}.rpmove", number);

            if let Some(ref rpmove_file) = self.rpmove_files[number] {
                let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                // .append(true)
                .open(path)
                .unwrap();

                if let Err(e) = writeln!(file, "{}", rpmove_file.line) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
    }
}