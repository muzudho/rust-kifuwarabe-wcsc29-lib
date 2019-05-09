extern crate rand;
use conf::kifuwarabe_wcsc29_master_config::*;
use rand::Rng;
use sheet_music_format::kifu_rpm::rpm_tape::*;
use sound::shogi_move::ShogiMove;
use sound::shogi_note::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::*;
use studio::application::Application;
use studio::board_size::*;
use studio::common::caret::*;
use studio::common::closed_interval::ClosedInterval;
use video_tape_model::integer_note_vec::*;

/// 対局情報。
pub struct CassetteTapeLabel {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub source_file: String,
}
impl CassetteTapeLabel {
    pub fn clear(&mut self) {
        self.date = "".to_string();
        self.event = "".to_string();
        self.player1 = "".to_string();
        self.player2 = "".to_string();
        self.source_file = "".to_string();
    }

    pub fn to_rpm(&self) -> RpmTapeLabel {
        RpmTapeLabel {
            date: self.date.to_string(),
            event: self.event.to_string(),
            player1: self.player1.to_string(),
            player2: self.player2.to_string(),
            source_file: self.source_file.to_string(),
        }
    }
}

/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1753122
pub struct CassetteTape {
    pub fragment_file_name: String,
    pub caret: Caret,
    pub label: CassetteTapeLabel,
    pub tracks: IntegerNoteVec,
}
impl CassetteTape {
    pub fn new_facing_right(app: &Application) -> Self {
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
            tracks: IntegerNoteVec::default(),
        }
    }

    pub fn new_facing_right_with_file(new_file_for_write: String) -> Self {
        CassetteTape {
            fragment_file_name: new_file_for_write,
            caret: Caret::new_facing_right_caret(),
            label: CassetteTapeLabel {
                date: "".to_string(),
                event: "".to_string(),
                player1: "".to_string(),
                player2: "".to_string(),
                source_file: "".to_string(),
            },
            tracks: IntegerNoteVec::default(),
        }
    }

    /// ランダムにファイル名を付けるぜ☆（*＾～＾*）
    pub fn create_tape_fragment_file_full_name(
        kw29_conf: &KifuwarabeWcsc29MasterConfig,
        app: &Application,
    ) -> String {
        let mut rng = rand::thread_rng();
        let rand1: u64 = rng.gen();
        let rand2: u64 = rng.gen();
        let rand3: u64 = rng.gen();
        let rand4: u64 = rng.gen();
        let file = format!("{}-{}-{}-{}.tapesfrag", rand1, rand2, rand3, rand4).to_string();

        Path::new(&kw29_conf.tapes_fragments)
            .join(file)
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. tape.fragment.")))
            .to_string()
    }

    /// キャレットが左を向いているか☆（＾～＾）
    pub fn is_facing_left_of_caret(&self) -> bool {
        self.caret.is_facing_left()
    }

    /// 範囲はキャレット番地で示す☆（＾～＾）
    /// ０に背を向けた２つのキャレットがあると仮定し、両端はピークを指すキャレット☆（＾～＾）
    pub fn get_span_caret_facing_outward(&self) -> ClosedInterval {
        let span = self.tracks.get_span_caret_facing_outward();
        ClosedInterval::from_all(
            span.get_minimum_caret_number(),
            span.get_maximum_caret_number(),
            self.is_facing_left_of_caret(),
        )
    }

    pub fn get_negative_peak_caret_facing_outward(&self) -> i16 {
        self.tracks.get_negative_peak_caret_facing_outward()
    }

    pub fn get_positive_peak_caret_facing_outward(&self) -> i16 {
        self.tracks.get_positive_peak_caret_facing_outward()
    }

    // キャレットがピークを指しているか☆（＾～＾）
    pub fn is_peak(&self) -> bool {
        let old = self.caret.step_in();
        if -1 < old {
            // 正の数の方で動く☆（＾～＾）
            if self.caret.is_facing_left() {
                // 0 の方を向いている☆（＾～＾）
                self.get_positive_peak_caret_facing_outward() + 1 == old
            } else {
                // 0 に背を向けている☆（＾～＾）
                self.get_positive_peak_caret_facing_outward() == old
            }
        } else {
            // 負の数の方で動く☆（＾～＾）
            if self.caret.is_facing_left() {
                // 0 に背を向けている☆（＾～＾）
                self.get_negative_peak_caret_facing_outward() == old
            } else {
                // 0 の方を向いている☆（＾～＾）
                self.get_negative_peak_caret_facing_outward() - 1 == old
            }
        }
    }
    // キャレットが次、オーバーフローするか☆（＾～＾）
    pub fn is_before_caret_overflow(&self) -> bool {
        let old = self.caret.step_in();
        if -1 < old {
            // 正の数の方で動く☆（＾～＾）
            if self.caret.is_facing_left() {
                // 0 の方を向いている☆（＾～＾）
                self.get_positive_peak_caret_facing_outward() + 1 < old
            } else {
                // 0 に背を向けている☆（＾～＾）
                self.get_positive_peak_caret_facing_outward() < old
            }
        } else {
            // 負の数の方で動く☆（＾～＾）
            if self.caret.is_facing_left() {
                // 0 に背を向けている☆（＾～＾）
                self.get_negative_peak_caret_facing_outward() > old
            } else {
                // 0 の方を向いている☆（＾～＾）
                self.get_negative_peak_caret_facing_outward() - 1 > old
            }
        }
    }

    /// 連結。
    pub fn append_cassette_tape_to_right(&mut self, cassette_tape_to_empty: &mut CassetteTape) {
        self.tracks
            .append_tape_to_right(&mut cassette_tape_to_empty.tracks);
    }
    pub fn append_cassette_tape_to_left(&mut self, cassette_tape_to_empty: &mut CassetteTape) {
        self.tracks
            .append_tape_to_left(&mut cassette_tape_to_empty.tracks);
    }

    /// フェーズ・チェンジか、エンド・オブ・テープを拾うまで進める☆（＾～＾）
    ///
    /// # Returns
    ///
    /// (taken overflow, caret number, フェーズ・チェンジを含み、オーバーフローを含まない１手の範囲)
    pub fn seek_1move(&mut self, app: &Application) -> (bool, ShogiMove) {
        self.tracks.seek_1move(&mut self.caret, &app)
    }

    /// キャレットは必ず１つ進みます。
    /// 0 は、正の数とします。（マイナスゼロは無いです）
    /// Noneを返したら、オーバーフローしています。
    ///
    /// # Returns
    ///
    /// (taken overflow, move, note)
    pub fn seek_to_next(&mut self, app: &Application) -> (bool, ShogiMove, Option<ShogiNote>) {
        self.tracks.seek_to_next(&mut self.caret, &app)
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    ///
    /// # Returns
    ///
    /// (taken overflow, move, note)
    pub fn seek_to_next_with_othre_caret(
        &self,
        caret: &mut Caret,
        app: &Application,
    ) -> (bool, ShogiMove, Option<ShogiNote>) {
        self.tracks.seek_to_next(caret, &app)
    }

    /// コマンドライン入力形式の棋譜。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size: BoardSize) -> (String, String) {
        self.tracks.to_sign(board_size)
    }

    /// このテープを、テープ・フラグメント書式で書きだすぜ☆（＾～＾）
    pub fn write_tape_fragment(&self, board_size: BoardSize, app: &Application) {
        if app.is_debug() {
            app.comm.println(&format!(
                "#Write tape fragment to '{}'...",
                self.fragment_file_name
            ));
        }

        let path = Path::new(&self.fragment_file_name);

        // ディレクトリー作成。
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_x) => {}
                Err(err) => panic!(err),
            }
        } else {
            panic!("Create directory fail. {}", self.fragment_file_name);
        }

        // 末尾に カンマ を付けて追記していくぜ☆（＾～＾）JSONとしては不完全だから、フラグメントだぜ☆（＾～＾）
        let mut file_obj = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|err| panic!(app.comm.panic_io(&err)));

        if let Err(e) = writeln!(file_obj, "{},", self.to_rpm(board_size).to_tape_json(&app)) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // comm.println("#Sheet saved.");
    }

    pub fn to_rpm(&self, board_size: BoardSize) -> RpmTape {
        RpmTape {
            label: self.label.to_rpm(),
            tracks: self.tracks.to_rpm_tracks(board_size),
        }
    }

    /// Human presentable large log.
    pub fn to_human_presentable(&self, board_size: BoardSize, app: &Application) -> String {
        format!(
            "[Tape: {} {}]",
            self.caret.to_human_presentable(&app),
            self.tracks.to_human_presentable(board_size, &app)
        )
        .to_string()
    }
}
