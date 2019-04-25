use piece_etc::*;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTapeForJson {
    pub piece_number: Vec<i8>,
    pub operation: Vec<String>,
}
impl RpmTapeForJson {
    pub fn to_human_presentable(&self) -> String {
        let mut text = String::new();

        for i in 0..self.operation.len() {
            text = format!(
                "{} '{}'{}",
                text,
                if let Some(pid) = PieceIdentify::from_number(self.piece_number[i]) {
                    pid.to_human_presentable()
                } else {
                    "|".to_string()
                },
                self.operation[i]
            )
            .to_string();
        }

        text.to_string()
    }
}
