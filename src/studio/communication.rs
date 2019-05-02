use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

#[derive(Default)]
pub struct Communication {
    /// logger, logging, log file.
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
        print!("{}", line);

        // ディレクトリー作成。
        if let Some(parent) = Path::new(&self.log_file_name).parent() {
            match fs::create_dir_all(parent) {
                Ok(_x) => {}
                Err(err) => panic!(err),
            }
        } else {
            panic!("Create directory fail. {}", self.log_file_name);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file_name)
            .unwrap_or_else(|f| panic!(f)); // このエラーは、無限ループしてしまうので、ログには残さない。

        if let Err(e) = write!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    /// Write line.
    pub fn println(&self, line: &str) {
        println!("{}", line);

        // ディレクトリー作成。
        if let Some(parent) = Path::new(&self.log_file_name).parent() {
            match fs::create_dir_all(parent) {
                Ok(_x) => {}
                Err(err) => panic!(err),
            }
        } else {
            panic!("Create directory fail. {}", self.log_file_name);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file_name)
            .unwrap_or_else(|f| panic!(f)); // このエラーは、無限ループしてしまうので、ログには残さない。

        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    /// panic! で包んで使う。
    pub fn panic(&self, msg: &str) -> String {
        self.println(msg); // コンソールに表示して、ログも取る。
        String::from(msg)
    }

    /// 入出力エラーの時、 panic! で包んで使う。
    pub fn panic_io(&self, err: &std::io::Error) -> String {
        let msg = format!("{}", err);
        self.println(&msg);
        String::from(msg)
    }
}
