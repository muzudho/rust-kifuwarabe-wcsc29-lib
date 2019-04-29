use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Default)]
pub struct Communication {
    pub log_file_name: String,
}
impl Communication {
    /// # Arguments
    ///
    /// * `file_name` - ログ・ファイル名。設定ファイルなどから読み込んで指定しておけだぜ☆（*＾～＾*）
    pub fn from_file(logging_file_name: &str) -> Communication {
        Communication {
            log_file_name: logging_file_name.to_string(),
        }
    }

    /// Write.
    pub fn print(&self, line: &str) {
        println!("{}", line);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file_name)
            .unwrap();

        if let Err(e) = write!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    /// Write line.
    pub fn println(&self, line: &str) {
        println!("{}", line);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file_name)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
