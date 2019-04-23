use board_size::*;
use communication::*;
use kifu_rpm::rpm_cassette_tape_recorder::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// .rpmove ファイルに対応。
pub struct RpmObjectSheet {
    file_path: String,
}
impl RpmObjectSheet {
    pub fn default(path_text: &str) -> RpmObjectSheet {
        RpmObjectSheet {
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

    /// 物理レコードを追加する。
    pub fn append_record(
        &self,
        _comm: &Communication,
        board_size: BoardSize,
        recorder: &RpmCassetteTapeRecorder,
    ) {
        // comm.println("#Sheet saving...");

        let path = Path::new(&self.file_path);

        // 新規作成、またはレコードを追記。
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        // 末尾にカンマを付けて終わる。
        if let Err(e) = writeln!(
            file,
            "{},",
            recorder.cassette_tape.to_json_object(board_size)
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // comm.println("#Sheet saved.");
    }
}
