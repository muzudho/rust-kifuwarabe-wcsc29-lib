use application::*;
use board_size::*;
use human::human_interface::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_editor::*;
use kifu_rpm::cassette_deck::rpm_cassette_tape_recorder::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::RpmCassetteTapeBoxConveyor;
use kifu_usi::usi_converter::*;
use piece_etc::*;
use position::*;
use thought::best_move_picker::*;

pub struct LibSub {}
impl LibSub {
    pub fn back_1_note(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        let rec_tape = tape_box_conveyor.get_mut_recording_cassette_tape();

        rec_tape.caret.turn_to_negative();
        if let Some(rnote) = rec_tape.go_1note_forcely(&app.comm) {
            RpmCassetteTapeRecorder::try_1note_on_1note(&rnote, position, recorder.ply, &app.comm);
            HumanInterface::bo(&app.comm, &rec_tape, recorder.ply, &position);
        }
    }

    pub fn back_1_move(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        let rec_tape = tape_box_conveyor.get_mut_recording_cassette_tape();
        rec_tape.caret.turn_to_negative();
        RpmCassetteTapeRecorder::try_1move_on_tape(
            rec_tape,
            position,
            recorder.ply,
            true,
            &app.comm,
        );
        HumanInterface::bo(&app.comm, &rec_tape, recorder.ply, &position);
    }

    pub fn back_10_move(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        tape_box_conveyor
            .get_mut_recording_cassette_tape()
            .caret
            .turn_to_negative();
        for _i in 0..10 {
            RpmCassetteTapeRecorder::try_1move_on_tape(
                &mut tape_box_conveyor.get_mut_recording_cassette_tape(),
                position,
                recorder.ply,
                true,
                &app.comm,
            );
        }
        HumanInterface::bo(
            &app.comm,
            &tape_box_conveyor.recording_cassette_tape,
            recorder.ply,
            &position,
        );
    }

    pub fn back_400_move(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        tape_box_conveyor
            .get_mut_recording_cassette_tape()
            .caret
            .turn_to_negative();
        for _i in 0..400 {
            RpmCassetteTapeRecorder::try_1move_on_tape(
                &mut tape_box_conveyor.get_mut_recording_cassette_tape(),
                position,
                recorder.ply,
                true,
                &app.comm,
            );
        }
        HumanInterface::bo(
            &app.comm,
            &tape_box_conveyor.recording_cassette_tape,
            recorder.ply,
            &position,
        );
    }

    pub fn forward_1_note(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        tape_box_conveyor
            .get_mut_recording_cassette_tape()
            .caret
            .turn_to_positive();
        if let Some(rnote) = tape_box_conveyor
            .get_mut_recording_cassette_tape()
            .go_1note_forcely(&app.comm)
        {
            RpmCassetteTapeRecorder::try_1note_on_1note(&rnote, position, recorder.ply, &app.comm);
            HumanInterface::bo(
                &app.comm,
                &tape_box_conveyor.recording_cassette_tape,
                recorder.ply,
                &position,
            );
        }
    }

    pub fn forward_1_move(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        // 非合法タッチは自動で戻します。
        tape_box_conveyor
            .get_mut_recording_cassette_tape()
            .caret
            .turn_to_positive();
        RpmCassetteTapeRecorder::try_1move_on_tape(
            &mut tape_box_conveyor.get_mut_recording_cassette_tape(),
            position,
            recorder.ply,
            true,
            &app.comm,
        );
        HumanInterface::bo(
            &app.comm,
            &tape_box_conveyor.recording_cassette_tape,
            recorder.ply,
            &position,
        );
    }

    pub fn forward_10_move(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        tape_box_conveyor
            .get_mut_recording_cassette_tape()
            .caret
            .turn_to_positive();
        for _i in 0..10 {
            RpmCassetteTapeRecorder::try_1move_on_tape(
                &mut tape_box_conveyor.get_mut_recording_cassette_tape(),
                position,
                recorder.ply,
                true,
                &app.comm,
            );
        }
        HumanInterface::bo(
            &app.comm,
            &tape_box_conveyor.recording_cassette_tape,
            recorder.ply,
            &position,
        );
    }

    pub fn forward_400_move(
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        tape_box_conveyor
            .recording_cassette_tape
            .caret
            .turn_to_positive();
        for _i in 0..400 {
            RpmCassetteTapeRecorder::try_1move_on_tape(
                &mut tape_box_conveyor.get_mut_recording_cassette_tape(),
                position,
                recorder.ply,
                true,
                &app.comm,
            );
        }
        HumanInterface::bo(
            &app.comm,
            &tape_box_conveyor.recording_cassette_tape,
            recorder.ply,
            &position,
        );
    }

    pub fn go(
        best_move_picker: &mut BestMovePicker,
        position: &mut Position,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        recorder: &mut RpmCassetteTapeEditor,
        app: &Application,
    ) {
        tape_box_conveyor
            .get_mut_recording_cassette_tape()
            .caret
            .turn_to_positive();
        let best_umove =
            best_move_picker.get_mut_best_move(position, tape_box_conveyor, recorder, &app);
        // Examples.
        // println!("bestmove 7g7f");
        // println!("bestmove win");
        // println!("bestmove resign");
        app.comm
            .println(&format!("bestmove {}", best_umove.to_sign()));

        // USI を再翻訳して再生するぜ☆（＾～＾）
        let rnote_opes = UsiConverter::convert_move(best_umove, &position, recorder.ply);
        for rnote_ope in rnote_opes {
            app.comm.println("lib.rs:go: touch_1note_ope");
            RpmCassetteTapeRecorder::touch_1note_ope(
                &rnote_ope,
                position,
                tape_box_conveyor,
                recorder,
                &app.comm,
            );
        }
    }

    pub fn gameover(
        board_size: BoardSize,
        tape_box_conveyor: &mut RpmCassetteTapeBoxConveyor,
        app: &Application,
    ) {
        tape_box_conveyor.write_cassette_tape_box(board_size, &app);
    }

    pub fn hand1(position: &Position, app: &Application) {
        // TODO 先手の持ち駒を表示。
        app.comm.println(&position.to_hand_text(Some(Phase::First)));
    }
    pub fn hand2(position: &Position, app: &Application) {
        // TODO 後手の持ち駒を表示。
        app.comm
            .println(&position.to_hand_text(Some(Phase::Second)));
    }
    pub fn hand3(position: &Position, app: &Application) {
        // TODO 使っていない駒を表示。
        app.comm.println(&position.to_hand_text(None));
    }
}
