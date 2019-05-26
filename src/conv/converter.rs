use audio_compo::audio_rack::*;
use audio_compo::cassette_deck::*;
use instrument::position::*;
use sheet_music_format::kifu_csa::csa_converter::CsaConverter;
use sheet_music_format::kifu_csa::csa_tape::*;
use sheet_music_format::kifu_kif::kif_converter::KifConverter;
use sheet_music_format::kifu_kif::kif_tape::*;
use std::ffi::OsStr;
use std::path::Path;
use studio::application::*;

pub struct Converter {}

impl Converter {
    pub fn convert(
        in_file: String,
        rack: &mut AudioRack,
        position: &mut Position,
        app: &Application,
    ) {
        let extension = Path::new(&in_file)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_extension_from_file_path.")))
            .to_uppercase();

        match extension.as_str() {
            "KIF" => {
                // Training data.
                let mut tape = KifTape::from_file(&in_file, &app);

                // Play out.
                KifConverter::play_out_kifu_tape(&tape, rack, position, &app);

                // Tape label
                rack.set_label_of_tape(Slot::Learning, tape.get_mut_tape_label());

                // Write.
                rack.write_leaning_tapes_fragment(position.get_board_size(), &app);
            }
            "CSA" => {
                // Training data.
                let mut tape = CsaTape::from_file(&in_file, &app);

                if app.is_debug() {
                    app.comm
                        .println(&format!("Ctape: '{}'", tape.to_human_presentable()));
                }

                // Play out.
                CsaConverter::play_out_csa_tape(&tape, rack, position, &app);

                // Tape label
                rack.set_label_of_tape(Slot::Learning, tape.get_mut_tape_label());

                // Write.
                rack.write_leaning_tapes_fragment(position.get_board_size(), &app);
            }
            _ => print!("Pass extension: {}", extension),
        }
    }
}
