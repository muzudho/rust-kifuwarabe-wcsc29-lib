extern crate rand;
use application::Application;
use board_size::BoardSize;
use conf::kifuwarabe_wcsc29_config::*;
use object_rpm::cassette_tape::CassetteTape;
use object_rpm::cassette_tape_box::*;
use rand::Rng;
use std::path::Path;

/// カセット・テープ・ボックスが満杯になったら、
/// 次のカセット・テープ・ボックスに変えてくれる☆（*＾～＾*）
pub struct CassetteTapeBoxConveyor {
    /// 書き込み用のテープ・ボックス。
    current_box_for_write: Option<CassetteTapeBox>,

    /// 記録用のカセットテープ。
    pub recording_cassette_tape: CassetteTape,
}
impl CassetteTapeBoxConveyor {
    pub fn new_empty() -> Self {
        CassetteTapeBoxConveyor {
            current_box_for_write: None,
            recording_cassette_tape: CassetteTape::new_facing_right_cassette_tape(),
        }
    }

    pub fn from_cassette_tape(inner_cassette_tape: CassetteTape) -> Self {
        CassetteTapeBoxConveyor {
            current_box_for_write: None,
            recording_cassette_tape: inner_cassette_tape,
        }
    }

    pub fn clear_tape_editor2(&mut self) {
        self.recording_cassette_tape.clear();
    }

    pub fn reset_caret(&mut self) {
        self.recording_cassette_tape.reset_caret();
    }

    pub fn get_mut_recording_cassette_tape(&mut self) -> &mut CassetteTape {
        &mut self.recording_cassette_tape
    }

    /// Human presentable large log.
    pub fn to_recording_cassette_tape_human_presentable(&self, board_size: BoardSize) -> String {
        self.recording_cassette_tape
            .to_human_presentable(board_size)
    }

    /// ランダムにファイル名を付けるぜ☆（*＾～＾*）
    pub fn create_file_name(extension_with_dot: &str) -> String {
        let mut rng = rand::thread_rng();
        let rand1: u64 = rng.gen();
        let rand2: u64 = rng.gen();
        let rand3: u64 = rng.gen();
        let rand4: u64 = rng.gen();
        format!(
            "{}-{}-{}-{}-learning{}",
            rand1, rand2, rand3, rand4, extension_with_dot
        )
        .to_string()
    }

    pub fn create_file_full_path(
        extension_with_dot: &str,
        kw29_conf: &KifuwarabeWcsc29Config,
    ) -> String {
        // ファイルのフル・パスをランダムに作成する。
        Path::new(&kw29_conf.learning)
            .join(CassetteTapeBoxConveyor::create_file_name(
                extension_with_dot,
            ))
            .to_str()
            .unwrap()
            .to_string()
    }

    /// テープボックスを指定するぜ☆（＾～＾）
    pub fn choice_box_manually(&mut self, file: &str) {
        self.current_box_for_write = Some(CassetteTapeBox::new_cassette_tape_box(file));
    }

    /// 空、または満杯なら、新しいテープボックスを作成するぜ☆（＾～＾）
    fn choice_box_automatically(&mut self, kw29_conf: &KifuwarabeWcsc29Config) {
        // パスをランダムに作成する。
        let tape_box_path = Path::new(&kw29_conf.learning)
            .join(CassetteTapeBoxConveyor::create_file_name(".rbox"))
            .to_str()
            .unwrap()
            .to_string();

        // TODO 本当は満杯になるまで使い回したい☆（＾～＾）
        self.current_box_for_write = Some(CassetteTapeBox::new_cassette_tape_box(&tape_box_path));
    }

    /// テープ・フラグメント単位で書き込めるぜ☆（*＾～＾*）
    pub fn write_cassette_tape_fragment(
        &mut self,
        file: String,
        board_size: BoardSize,
        app: &Application,
    ) {
        self.recording_cassette_tape
            .write_cassette_tape_fragment(file, board_size, &app.comm);
    }

    /// テープ・ボックス単位で書き込めるぜ☆（*＾～＾*）
    pub fn write_cassette_tape_box(&mut self, board_size: BoardSize, app: &Application) {
        self.choice_box_automatically(&app.kw29_conf);

        if let Some(box_for_write) = &self.current_box_for_write {
            box_for_write.write_cassette_tape_box(
                board_size,
                &self.recording_cassette_tape,
                &app.comm,
            )
        } else {
            panic!("Get tape box fail.")
        }
    }
}
