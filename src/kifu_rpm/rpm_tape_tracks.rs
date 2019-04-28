use application::*;
use board_size::*;
use common::caret::*;
use object_rpm::shogi_note::ShogiNote;
use object_rpm::shogi_note_operation::ShogiNoteOpe;
use piece_etc::*;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTapeTracks {
    // 駒の背番号は、半角スペース１個区切り。
    pub id: String,
    // 操作は、半角スペース１個区切り。
    pub ope: String,
}
impl RpmTapeTracks {
    pub fn new() -> Self {
        RpmTapeTracks {
            id: "".to_string(),
            ope: "".to_string(),
        }
    }

    pub fn to_human_presentable(&self) -> String {
        let mut text = String::new();

        let id_vec: Vec<&str> = self.id.split(' ').collect();
        let ope_vec: Vec<&str> = self.ope.split(' ').collect();

        for (i, ope_element) in id_vec.iter().enumerate() {
            let id: i8 = id_vec[i].parse().unwrap();

            text = format!(
                "{} '{}'{}",
                text,
                if let Some(pid) = PieceIdentify::from_number(id) {
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

    /// 変換。
    pub fn to_positive_vec(&self, board_size: BoardSize, app: &Application) -> Vec<ShogiNote> {
        let mut notes: Vec<ShogiNote> = Vec::new();

        let id_vec: Vec<&str> = self.id.split(' ').collect();
        let ope_vec: Vec<&str> = self.ope.split(' ').collect();

        for (i, ope_element) in id_vec.iter().enumerate() {
            let id: i8 = id_vec[i].parse().unwrap();

            let caret = Caret::new_facing_right_caret();
            let (_last_caret, Some(note_ope)) =
                ShogiNoteOpe::parse_1ope(&ope_element, &mut caret, board_size, &app.comm);

            notes.push(ShogiNote {
                identify: PieceIdentify::from_number(id),
                operation: note_ope,
            });
        }

        notes
    }
}
