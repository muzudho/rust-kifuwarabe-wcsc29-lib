extern crate rand;
use audio_compo::cassette_deck::Slot;
use conf::kifuwarabe_wcsc29_master_config::*;
use rand::Rng;
use serde::*;
use sheet_music_format::kifu_rpm::rpm_tape::*;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use studio::application::Application;
use studio::board_size::*;
use video_tape_model::cassette_tape_box::*;

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

    /// JSONファイル読み取り。
    ///
    /// # Arguments
    ///
    /// * `box_file` - ファイル名。存在しないファイルの場合、新規作成。
    pub fn from_box_file(box_file: &str, app: &Application) -> Self {
        // app.comm.println(&format!("Box file name: '{}'.", box_file));

        let path = Path::new(box_file);
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(x) => x,
                    Err(err) => panic!("File open error. {:?}", err),
                };

                // TODO 空っぽのファイルを読み込んでしまって、JSONのパースエラーになってしまうことがある☆（＾～＾）
                // app.comm.println(&format!("Contents: '{}'.", contents));

                match serde_json::from_str(&contents) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!(err);
                    }
                }
            }
            Err(_err) => {
                // 存在しないファイルの場合。
                // ダミーの内容を書いておきたい☆（＾～＾）
                let rpm_tape_box = RpmTapeBox::new();
                rpm_tape_box.write(box_file, &app);

                rpm_tape_box
            }
        }
    }

    pub fn push(&mut self, tape: RpmTape) {
        self.tape_box.push(tape);
    }

    /// ランダムにファイル名を付けるぜ☆（*＾～＾*）
    pub fn create_file_full_name(
        kw29_conf: &KifuwarabeWcsc29MasterConfig,
        app: &Application,
    ) -> String {
        let mut rng = rand::thread_rng();
        let rand1: u64 = rng.gen();
        let rand2: u64 = rng.gen();
        let rand3: u64 = rng.gen();
        let rand4: u64 = rng.gen();
        let file = format!("{}-{}-{}-{}.rbox", rand1, rand2, rand3, rand4).to_string();

        Path::new(&kw29_conf.learning)
            .join(file)
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. conf.learning.")))
            .to_string()
    }

    /// テープ・ボックス単位で書きだすぜ☆（＾～＾）
    pub fn write(&self, file_name: &str, app: &Application) {
        if app.is_debug() {
            app.comm
                .println(&format!("#Write tape box to '{}'...", file_name));
        }

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
            .unwrap_or_else(|err| panic!(app.comm.panic_io(&err)));

        let json_text = serde_json::to_string(self).unwrap();

        if let Err(e) = writeln!(file_obj, "{}", json_text) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // comm.println("#Sheet saved.");
    }

    /// JSONを、オブジェクトに変換します。（トレーニング・テープを想定☆（＾～＾））
    pub fn to_training_object(&self, board_size: BoardSize, app: &Application) -> CassetteTapeBox {
        let mut tape_box = CassetteTapeBox::new_empty(Slot::Training, &app);

        for tape_j in &self.tape_box {
            let tape = tape_j.to_object(board_size, &app);
            tape_box.change_with_tape(tape);
        }

        // カーソルが進んでしまっているので戻すぜ☆（＾～＾）
        tape_box.eject(&app);

        tape_box
    }
}
