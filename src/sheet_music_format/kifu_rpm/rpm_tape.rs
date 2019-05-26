use media::cassette_tape::*;
use media::two_heads_vec::*;
use serde::*;
use sheet_music_format::kifu_rpm::rpm_tape_tracks::RpmTapeTracks;
use sheet_music_format::tape_label::*;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::*;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct RpmTape {
    pub label: TapeLabel,
    pub tracks: RpmTapeTracks,
}
impl RpmTape {
    pub fn new() -> Self {
        RpmTape {
            label: TapeLabel::new(),
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
            label: TapeLabel::new(),
            tracks: TwoHeadsVec::from_vector(
                self.tracks.to_positive_vec(board_size, &app),
                Vec::new(),
            ),
        }
    }
}
