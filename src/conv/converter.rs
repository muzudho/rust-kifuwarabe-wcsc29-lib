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
    fn get_extension_from_file_path(file_path: &str) -> Option<&str> {
        Path::new(file_path).extension().and_then(OsStr::to_str)
    }

    fn get_file_stem_from_file_path(file_path: &str) -> Option<&str> {
        Path::new(file_path).file_stem().and_then(OsStr::to_str)
    }

    pub fn convert(
        in_file: String,
        rack: &mut AudioRack,
        position: &mut Position,
        app: &Application,
    ) {
        let file_stem = Converter::get_file_stem_from_file_path(&in_file)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_file_stem_from_file_path.")));

        let extension = Converter::get_extension_from_file_path(&in_file)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_extension_from_file_path.")))
            .to_uppercase();

        match extension.as_str() {
            "KIF" => {
                // Training data.
                let mut tape = KifTape::from_file(&in_file, &app);

                // Play out.
                KifConverter::play_out_kifu_tape(&tape, rack, position, &app);

                // Tape label
                rack.set_name_of_tape(Slot::Learning, file_stem.to_string());
                rack.set_game_date_of_tape(Slot::Learning, tape.get_game_date());
                rack.set_event_of_tape(Slot::Learning, tape.get_event());
                rack.set_player1_of_tape(Slot::Learning, tape.get_player1());
                rack.set_player2_of_tape(Slot::Learning, tape.get_player2());

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
                rack.set_name_of_tape(Slot::Learning, file_stem.to_string());
                rack.set_game_date_of_tape(Slot::Learning, tape.get_game_date());
                rack.set_event_of_tape(Slot::Learning, tape.get_event());
                rack.set_player1_of_tape(Slot::Learning, tape.get_player1());
                rack.set_player2_of_tape(Slot::Learning, tape.get_player2());

                // Write.
                rack.write_leaning_tapes_fragment(position.get_board_size(), &app);
            }
            _ => print!("Pass extension: {}", extension),
        }
    }
}
