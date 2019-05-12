use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::position::*;
use live::base_performer::*;
use live::best_move_picker::*;
use sheet_music_format::kifu_usi::usi_converter::*;
use studio::application::*;

pub struct ComputerPerformer {}
impl ComputerPerformer {
    // #####
    // # G #
    // #####

    pub fn go(
        best_move_picker: &mut BestMovePicker,
        deck: &mut CassetteDeck,
        position: &mut Position,
        app: &Application,
    ) {
        deck.slots[Slot::Learning as usize].turn_caret_towards_positive_infinity(&app);

        let best_umove = best_move_picker.get_mut_best_move(position, deck, &app);
        // Examples.
        // println!("bestmove 7g7f");
        // println!("bestmove win");
        // println!("bestmove resign");
        app.comm
            .println(&format!("bestmove {}", best_umove.to_sign(&app)));

        // USI を再翻訳して再生するぜ☆（＾～＾）
        let rnote_opes =
            UsiConverter::convert_move(best_umove, &position, deck.get_ply(Slot::Learning), &app);
        for rnote_ope in rnote_opes {
            // app.comm.println("lib.rs:go: touch_1note_ope");

            // 非合法手はいったん出力し、将棋所の方でエラーにする☆（＾～＾）
            BasePerformer::improvise_note_ope_no_log(deck, &rnote_ope, true, position, &app);
            HumanInterface::bo(deck, position, &app);
        }
    }
}
