use audio_compo::audio_rack::*;
use audio_compo::cassette_deck::Slot;
use instrument::position::*;
use std::*;
use studio::application::Application;

pub struct TrackView {}
impl TrackView {
    /// TODO キャレット付近表示。
    pub fn show_track_by_slot(
        rack: &mut AudioRack,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) {
        if app.is_debug() {
            TrackView::show_ope_track_by_slot(rack, slot, position, &app);
        }
    }

    /// TODO キャレット付近表示。
    pub fn show_ope_track_by_slot(
        rack: &mut AudioRack,
        slot: Slot,
        position: &mut Position,
        app: &Application,
    ) {
        if app.is_debug() {
            let (_numbers, operations) =
                &rack.get_sign_of_current_tape(slot, position.get_board_size());

            // 1セルは 5width。
            let e10 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e11 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e12 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e13 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e14 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e15 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e16 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e17 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e18 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e19 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };

            // 10個戻る。
            rack.look_back_caret(slot, &app);
            for _i in 0..10 {
                rack.seek_a_note(slot, &app);
            }

            let e09 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e08 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e07 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e06 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e05 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e04 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e03 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e02 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e01 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };
            let e00 = {
                let (_taken_overflow, _awareness, note_opt) = rack.seek_a_note(slot, &app);
                if let Some(note) = note_opt {
                    note.to_human_presentable_ope_5width(position.get_board_size(), &app)
                } else {
                    "     ".to_string()
                }
            };

            // 10個戻る。
            rack.look_back_caret(slot, &app);
            for _i in 0..10 {
                rack.seek_a_note(slot, &app);
            }

            app.comm.println(
                "+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+ # +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+",
            );
            app.comm.println(&format!(
                "|{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}| # |{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}+",
                e00.to_string(),
                e01.to_string(),
                e02.to_string(),
                e03.to_string(),
                e04.to_string(),
                e05.to_string(),
                e06.to_string(),
                e07.to_string(),
                e08.to_string(),
                e09.to_string(),
                e10.to_string(),
                e11.to_string(),
                e12.to_string(),
                e13.to_string(),
                e14.to_string(),
                e15.to_string(),
                e16.to_string(),
                e17.to_string(),
                e18.to_string(),
                e19.to_string(),
            ));
            app.comm.println(
                "+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+ # +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+",
            );
        }
    }
}
