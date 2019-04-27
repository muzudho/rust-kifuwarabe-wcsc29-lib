use board_size::*;
use common::caret::*;
use communication::*;
use object_rpm::integer_note_vec::*;
use object_rpm::shogi_move::ShogiMove;
use object_rpm::shogi_note::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::*;

/// 対局情報。
pub struct CassetteTapeLabel {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}
impl CassetteTapeLabel {
    pub fn clear(&mut self) {
        self.date = "".to_string();
        self.event = "".to_string();
        self.player1 = "".to_string();
        self.player2 = "".to_string();
        self.read_file = "".to_string();
    }
}

/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1753122
pub struct CassetteTape {
    pub caret: Caret,
    pub label: CassetteTapeLabel,
    pub tracks: IntegerNoteVec,
}
impl fmt::Display for CassetteTape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Caret: {} {}",
            self.caret.to_human_presentable(),
            self.tracks
        )
    }
}
impl CassetteTape {
    pub fn new_facing_right_cassette_tape() -> Self {
        CassetteTape {
            caret: Caret::new_facing_right_caret(),
            label: CassetteTapeLabel {
                date: "".to_string(),
                event: "".to_string(),
                player1: "".to_string(),
                player2: "".to_string(),
                read_file: "".to_string(),
            },
            tracks: IntegerNoteVec::default(),
        }
    }

    /// 指し手１つから、テープを作るぜ☆（＾～＾）
    pub fn from_1_move(rmove: &ShogiMove) -> Self {
        CassetteTape {
            caret: Caret::new_facing_right_caret(),
            label: CassetteTapeLabel {
                date: "".to_string(),
                event: "".to_string(),
                player1: "".to_string(),
                player2: "".to_string(),
                read_file: "".to_string(),
            },
            tracks: IntegerNoteVec::from_1_move(rmove),
        }
    }

    /// 新品の状態に戻します。
    pub fn clear(&mut self) {
        self.caret.clear_facing_right();
        self.label.clear();
        self.tracks.clear();
    }

    pub fn reset_caret(&mut self) {
        self.caret.reset();
    }

    pub fn get_positive_peak_caret(&self) -> i16 {
        self.tracks.len_positive() as i16
    }
    pub fn get_negative_peak_caret(&self) -> i16 {
        -(self.tracks.len_negative() as i16) - 1
    }

    pub fn is_positive_peak(&self) -> bool {
        self.caret.equals(self.get_positive_peak_caret())
    }
    pub fn is_negative_peak(&self) -> bool {
        self.caret.equals(self.get_negative_peak_caret())
    }

    /// 連結。
    pub fn append_cassette_tape_to_right(&mut self, cassette_tape_to_empty: &mut CassetteTape) {
        self.tracks
            .append_tape_to_right(&mut cassette_tape_to_empty.tracks);
    }
    pub fn append_cassette_tape_to_left(&mut self, cassette_tape_to_empty: &mut CassetteTape) {
        self.tracks
            .append_tape_to_left(&mut cassette_tape_to_empty.tracks);
    }

    /// 現在の要素を返してから、キャレットを動かします。
    pub fn go_1note_forcely(&mut self, comm: &Communication) -> Option<ShogiNote> {
        self.tracks.go_1note_forcely(&mut self.caret, comm)
    }

    /// Human presentable large log.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        format!(
            "{} {}",
            self.caret.to_human_presentable(),
            self.tracks.to_human_presentable(board_size)
        )
        .to_string()
    }

    /// コマンドライン入力形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size: BoardSize) -> (String, String) {
        self.tracks.to_sign(board_size)
    }

    /// テープ・フラグメント単位で書きだすぜ☆（＾～＾）
    pub fn write_cassette_tape_fragment(
        &self,
        file: String,
        board_size: BoardSize,
        comm: &Communication,
    ) {
        comm.println(&format!("#Write tape fragment to '{}'...", file));

        let path = Path::new(&file);

        // ディレクトリー作成。
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_x) => {}
                Err(err) => panic!(err),
            }
        } else {
            panic!("Create directory fail. {}", file);
        }

        // 末尾に カンマ を付けて追記していくぜ☆（＾～＾）
        let mut file_obj = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        if let Err(e) = writeln!(file_obj, "{},", self.to_tape_json(board_size)) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // comm.println("#Sheet saved.");
    }

    /// JSONのオブジェクト形式。テープだけ。
    pub fn to_tape_json(&self, board_size: BoardSize) -> String {
        let (numbers, operations) = self.tracks.to_tracks_json(board_size);

        let mut text = "{\n".to_string();
        text = format!("{}    \"label\" : {{\n", text);
        text = format!("{}        \"date\" : \"{}\",\n", text, self.label.date);
        text = format!("{}        \"event\" : \"{}\",\n", text, self.label.event);
        text = format!(
            "{}        \"player1\" : \"{}\",\n",
            text, self.label.player1
        );
        text = format!(
            "{}        \"player2\" : \"{}\",\n",
            text, self.label.player2
        );
        text = format!(
            "{}        \"read_file\" : \"{}\"\n",
            text, self.label.read_file
        );
        text = format!("{}    }},\n", text);
        text = format!("{}    \"tracks\" : {{\n", text);
        text = format!("{}        \"id\" : [\n", text);
        text = format!("{}            {}\n", text, numbers);
        text = format!("{}        ],\n", text);
        text = format!("{}        \"ope\" : \"{}\"\n", text, operations);
        text = format!("{}    }}\n", text);
        text = format!("{}}}", text);
        text
    }
}
