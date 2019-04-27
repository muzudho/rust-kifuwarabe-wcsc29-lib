use board_size::*;
use common::caret::*;
use communication::*;
use kifu_rpm::object::rpm_tape::*;
use kifu_rpm::thread::rpm_note::*;
use std::*;

/// 対局情報。
pub struct RpmCassetteTapeLabel {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}
impl RpmCassetteTapeLabel {
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
pub struct RpmCassetteTape {
    pub caret: Caret,
    pub label: RpmCassetteTapeLabel,
    pub tape: RpmTape,
}
impl fmt::Display for RpmCassetteTape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Caret: {} {}",
            self.caret.to_human_presentable(),
            self.tape
        )
    }
}
impl RpmCassetteTape {
    pub fn new_facing_right_cassette_tape() -> Self {
        RpmCassetteTape {
            caret: Caret::new_facing_right_caret(),
            label: RpmCassetteTapeLabel {
                date: "".to_string(),
                event: "".to_string(),
                player1: "".to_string(),
                player2: "".to_string(),
                read_file: "".to_string(),
            },
            tape: RpmTape::default(),
        }
    }

    /// 新品の状態に戻します。
    pub fn clear(&mut self) {
        self.caret.clear_facing_right();
        self.label.clear();
        self.tape.clear();
    }

    pub fn reset_caret(&mut self) {
        self.caret.reset();
    }

    pub fn get_positive_peak_caret(&self) -> i16 {
        self.tape.len_positive() as i16
    }
    pub fn get_negative_peak_caret(&self) -> i16 {
        -(self.tape.len_negative() as i16) - 1
    }

    pub fn is_positive_peak(&self) -> bool {
        self.caret.equals(self.get_positive_peak_caret())
    }
    pub fn is_negative_peak(&self) -> bool {
        self.caret.equals(self.get_negative_peak_caret())
    }

    /// 連結。
    pub fn append_cassette_tape_to_right(&mut self, cassette_tape_to_empty: &mut RpmCassetteTape) {
        self.tape
            .append_tape_to_right(&mut cassette_tape_to_empty.tape);
    }
    pub fn append_cassette_tape_to_left(&mut self, cassette_tape_to_empty: &mut RpmCassetteTape) {
        self.tape
            .append_tape_to_left(&mut cassette_tape_to_empty.tape);
    }

    /// 現在の要素を返してから、キャレットを動かします。
    pub fn get_note_and_go_tape(&mut self, comm: &Communication) -> Option<RpmNote> {
        self.tape.get_note_and_go_note(&mut self.caret, comm)
    }

    /// Human presentable large log.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        format!(
            "{} {}",
            self.caret.to_human_presentable(),
            self.tape.to_human_presentable(board_size)
        )
        .to_string()
    }

    /// コマンドライン入力形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size: BoardSize) -> (String, String) {
        self.tape.to_sign(board_size)
    }

    /// JSONのオブジェクト形式。テープだけ。
    pub fn to_tape_json(&self, board_size: BoardSize) -> String {
        let (numbers, operations) = self.tape.to_tape_json(board_size);

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
        text = format!("{}    \"tape\" : {{\n", text);
        text = format!("{}        \"id\" : [\n", text);
        text = format!("{}            {}\n", text, numbers);
        text = format!("{}        ],\n", text);
        text = format!("{}        \"ope\" : [\n", text);
        text = format!("{}            {}\n", text, operations);
        text = format!("{}        ]\n", text);
        text = format!("{}    }}\n", text);
        text = format!("{}}}", text);
        text
    }
}
