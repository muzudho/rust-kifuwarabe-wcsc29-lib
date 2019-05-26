use audio_compo::audio_rack::*;
use audio_compo::cassette_deck::Slot;
use instrument::position::*;
use std::*;
use studio::application::Application;

pub struct TapeView {}
impl TapeView {
    fn seek(
        rack: &mut AudioRack,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) -> (String, String, String) {
        let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
        if let Some(note) = note_opt {
            (
                note.to_human_presentable_id_5width(),
                note.to_human_presentable_ope_5width(position.get_board_size(), &app),
                note.to_human_presentable_facing_5width(),
            )
        } else {
            (
                "     ".to_string(),
                "     ".to_string(),
                "     ".to_string(),
            )
        }
    }

    /// TODO キャレット付近表示。
    pub fn show_tape_by_slot(
        rack: &mut AudioRack,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) {
        if app.is_debug() {
            if rack.is_none_current_tape(slot) {
                app.comm.println("There is no tape.");
                return;
            }

            // キャレットの向き。
            let caret_text = if rack.is_facing_left_of_current_tape(slot, app) {
                "<#".to_string()
            } else {
                "#>".to_string()
            };

            // 1セルは 5width。
            let (id10, op10, fc10) = TapeView::seek(rack, slot, position, app);
            let (id11, op11, fc11) = TapeView::seek(rack, slot, position, app);
            let (id12, op12, fc12) = TapeView::seek(rack, slot, position, app);
            let (id13, op13, fc13) = TapeView::seek(rack, slot, position, app);
            let (id14, op14, fc14) = TapeView::seek(rack, slot, position, app);
            let (id15, op15, fc15) = TapeView::seek(rack, slot, position, app);
            let (id16, op16, fc16) = TapeView::seek(rack, slot, position, app);
            let (id17, op17, fc17) = TapeView::seek(rack, slot, position, app);
            let (id18, op18, fc18) = TapeView::seek(rack, slot, position, app);
            let (id19, op19, fc19) = TapeView::seek(rack, slot, position, app);

            // 10個戻る。
            rack.look_back_caret(slot, &app);
            for _i in 0..10 {
                rack.seek_a_note(slot, &app);
            }

            let (id09, op09, fc09) = TapeView::seek(rack, slot, position, app);
            let (id08, op08, fc08) = TapeView::seek(rack, slot, position, app);
            let (id07, op07, fc07) = TapeView::seek(rack, slot, position, app);
            let (id06, op06, fc06) = TapeView::seek(rack, slot, position, app);
            let (id05, op05, fc05) = TapeView::seek(rack, slot, position, app);
            let (id04, op04, fc04) = TapeView::seek(rack, slot, position, app);
            let (id03, op03, fc03) = TapeView::seek(rack, slot, position, app);
            let (id02, op02, fc02) = TapeView::seek(rack, slot, position, app);
            let (id01, op01, fc01) = TapeView::seek(rack, slot, position, app);
            let (id00, op00, fc00) = TapeView::seek(rack, slot, position, app);

            // 10個戻る。
            rack.look_back_caret(slot, &app);
            for _i in 0..10 {
                rack.seek_a_note(slot, &app);
            }

            app.comm.println(
                &format!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+ {} +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+",caret_text),
            );
            app.comm.println(&format!(
                "|{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}| {20} |{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}+",
                id00.to_string(),
                id01.to_string(),
                id02.to_string(),
                id03.to_string(),
                id04.to_string(),
                id05.to_string(),
                id06.to_string(),
                id07.to_string(),
                id08.to_string(),
                id09.to_string(),
                id10.to_string(),
                id11.to_string(),
                id12.to_string(),
                id13.to_string(),
                id14.to_string(),
                id15.to_string(),
                id16.to_string(),
                id17.to_string(),
                id18.to_string(),
                id19.to_string(),
                caret_text
            ));
            app.comm.println(&format!(
                "|{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}| {20} |{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}+",
                op00.to_string(),
                op01.to_string(),
                op02.to_string(),
                op03.to_string(),
                op04.to_string(),
                op05.to_string(),
                op06.to_string(),
                op07.to_string(),
                op08.to_string(),
                op09.to_string(),
                op10.to_string(),
                op11.to_string(),
                op12.to_string(),
                op13.to_string(),
                op14.to_string(),
                op15.to_string(),
                op16.to_string(),
                op17.to_string(),
                op18.to_string(),
                op19.to_string(),
                caret_text
            ));
            app.comm.println(&format!(
                "|{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}| {20} |{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}+",
                fc00.to_string(),
                fc01.to_string(),
                fc02.to_string(),
                fc03.to_string(),
                fc04.to_string(),
                fc05.to_string(),
                fc06.to_string(),
                fc07.to_string(),
                fc08.to_string(),
                fc09.to_string(),
                fc10.to_string(),
                fc11.to_string(),
                fc12.to_string(),
                fc13.to_string(),
                fc14.to_string(),
                fc15.to_string(),
                fc16.to_string(),
                fc17.to_string(),
                fc18.to_string(),
                fc19.to_string(),
                caret_text
            ));
            app.comm.println(
                &format!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+ {} +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+",caret_text),
            );
        }
    }
}
