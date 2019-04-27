extern crate getopts;
/// My name is Kifuwarabe.
/// I am a computer shogi engine.
/// I will go to WCSC29 this year.
///
/// Let's explain how to use me.
///
/// Windows 10.
///
/// [Windows] + [R].
/// `cmd`, [Enter].
///
/// ```Shell
/// ### Example.
/// cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
/// cls
///
/// ### Compile.
/// cargo clippy
///
/// ### Build.
/// cargo build --release
/// ```
///
/// Execution file.
/// C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/rust-kifuwarabe-wcsc29.exe

/// extern crate は main.rs か lib.rs に入れる。
/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;
use std::io;
pub mod address;
pub mod application;
pub mod board_size;
pub mod common;
pub mod communication;
pub mod conf;
pub mod human;
pub mod kifu_csa;
pub mod kifu_kif;
pub mod kifu_rpm;
pub mod learn;
pub mod parser;
pub mod piece_etc;
pub mod position;
pub mod thought;
pub mod usi_conv;
use application::*;
use human::human_interface::*;
use kifu_rpm::object::rpm_cassette_tape_box_conveyor::*;
use kifu_rpm::play::rpm_move_player::*;
use kifu_rpm::play::rpm_note_player::*;
use kifu_rpm::recorder::rpm_cassette_tape_recorder::*;
use piece_etc::*;
use position::*;
use thought::best_move_picker::*;
use usi_conv::fen::*;
use usi_conv::usi_player::*;
use usi_conv::usi_position::*;

pub fn main_loop() {
    // The application contains all immutable content.
    let app = Application::new();

    // Record.
    let mut tape_box_conveyor = RpmCassetteTapeBoxConveyor::new_empty();
    let mut recorder = RpmCassetteTapeRecorder::new_cassette_tape_recorder();

    let mut position = Position::default();
    let mut best_move_picker = BestMovePicker::default();

    loop {
        // Standard input.
        // Be sure to add "info" before the output message.
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("info Failed: stdin.read_line.");

        // Excludes trailing newlines. The surrounding whitespace deletes.
        line = line.trim().parse().expect("info Failed: stdin parse.");

        // ######
        // # 数 #
        // ######
        // ########
        // # 記号 #
        // ########

        if line.starts_with('0')
            || line.starts_with('1')
            || line.starts_with('2')
            || line.starts_with('3')
            || line.starts_with('4')
            || line.starts_with('5')
            || line.starts_with('6')
            || line.starts_with('7')
            || line.starts_with('8')
            || line.starts_with('9')
            || line.starts_with('+')
            || line.starts_with('-')
            || line.starts_with('|')
        {
            recorder.read_tape(&app.comm, &line, &mut position);

        // #####
        // # B #
        // #####
        } else if line == "b" {
            // Back 1mark.
            recorder.cassette_tape.caret.turn_to_negative();
            if let Some(rnote) = recorder.cassette_tape.get_note_and_go_tape(&app.comm) {
                RpmNotePlayer::go_1note(&rnote, &mut position, recorder.ply, &app.comm);
                HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
            }
        } else if line == "bb" {
            // Back 1ply.
            recorder.cassette_tape.caret.turn_to_negative();
            RpmMovePlayer::go_next_1_move(
                &mut recorder.cassette_tape,
                &mut position,
                recorder.ply,
                true,
                &app.comm,
            );
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "bbb" {
            // Back 10ply.
            recorder.cassette_tape.caret.turn_to_negative();
            for _i in 0..10 {
                RpmMovePlayer::go_next_1_move(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &app.comm,
                );
            }
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "bbbb" {
            // Back 400ply.
            recorder.cassette_tape.caret.turn_to_negative();
            for _i in 0..400 {
                RpmMovePlayer::go_next_1_move(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &app.comm,
                );
            }
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line.starts_with("bo") {
            // Board.
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);

        /*
        // #####
        // # C #
        // #####
        } else if line == "cls" {
            // Clear screen.
            comm.println(&format!("{}[2J", 27 as char));
            */

        // #####
        // # D #
        // #####
        } else if line == "d" {
            // Delete 1mark.
            RpmNotePlayer::pop_1note(&mut recorder, &mut position, &app.comm);
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "dd" {
            // Delete 1ply.
            RpmMovePlayer::pop_current_1move_on_record(&mut recorder, &mut position, &app.comm);
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "ddd" {
            // Delete 10ply.
            for _i in 0..10 {
                RpmMovePlayer::pop_current_1move_on_record(&mut recorder, &mut position, &app.comm);
            }
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "dddd" {
            // Delete 400ply.
            for _i in 0..400 {
                RpmMovePlayer::pop_current_1move_on_record(&mut recorder, &mut position, &app.comm);
            }
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        // #####
        // # N #
        // #####
        } else if line == "f" {
            // Forward 1note.
            recorder.cassette_tape.caret.turn_to_positive();
            if let Some(rnote) = recorder.cassette_tape.get_note_and_go_tape(&app.comm) {
                RpmNotePlayer::go_1note(&rnote, &mut position, recorder.ply, &app.comm);
                HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
            }
        } else if line == "ff" {
            // Forward 1move. （非合法タッチは自動で戻します）
            recorder.cassette_tape.caret.turn_to_positive();
            RpmMovePlayer::go_next_1_move(
                &mut recorder.cassette_tape,
                &mut position,
                recorder.ply,
                true,
                &app.comm,
            );
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "fff" {
            // Forward 10move.
            recorder.cassette_tape.caret.turn_to_positive();
            for _i in 0..10 {
                RpmMovePlayer::go_next_1_move(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &app.comm,
                );
            }
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "ffff" {
            // Forward 400move.
            recorder.cassette_tape.caret.turn_to_positive();
            for _i in 0..400 {
                RpmMovePlayer::go_next_1_move(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &app.comm,
                );
            }
            HumanInterface::bo(&app.comm, &recorder.cassette_tape, recorder.ply, &position);

        // #####
        // # G #
        // #####
        } else if line.starts_with("go") {
            recorder.cassette_tape.caret.turn_to_positive();
            let best_logical_move = best_move_picker.get_best_move(
                &app.comm,
                &app.kw29_conf,
                &mut recorder,
                &mut position,
            );
            // Examples.
            // println!("bestmove 7g7f");
            // println!("bestmove win");
            // println!("bestmove resign");
            app.comm
                .println(&format!("bestmove {}", best_logical_move.to_sign()));

            let best_rnote_opes =
                UsiPlayer::convert_move(best_logical_move, &position, recorder.ply);
            for rnote_ope in best_rnote_opes {
                app.comm.println("lib.rs:go: touch_brandnew_note");
                RpmNotePlayer::touch_brandnew_note(
                    &rnote_ope,
                    &mut position,
                    &mut recorder,
                    &app.comm,
                );
            }
        } else if line.starts_with("gameover") {
            // TODO lose とか win とか。

            tape_box_conveyor.write_cassette_tape_box(
                &app.kw29_conf,
                position.get_board_size(),
                &recorder.cassette_tape,
                &app.comm,
            );

        // #####
        // # H #
        // #####
        } else if line == "hand1" {
            // TODO 先手の持ち駒を表示。
            app.comm.println(&position.to_hand_text(Some(Phase::First)));
        } else if line == "hand2" {
            // TODO 後手の持ち駒を表示。
            app.comm
                .println(&position.to_hand_text(Some(Phase::Second)));
        } else if line == "hand3" {
            // TODO 使っていない駒を表示。
            app.comm.println(&position.to_hand_text(None));

        // #####
        // # I #
        // #####
        } else if line == "isready" {
            app.comm.println("readyok");

        // #########
        // # Piece #
        // #########
        } else if line.starts_with('B')
            | line.starts_with('G')
            | line.starts_with('K')
            | line.starts_with('L')
            | line.starts_with('N')
            | line.starts_with('P')
            | line.starts_with('S')
            | line.starts_with('R')
        {
            recorder.read_tape(&app.comm, &line, &mut position);

        // #####
        // # P #
        // #####
        } else if line.starts_with("position") {
            // 相手が指したあとの局面まで進める。
            let mut urecord_opt = None;
            let mut start = 0;

            //comm.println("#Lib: 'position' command(1).");
            if Fen::parse_initial_position(
                &app.comm,
                &line,
                &mut start,
                &mut recorder,
                &mut position,
            ) {
                urecord_opt = UsiPosition::parse_usi_line_moves(
                    &app.comm,
                    &line,
                    &mut start,
                    position.get_board_size(),
                );
            }
            //comm.println("#Position parse end1.");
            //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);

            // USI -> RPM 変換を作れていないので、ポジションをもう１回初期局面に戻してから、プレイアウトします。
            // TODO できれば USI -> RPM 変換したい。
            // comm.println("#Lib: TODO 'position' command(2).");
            {
                //comm.println("#Lib: 'position' command(2).");
                let mut start = 0;
                if Fen::parse_initial_position(
                    &app.comm,
                    &line,
                    &mut start,
                    &mut recorder,
                    &mut position,
                ) {
                    //comm.println("#Position parsed.");
                }

                if let Some(urecord) = urecord_opt {
                    // 差し替え。
                    recorder.clear_recorder();
                    UsiPlayer::play_out_and_record(
                        &mut position,
                        &urecord,
                        &mut recorder,
                        &app.comm,
                    );
                }
                //comm.println("#Record converted1.");
                //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);
                //comm.println("#Record converted2.");
            }

        // #####
        // # Q #
        // #####
        } else if line == "quit" {
            break;

        // #####
        // # U #
        // #####
        } else if line == "usi" {
            app.comm.println("id name Kifuwarabe Build.18");
            app.comm.println("id author Satoshi TAKAHASHI");
            app.comm.println("usiok");
        } else if line == "usinewgame" {
        }
    }
}
