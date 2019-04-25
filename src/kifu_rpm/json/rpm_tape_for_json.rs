use piece_etc::*;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTapeForJson {
    pub id: Vec<i8>,
    pub ope: Vec<String>,
}
impl RpmTapeForJson {
    pub fn to_human_presentable(&self) -> String {
        let mut text = String::new();

        for i in 0..self.ope.len() {
            text = format!(
                "{} '{}'{}",
                text,
                if let Some(pid) = PieceIdentify::from_number(self.id[i]) {
                    pid.to_human_presentable()
                } else {
                    "|".to_string()
                },
                self.ope[i]
            )
            .to_string();
        }

        text.to_string()
    }
}
