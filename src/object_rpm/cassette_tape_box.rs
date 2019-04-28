extern crate rand;
use application::Application;
use board_size::*;
use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use object_rpm::cassette_tape::*;
use rand::Rng;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

/// .rbox ファイルに対応。
pub struct CassetteTapeBox {
    file: String,
    tapes: Vec<CassetteTape>,
}
impl CassetteTapeBox {
    pub fn new_at_random(app: &Application) -> CassetteTapeBox {
        CassetteTapeBox {
            file: CassetteTapeBox::create_file_full_name(&app.kw29_conf),
            tapes: Vec::new(),
        }
    }

    pub fn new_with_file(file: &str) -> CassetteTapeBox {
        CassetteTapeBox {
            file: file.to_string(),
        }
    }

    /// ランダムにファイル名を付けるぜ☆（*＾～＾*）
    pub fn create_file_full_name(kw29_conf: &KifuwarabeWcsc29Config) -> String {
        let mut rng = rand::thread_rng();
        let rand1: u64 = rng.gen();
        let rand2: u64 = rng.gen();
        let rand3: u64 = rng.gen();
        let rand4: u64 = rng.gen();
        let file = format!("{}-{}-{}-{}.rbox", rand1, rand2, rand3, rand4).to_string();

        Path::new(&kw29_conf.recording)
            .join(file)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn to_box_json(&self, board_size: BoardSize, cassette_tape: &CassetteTape) -> String {
        let content = cassette_tape.to_tape_json(board_size);
        format!("{{\"tape_box\": [{}]}}", content).to_string()
    }

    /// テープ・ボックス単位で書きだすぜ☆（＾～＾）
    pub fn write_cassette_tape_box(
        &self,
        board_size: BoardSize,
        cassette_tape: &CassetteTape,
        comm: &Communication,
    ) {
        comm.println(&format!("#Write tape box to '{}'...", self.file));

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
