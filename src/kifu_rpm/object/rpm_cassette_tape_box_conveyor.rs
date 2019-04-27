extern crate rand;
use board_size::BoardSize;
use communication::Communication;
use conf::kifuwarabe_wcsc29_config::*;
use kifu_rpm::object::rpm_cassette_tape::RpmCassetteTape;
use kifu_rpm::object::rpm_cassette_tape_box::*;
use rand::Rng;
use std::fs;
use std::path::Path;

/// カセット・テープ・ボックスが満杯になったら、
/// 次のカセット・テープ・ボックスに変えてくれる☆（*＾～＾*）
pub struct RpmCassetteTapeBoxConveyor {
    current_tape_box: Option<RpmCassetteTapeBox>,
}
impl RpmCassetteTapeBoxConveyor {
    pub fn new_empty() -> Self {
        RpmCassetteTapeBoxConveyor {
            current_tape_box: None,
        }
    }

    /// ランダムにファイル名を付けるぜ☆（*＾～＾*）
    pub fn create_file_name() -> String {
        let mut rng = rand::thread_rng();
        let rand1: u64 = rng.gen();
        let rand2: u64 = rng.gen();
        let rand3: u64 = rng.gen();
        let rand4: u64 = rng.gen();
        format!("{}-{}-{}-{}-learning.rpmove", rand1, rand2, rand3, rand4).to_string()
    }

    /// テープボックスを指定するぜ☆（＾～＾）
    pub fn choice_box_manually(&mut self, file: &str) {
        // Prepare parent directory.
        let file_path = Path::new(file);
        let parent_dir = file_path.parent().unwrap();
        match fs::create_dir_all(parent_dir) {
            Ok(_x) => {}
            Err(err) => panic!("Directory create fail: {}", err),
        }
    }

    /// 空、または満杯なら、新しいテープボックスを作成するぜ☆（＾～＾）
    fn choice_box_automatically(&mut self, kw29_conf: &KifuwarabeWcsc29Config) {
        // パスをランダムに作成する。
        let tape_box_path = Path::new(&kw29_conf.learning)
            .join(RpmCassetteTapeBoxConveyor::create_file_name())
            .to_str()
            .unwrap()
            .to_string();

        // TODO 本当は満杯になるまで使い回したい☆（＾～＾）
        self.current_tape_box = Some(RpmCassetteTapeBox::new_cassette_tape_box(&tape_box_path));
    }

    /// まだ書き込めるテープ・ボックスを適当に返すぜ☆（*＾～＾*）
    pub fn write_cassette_tape(
        &mut self,
        kw29_conf: &KifuwarabeWcsc29Config,
        board_size: BoardSize,
        cassette_tape: &RpmCassetteTape,
        comm: &Communication,
    ) {
        self.choice_box_automatically(&kw29_conf);

        if let Some(tape_box) = &self.current_tape_box {
            tape_box.write_cassette_tape(board_size, cassette_tape, comm)
        } else {
            panic!("Get tape box fail.")
        }
    }
}
