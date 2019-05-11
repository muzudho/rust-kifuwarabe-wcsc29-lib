use media::cassette_tape::*;
use media::two_heads_vec::*;
use serde::*;
use sheet_music_format::kifu_rpm::rpm_tape_tracks::RpmTapeTracks;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::*;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTapeLabel {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub source_file: String,
}
impl RpmTapeLabel {
    pub fn new() -> Self {
        RpmTapeLabel {
            date: "".to_string(),
            event: "".to_string(),
            player1: "".to_string(),
            player2: "".to_string(),
            source_file: "".to_string(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTape {
    pub label: RpmTapeLabel,
    pub tracks: RpmTapeTracks,
}
impl RpmTape {
    pub fn new() -> Self {
        RpmTape {
            label: RpmTapeLabel::new(),
            tracks: RpmTapeTracks::new(),
        }
    }

    pub fn to_human_presentable(&self, app: &Application) -> String {
        self.tracks.to_human_presentable(&app)
    }

    pub fn to_tape_json(&self, app: &Application) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|err| panic!(app.comm.println(&format!("{}", err))))
    }

    pub fn to_object(&self, board_size: BoardSize, app: &Application) -> CassetteTape {
        CassetteTape {
            fragment_file_name: CassetteTape::create_tape_fragment_file_full_name(
                &app.kw29_conf,
                &app,
            ),
            caret: Caret::new_facing_right_caret(),
            label: CassetteTapeLabel {
                date: "".to_string(),
                event: "".to_string(),
                player1: "".to_string(),
                player2: "".to_string(),
                source_file: "".to_string(),
            },
            tracks: TwoHeadsVec::from_vector(
                self.tracks.to_positive_vec(board_size, &app),
                Vec::new(),
            ),
        }
    }
}
