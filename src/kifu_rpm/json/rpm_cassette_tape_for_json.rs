use kifu_rpm::json::rpm_tape_for_json::*;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmCassetteTapeLabelForJson {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmCasetteTapeForJson {
    pub label: RpmCassetteTapeLabelForJson,
    pub tape: RpmTapeForJson,
}
impl RpmCasetteTapeForJson {
    pub fn to_human_presentable(&self) -> String {
        self.tape.to_human_presentable()
    }
}
