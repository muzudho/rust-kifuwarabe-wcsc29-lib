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

use rand::Rng;
use std::io;
use std::path::Path;

pub mod address;
pub mod board_size;
pub mod communication;
pub mod conf;
pub mod csa_conv;
pub mod human;
pub mod kif_conv;
pub mod learn;
pub mod parser;
pub mod piece_etc;
pub mod position;
pub mod rpm_conv;
pub mod rpm_for_json;
pub mod rpm_play;
pub mod thought;
pub mod usi_conv;

use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use conf::kifuwarabe_wcsc29_lib_config::*;
use human::human_interface::*;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_cassette_tape_recorder::*;
use rpm_conv::rpm_object_sheet::*;
use rpm_play::rpm_move_player::*;
use rpm_play::rpm_note_player::*;
use thought::best_move_picker::*;
use usi_conv::fen::*;
use usi_conv::usi_player::*;
use usi_conv::usi_position::*;

pub fn main_loop() {
    // Logging.
    let comm = Communication::new();

    // Config.
    let my_config = KifuwarabeWcsc29LibConfig::load();
    let kw29_config = KifuwarabeWcsc29Config::load(&my_config);

    // ファイル名をランダムに作成する。
    let rpm_object_sheet_path;
    {
        let mut rng = rand::thread_rng();
        let rand1: i64 = rng.gen();
        let rand2: i64 = rng.gen();
        let rand3: i64 = rng.gen();
        let rand4: i64 = rng.gen();
        rpm_object_sheet_path = Path::new(&kw29_config.learning)
            .join(format!(
                "{}-{}-{}-{}-learning.rpmove",
                rand1, rand2, rand3, rand4
            ))
            .to_str()
            .unwrap()
            .to_string();
    }

    // 対局中の棋譜を入れる。
    let rpm_object_sheet = RpmObjectSheet::default(&rpm_object_sheet_path);
    let mut recorder = RpmCassetteTapeRecorder::default();

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
            recorder.read_tape(&comm, &line, &mut position);

        // #####
        // # B #
        // #####
        } else if line == "b" {
            // Back 1mark.
            if let Some(rnote) = recorder.cassette_tape.back_note() {
                RpmNotePlayer::back_1note(&rnote, &mut position, recorder.ply, &comm);
                HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
            }
        } else if line == "bb" {
            // Back 1ply.
            RpmMovePlayer::back_1move_on_tape(
                &mut recorder.cassette_tape,
                &mut position,
                recorder.ply,
                true,
                &comm,
            );
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "bbb" {
            // Back 10ply.
            for _i in 0..10 {
                RpmMovePlayer::back_1move_on_tape(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &comm,
                );
            }
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "bbbb" {
            // Back 400ply.
            for _i in 0..400 {
                RpmMovePlayer::back_1move_on_tape(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &comm,
                );
            }
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line.starts_with("bo") {
            // Board.
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);

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
            RpmNotePlayer::pop_current_1note_on_record(&mut recorder, &mut position, &comm);
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "dd" {
            // Delete 1ply.
            RpmMovePlayer::pop_current_1move_on_record(&mut recorder, &mut position, &comm);
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "ddd" {
            // Delete 10ply.
            for _i in 0..10 {
                RpmMovePlayer::pop_current_1move_on_record(&mut recorder, &mut position, &comm);
            }
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "dddd" {
            // Delete 400ply.
            for _i in 0..400 {
                RpmMovePlayer::pop_current_1move_on_record(&mut recorder, &mut position, &comm);
            }
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);

        // #####
        // # G #
        // #####
        } else if line.starts_with("go") {
            let best_logical_move =
                best_move_picker.get_best_move(&comm, &kw29_config, &mut recorder, &mut position);
            // Examples.
            // println!("bestmove 7g7f");
            // println!("bestmove win");
            // println!("bestmove resign");
            comm.println(&format!("bestmove {}", best_logical_move.to_sign()));

            let best_rnote_opes =
                UsiPlayer::convert_move(best_logical_move, &position, recorder.ply);
            for rnote_ope in best_rnote_opes {
                comm.println("lib.rs:go: touch_brandnew_note");
                RpmNotePlayer::touch_brandnew_note(&mut recorder, &rnote_ope, &mut position, &comm);
            }
        } else if line.starts_with("gameover") {
            // TODO lose とか win とか。

            rpm_object_sheet.append_record(&comm, position.get_board_size(), &recorder);

        // #####
        // # H #
        // #####
        } else if line == "hand1" {
            // TODO 先手の持ち駒を表示。
            comm.println(&position.to_hand_text(Some(Phase::First)));
        } else if line == "hand2" {
            // TODO 後手の持ち駒を表示。
            comm.println(&position.to_hand_text(Some(Phase::Second)));
        } else if line == "hand3" {
            // TODO 使っていない駒を表示。
            comm.println(&position.to_hand_text(None));

        // #####
        // # I #
        // #####
        } else if line == "isready" {
            comm.println("readyok");

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
            recorder.read_tape(&comm, &line, &mut position);

        // #####
        // # N #
        // #####
        } else if line == "n" {
            // Forward 1note.
            if let Some(rnote) = recorder.cassette_tape.next_note() {
                RpmNotePlayer::next_1note(&rnote, &mut position, recorder.ply, &comm);
                HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
            }
        } else if line == "nn" {
            // Forward 1move. （非合法タッチは自動で戻します）
            RpmMovePlayer::next_1move_on_tape(
                &mut recorder.cassette_tape,
                &mut position,
                recorder.ply,
                true,
                &comm,
            );
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "nnn" {
            // Forward 10move.
            for _i in 0..10 {
                RpmMovePlayer::next_1move_on_tape(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &comm,
                );
            }
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);
        } else if line == "nnnn" {
            // Forward 400move.
            for _i in 0..400 {
                RpmMovePlayer::next_1move_on_tape(
                    &mut recorder.cassette_tape,
                    &mut position,
                    recorder.ply,
                    true,
                    &comm,
                );
            }
            HumanInterface::bo(&comm, &recorder.cassette_tape, recorder.ply, &position);

        // #####
        // # Q #
        // #####
        } else if line == "quit" {
            break;

        // #####
        // # U #
        // #####
        } else if line == "usi" {
            comm.println("id name Kifuwarabe Build.18");
            comm.println("id author Satoshi TAKAHASHI");
            comm.println("usiok");
        } else if line == "usinewgame" {
            // #####
            // # P #
            // #####
        } else if line.starts_with("position") {
            // 相手が指したあとの局面まで進める。
            let mut urecord_opt = None;
            let mut start = 0;

            //comm.println("#Lib: 'position' command(1).");
            if Fen::parse_position(&comm, &line, &mut start, &mut recorder, &mut position) {
                urecord_opt = UsiPosition::parse_usi_line_moves(
                    &comm,
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
                if Fen::parse_position(&comm, &line, &mut start, &mut recorder, &mut position) {
                    //comm.println("#Position parsed.");
                }

                if let Some(urecord) = urecord_opt {
                    // 差し替え。
                    recorder = UsiPlayer::play_out_and_record(&comm, &mut position, &urecord);
                }
                //comm.println("#Record converted1.");
                //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);
                //comm.println("#Record converted2.");
            }
        }
    }
}
