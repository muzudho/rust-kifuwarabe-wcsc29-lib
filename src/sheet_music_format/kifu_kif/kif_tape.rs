use sheet_music_format::kifu_kif::kif_move::*;
use sheet_music_format::kifu_kif::kif_parser::*;
use sheet_music_format::tape_label::*;
use std::*;
use studio::application::Application;

#[derive(Default)]
pub struct KifTape {
    // 指し手。
    pub moves: Vec<KifMove>,

    // テープ・ラベル。
    tape_label: TapeLabel,
}
impl KifTape {
    pub fn new() -> KifTape {
        KifTape {
            moves: Vec::new(),
            tape_label: TapeLabel::new(),
        }
    }

    // #####
    // # F #
    // #####

    /// ファイル読取。
    pub fn from_file(file: &str, app: &Application) -> KifTape {
        // .kif形式には、バージョンがいろいろあるようだ。
        KifParser::from_file(file, &app)
    }

    // #####
    // # G #
    // #####

    // テープ・ラベル。
    pub fn get_mut_tape_label(&mut self) -> &mut TapeLabel {
        &mut self.tape_label
    }

    // #####
    // # P #
    // #####

    /// 指し手を追加。
    pub fn push_move(&mut self, mov: KifMove) {
        self.moves.push(mov);
    }
}
