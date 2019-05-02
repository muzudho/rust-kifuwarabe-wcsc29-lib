use instrument::piece_etc::*;
use serde::*;
use sound::shogi_note::ShogiNote;
use sound::shogi_note_operation::ShogiNoteOpe;
use studio::application::*;
use studio::board_size::*;
use studio::common::caret::*;

#[derive(Debug, Default, Deserialize, Serialize)]
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

    pub fn to_human_presentable(&self, app: &Application) -> String {
        let mut text = String::new();

        let id_vec: Vec<&str> = self.id.split(' ').collect();
        let ope_vec: Vec<&str> = self.ope.split(' ').collect();

        for (i, ope_element) in ope_vec.iter().enumerate() {
            let id: i8 = id_vec[i]
                .parse()
                .unwrap_or_else(|err| panic!(app.comm.println(&format!("{}", err))));

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

        for (i, ope_element) in ope_vec.iter().enumerate() {
            let id: i8 = id_vec[i]
                .parse()
                .unwrap_or_else(|err| panic!(app.comm.println(&format!("{}", err))));

            let mut caret = Caret::new_facing_right_caret();
            let (_last_caret, note_ope_opt) =
                ShogiNoteOpe::parse_1ope(&ope_element, &mut caret, board_size, &app);

            if let Some(note_ope) = note_ope_opt {
                notes.push(ShogiNote::from_id_ope(
                    PieceIdentify::from_number(id),
                    note_ope,
                ));
            } else {
                panic!("Note_ope none.")
            }
        }

        notes
    }
}
