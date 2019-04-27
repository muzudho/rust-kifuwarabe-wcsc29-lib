use kifu_rpm::rpm_tape_tracks::RpmTapeTracks;
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
    pub tracks: RpmTapeTracks,
}
impl RpmCasetteTapeForJson {
    pub fn to_human_presentable(&self) -> String {
        self.tracks.to_human_presentable()
    }
}
