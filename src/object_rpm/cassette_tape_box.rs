use board_size::*;
use communication::*;
use object_rpm::cassette_tape::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

/// .rbox ファイルに対応。
pub struct CassetteTapeBox {
    file: String,
}
impl CassetteTapeBox {
    pub fn new_cassette_tape_box(file: &str) -> CassetteTapeBox {
        CassetteTapeBox {
            file: file.to_string(),
        }
    }

    pub fn to_box_json(&self, board_size: BoardSize, cassette_tape: &CassetteTape) -> String {
        let content = cassette_tape.to_tape_json(board_size);
        format!("{{\"tape_box\": [{}]}}", content).to_string()
    }

    /// シートに、カセット・テープを追加書き込みします。
    pub fn write_cassette_tape_box(
        &self,
        board_size: BoardSize,
        cassette_tape: &CassetteTape,
        comm: &Communication,
    ) {
        comm.println(&format!("#Append record to '{}'...", self.file));

        let path = Path::new(&self.file);

        // ディレクトリー作成。
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_x) => {}
                Err(err) => panic!(err),
            }
        } else {
            panic!("Create directory fail. {}", self.file);
        }

        // TODO 追記にしたいんだが、JSONに向いてない☆（＾～＾）しぶしぶ全文上書きで☆（＾～＾）
        let mut file_obj = OpenOptions::new()
            .create(true)
            .write(true)
            // .append(true)
            .open(path)
            .unwrap();

        if let Err(e) = writeln!(file_obj, "{}", self.to_box_json(board_size, cassette_tape)) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // comm.println("#Sheet saved.");
    }
}
