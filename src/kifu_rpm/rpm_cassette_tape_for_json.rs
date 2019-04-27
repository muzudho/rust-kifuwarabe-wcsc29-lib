use kifu_rpm::integer_note_vec_for_json::*;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct CassetteTapeLabelForJson {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmCasetteTapeForJson {
    pub label: CassetteTapeLabelForJson,
    pub tape: IntegerNoteVecForJson,
}
impl RpmCasetteTapeForJson {
    pub fn to_human_presentable(&self) -> String {
        self.tape.to_human_presentable()
    }
}
