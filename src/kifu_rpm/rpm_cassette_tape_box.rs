use board_size::*;
use communication::*;
use kifu_rpm::rpm_cassette_tape::*;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// .rpmove ファイルに対応。
pub struct RpmCassetteTapeBox {
    file_path: String,
}
impl RpmCassetteTapeBox {
    pub fn default(path_text: &str) -> RpmCassetteTapeBox {
        RpmCassetteTapeBox {
            file_path: path_text.to_string(),
        }
    }

    pub fn read_sheet(&mut self) {
        // Path::new(directory).join(file_path_text)
        let path = Path::new(&self.file_path);
        for result in BufReader::new(File::open(path).unwrap()).lines() {
            let line = result.unwrap();
            println!("Read line: `{}`.", line);
            // TODO self.records.push(record);
        }
    }

    /// シートに、カセット・テープを追加します。
    pub fn append_cassette_tape(
        &self,
        comm: &Communication,
        board_size: BoardSize,
        cassette_tape: &RpmCassetteTape,
    ) {
        comm.println(&format!("#Append record to '{}'...", self.file_path));

        let path = Path::new(&self.file_path);

        // ディレクトリー作成。
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_x) => {}
                Err(err) => panic!(err),
            }
        } else {
            panic!("Create directory fail. {}", self.file_path);
        }

        // 新規作成、またはレコードを追記。
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        comm.println(&format!(
            "#Append record: セーブする内容: {}",
            cassette_tape.to_json_object(board_size)
        ));

        // 末尾にカンマを付けて終わる。
        if let Err(e) = writeln!(file, "{},", cassette_tape.to_json_object(board_size)) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // comm.println("#Sheet saved.");
    }
}
