use piece_etc::*;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTapeTracks {
    pub id: Vec<i8>,
    // 操作は、半角スペース１個区切り。
    pub ope: String,
}
impl RpmTapeTracks {
    pub fn to_human_presentable(&self) -> String {
        let mut text = String::new();

        let ope_vec: Vec<&str> = self.ope.split(' ').collect();

        for (i, ope_element) in ope_vec.iter().enumerate() {
            text = format!(
                "{} '{}'{}",
                text,
                if let Some(pid) = PieceIdentify::from_number(self.id[i]) {
                    pid.to_human_presentable()
                } else {
                    "|".to_string()
                },
                ope_element
            )
            .to_string();
        }

        text.to_string()
    }
}
