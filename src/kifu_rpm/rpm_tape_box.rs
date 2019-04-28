extern crate rand;
use board_size::*;
use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use kifu_rpm::rpm_tape::*;
use object_rpm::cassette_tape::*;
use rand::Rng;
use serde::*;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

/// -rbox.json ファイルに対応。
#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTapeBox {
    pub tape_box: Vec<RpmTape>,
}
impl RpmTapeBox {
    pub fn new() -> Self {
        RpmTapeBox {
            tape_box: Vec::new(),
        }
    }

    pub fn push(&mut self, tape: RpmTape) {
        self.tape_box.push(tape);
    }

    /// JSONファイル読み取り。
    pub fn from_box_file(box_file: &str) -> Self {
        let path = Path::new(box_file);
        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => {
                panic!(err);
            }
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

    /// テープ・ボックス単位で書きだすぜ☆（＾～＾）
    pub fn write(&self, file_name: String, board_size: BoardSize, comm: &Communication) {
        comm.println(&format!("#Write tape box to '{}'...", file_name));

        let path = Path::new(&file_name);

        // ディレクトリー作成。
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_x) => {}
                Err(err) => panic!(err),
            }
        } else {
            panic!("Create directory fail. {}", file_name);
        }

        // 全文上書き☆（＾～＾）
        let mut file_obj = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();

        let json_text = serde_json::to_string(self).unwrap();

        if let Err(e) = writeln!(file_obj, "{}", json_text) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // comm.println("#Sheet saved.");
    }
}
