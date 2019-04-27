use board_size::*;
use communication::*;
use kifu_rpm::object::rpm_cassette_tape::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

/// .rpmove ファイルに対応。
pub struct RpmCassetteTapeBox {
    file_path: String,
}
impl RpmCassetteTapeBox {
    pub fn new_cassette_tape_box(path_text: &str) -> RpmCassetteTapeBox {
        RpmCassetteTapeBox {
            file_path: path_text.to_string(),
        }
    }

    /// シートに、カセット・テープを追加書き込みします。
    pub fn write_cassette_tape(
        &self,
        board_size: BoardSize,
        cassette_tape: &RpmCassetteTape,
        comm: &Communication,
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
