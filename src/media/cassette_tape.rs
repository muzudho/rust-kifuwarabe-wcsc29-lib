extern crate rand;
use conf::kifuwarabe_wcsc29_master_config::*;
use media::two_heads_vec::*;
use rand::Rng;
use sheet_music_format::kifu_rpm::rpm_tape::*;
use sheet_music_format::tape_label::TapeLabel;
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

/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1752788
/// 説明 https://ch.nicovideo.jp/kifuwarabe/blomaga/ar1753122
pub struct CassetteTape {
    pub fragment_file_name: String,
    pub caret: Caret,
    pub label: TapeLabel,
    pub tracks: TwoHeadsVec,
}
impl CassetteTape {
    // ###############
    // # Constructor #
    // ###############
    pub fn new_facing_right(app: &Application) -> Self {
        CassetteTape {
            fragment_file_name: CassetteTape::create_tape_fragment_file_full_name(
                &app.kw29_conf,
                &app,
            ),
            caret: Caret::new_facing_right_caret(),
            label: TapeLabel::new(),
            tracks: TwoHeadsVec::default(),
        }
    }

    // #####
    // # A #
    // #####

    /// 連結。
    pub fn append_cassette_tape_to_right(&mut self, cassette_tape_to_empty: &mut CassetteTape) {
        self.tracks
            .append_tape_to_right(&mut cassette_tape_to_empty.tracks);
    }
    pub fn append_cassette_tape_to_left(&mut self, cassette_tape_to_empty: &mut CassetteTape) {
        self.tracks
            .append_tape_to_left(&mut cassette_tape_to_empty.tracks);
    }

    // #####
    // # C #
    // #####

    /// ラベルを除くクリアー。
    pub fn clear_tape_body(&mut self, _app: &Application) {
        self.caret.clear_facing_right();
        self.tracks.clear();
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

    // #####
    // # G #
    // #####

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

    // #####
    // # I #
    // #####

    pub fn insert_note(&mut self, note: ShogiNote, board_size: BoardSize, app: &Application) {
        self.tracks =
            self.tracks
                .new_vector_with_inserted_note(&mut self.caret, note, board_size, &app);
    }

    /// キャレットが左を向いているか☆（＾～＾）
    pub fn is_facing_left_of_caret(&self) -> bool {
        self.caret.is_facing_left()
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

    // #####
    // # P #
    // #####

    /// 正の方のテープの末端にノートを追加。
    pub fn push_note(&mut self, note: ShogiNote) {
        self.tracks.push_note(note);
    }
    pub fn pop_note(&mut self) -> Option<ShogiNote> {
        self.tracks.pop_note()
    }

    // #####
    // # S #
    // #####

    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note(&mut self, app: &Application) -> (bool, Awareness, Option<ShogiNote>) {
        self.tracks.seek_a_note(&mut self.caret, &app)
    }

    pub fn set_file_full_name_without_extension(&mut self, file_name_without_extension: &str) {
        self.fragment_file_name = format!("{}.tapesfrag", file_name_without_extension).to_string();
    }

    /// テープのラベルを書く。
    pub fn set_label(&mut self, tape_label: &TapeLabel) {
        self.label = tape_label.clone();
    }

    /// # Returns
    ///
    /// (taken overflow, caret number, フェーズ・チェンジを含み、オーバーフローを含まない１手の範囲)
    pub fn skip_a_move(&mut self, app: &Application) -> (bool, ShogiMove) {
        self.tracks.skip_a_move(&mut self.caret, &app)
    }

    /// 正負の両端の先端要素を超えたら、キャレットは進めずにNoneを返します。
    ///
    /// # Returns
    ///
    /// (taken overflow, awareness, note)
    pub fn seek_a_note_with_othre_caret(
        &self,
        caret: &mut Caret,
        app: &Application,
    ) -> (bool, Awareness, Option<ShogiNote>) {
        self.tracks.seek_a_note(caret, &app)
    }

    // #####
    // # T #
    // #####

    /// コマンドライン入力形式の棋譜。
    ///
    /// # Returns
    ///
    /// 駒の背番号, 操作。
    pub fn to_sign(&self, board_size: BoardSize) -> (String, String) {
        self.tracks.to_sign(board_size)
    }

    pub fn to_rpm(&self, board_size: BoardSize) -> RpmTape {
        RpmTape {
            label: self.label.clone(),
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

    // #####
    // # W #
    // #####

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
}
