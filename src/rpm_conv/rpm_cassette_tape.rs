use board_size::*;
use common::caret::*;
use rpm_conv::rpm_tape::*;
use rpm_conv::thread::rpm_note::*;
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
    pub fn default() -> Self {
        RpmCassetteTape {
            caret: Caret::new_next_caret(),
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
    pub fn append_next_cassette_tape(&mut self, cassette_tape_to_empty: &mut RpmCassetteTape) {
        self.tape.append_next_tape(&mut cassette_tape_to_empty.tape);
    }
    pub fn append_back_cassette_tape(&mut self, cassette_tape_to_empty: &mut RpmCassetteTape) {
        self.tape.append_back_tape(&mut cassette_tape_to_empty.tape);
    }

    /// 現在の要素を返してから、キャレットを動かします。
    pub fn get_note_and_go(&mut self) -> Option<RpmNote> {
        self.tape.get_note_and_go(&mut self.caret)
    }
    /// キャレットを動かしてから、現在の要素を返します。
    pub fn cancel_and_get_note(&mut self) -> Option<RpmNote> {
        self.tape.cancel_and_get_note(&mut self.caret)
    }

    /// Human presentable large log.
    pub fn to_human_presentable(&self, board_size: BoardSize) -> String {
        self.tape.to_human_presentable(board_size)
    }

    /// コマンドライン入力形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size: BoardSize) -> (String, String) {
        self.tape.to_sign(board_size)
    }

    /// JSONファイル保存形式。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_json(&self, board_size: BoardSize) -> (String, String) {
        self.tape.to_json(board_size)
    }

    /// JSONのオブジェクト形式。ラベル付き。
    pub fn to_json_object(&self, board_size: BoardSize) -> String {
        let (numbers, operations) = self.to_json(board_size);

        let mut text = "{\n".to_string();
        text = format!("{}    \"header\" : {{\n", text);
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
        text = format!("{}    \"body\" : {{\n", text);
        text = format!("{}        \"operation\" : [\n", text);
        text = format!("{}            {}\n", text, operations);
        text = format!("{}        ],\n", text);
        text = format!("{}        \"piece_number\" : [\n", text);
        text = format!("{}            {}\n", text, numbers);
        text = format!("{}        ]\n", text);
        text = format!("{}    }}\n", text);
        text = format!("{}}}", text);
        text
    }
}
