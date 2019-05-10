use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

#[derive(Default)]
pub struct Communication {
    /// logger, logging, log file.
    pub log_file_name: String,

    /// 起動直後、コンソールにデバッグ用文字列を出力するのは　プロトコル違反になるので、最初は 無効にしてある☆（＾～＾）
    pub enabled_standard_output: bool,
}
impl Communication {
    /// # Arguments
    ///
    /// * `file_name` - ログ・ファイル名。設定ファイルなどから読み込んで指定しておけだぜ☆（*＾～＾*）
    pub fn from_file(logging_file_name: &str) -> Communication {
        Communication {
            log_file_name: logging_file_name.to_string(),
            enabled_standard_output: false,
        }
    }

    /// これで標準出力への出力を可能にすること。
    pub fn activate_standard_output(&mut self, value: bool) {
        self.enabled_standard_output = value;
    }

    /// Write.
    pub fn print(&self, line: &str) {
        if self.enabled_standard_output {
            print!("{}", line);
        }

        self.log(line);
    }

    pub fn log(&self, line: &str) {
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
        if self.enabled_standard_output {
            println!("{}", line);
        }

        self.logln(line);
    }

    pub fn logln(&self, line: &str) {
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
        if self.enabled_standard_output {
            self.println(msg); // コンソールに表示して、ログも取る。
        }
        String::from(msg)
    }

    /// 入出力エラーの時、 panic! で包んで使う。
    pub fn panic_io(&self, err: &std::io::Error) -> String {
        let msg = format!("{}", err);
        if self.enabled_standard_output {
            self.println(&msg);
        }
        msg
    }
}
