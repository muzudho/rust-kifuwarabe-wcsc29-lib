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
extern crate serde;
extern crate serde_json;
extern crate regex;
extern crate getopts;

use rand::Rng;
use std::io;

pub mod address;
pub mod board_size;
pub mod communication;
pub mod csa_conv;
pub mod human;
pub mod kif_conv;
pub mod learn;
pub mod conf;
pub mod parser;
pub mod piece_etc;
pub mod position;
pub mod rpm_conv;
pub mod rpm_for_json;
pub mod rpm_play;
pub mod usi_conv;
pub mod thought;

use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use conf::kifuwarabe_wcsc29_lib_config::*;
use human::human_interface::*;
use rpm_conv::rpm_record::*;
use rpm_conv::rpm_object_sheet::*;
use rpm_play::rpm_move_player::*;
use rpm_play::rpm_note_player::*;
use std::path::Path;
use usi_conv::fen::*;
use usi_conv::usi_position::*;
use piece_etc::*;
use position::*;
use usi_conv::usi_player::*;
use thought::best_move_picker::*;

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
        rpm_object_sheet_path = Path::new(&kw29_config.learning).join(format!("{}-{}-{}-{}-learning.rpmove", rand1, rand2, rand3, rand4)).to_str().unwrap().to_string();
    }

    // 学習中の棋譜を入れる。
    let rpm_object_sheet = RpmObjectSheet::default(&rpm_object_sheet_path);
    let mut rrecord = RpmRecord::default();

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
        line = line.trim()
            .parse()
            .expect("info Failed: stdin parse.");

        // ######
        // # 数 #
        // ######
        // ########
        // # 記号 #
        // ########

        if line.starts_with('0') || 
            line.starts_with('1') || 
            line.starts_with('2') ||
            line.starts_with('3') ||
            line.starts_with('4') ||
            line.starts_with('5') ||
            line.starts_with('6') ||
            line.starts_with('7') ||
            line.starts_with('8') ||
            line.starts_with('9') ||
            line.starts_with('+') ||
            line.starts_with('-') ||
            line.starts_with('|')
        {
            RpmRecord::read_tape(&comm, &line, &mut rrecord, &mut position);

        // #####
        // # B #
        // #####

        } else if line == "b" {
            // Back 1mark.
            RpmNotePlayer::back_1note_on_record(&comm, &mut rrecord, &mut position);
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "bb" {
            // Back 1ply.
            RpmMovePlayer::back_1move_on_record(&comm, &mut rrecord, &mut position);
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "bbb" {
            // Back 10ply.
            for _i in 0..10 {
                RpmMovePlayer::back_1move_on_record(&comm, &mut rrecord, &mut position);
            }
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "bbbb" {
            // Back 400ply.
            for _i in 0..400 {
                RpmMovePlayer::back_1move_on_record(&comm, &mut rrecord, &mut position);
            }
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line.starts_with("bo") {
            // Board.
            HumanInterface::bo(&comm, &rrecord, &position);

        // #####
        // # D #
        // #####

        } else if line == "d" {
            // Delete 1mark.
            RpmNotePlayer::pop_current_1note_on_record(&comm, &mut rrecord, &mut position);
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "dd" {
            // Delete 1ply.
            RpmMovePlayer::pop_current_1move_on_record(&comm, &mut rrecord, &mut position);
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "ddd" {
            // Delete 10ply.
            for _i in 0..10 {
                RpmMovePlayer::pop_current_1move_on_record(&comm, &mut rrecord, &mut position);
            }
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "dddd" {
            // Delete 400ply.
            for _i in 0..400 {
                RpmMovePlayer::pop_current_1move_on_record(&comm, &mut rrecord, &mut position);
            }
            HumanInterface::bo(&comm, &rrecord, &position);

        // #####
        // # F #
        // #####

        } else if line == "f" {
            // Forward 1mark.
            RpmNotePlayer::forward_1note_on_record(&comm, &mut rrecord, &mut position);
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "ff" {
            // Forward 1ply.
            RpmMovePlayer::forward_1move_on_record(&comm, &mut rrecord, &mut position);
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "fff" {
            // Forward 10ply.
            for _i in 0..10 {
                RpmMovePlayer::forward_1move_on_record(&comm, &mut rrecord, &mut position);
            }
            HumanInterface::bo(&comm, &rrecord, &position);

        } else if line == "ffff" {
            // Forward 400ply.
            for _i in 0..400 {
                RpmMovePlayer::forward_1move_on_record(&comm, &mut rrecord, &mut position);
            }
            HumanInterface::bo(&comm, &rrecord, &position);

        // #####
        // # G #
        // #####

        } else if line.starts_with("go") {
            let best_logical_move = best_move_picker.get_best_move(&comm, &kw29_config, &position);
            // Examples.
            // println!("bestmove 7g7f");
            // println!("bestmove win");
            // println!("bestmove resign");
            comm.println(&format!("bestmove {}", best_logical_move.to_sign()));

            let best_rpm_operation_move = UsiConverter::convert_move(
                best_logical_move,
                &position);
            for rpm_operation_note in best_rpm_operation_move {
                RpmNotePlayer::touch_brandnew_note(&comm, &mut rrecord.body.rpm_tape, &rpm_operation_note, &mut position);
            }

        } else if line.starts_with("gameover") {
            // TODO lose とか win とか。

            rpm_object_sheet.append(&comm, position.get_board_size(), &rrecord);

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
        } else if line.starts_with('B') | line.starts_with('G') | line.starts_with('K') | line.starts_with('L') |
            line.starts_with('N') | line.starts_with('P') | line.starts_with('S') | line.starts_with('R') {

            RpmRecord::read_tape(&comm, &line, &mut rrecord, &mut position);

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
            if Fen::parse_position(&comm, &line, &mut start, &mut rrecord, &mut position) {
                urecord_opt = UsiPosition::parse_usi_line_moves(&comm, &line, &mut start, position.get_board_size());
            }
            //comm.println("#Position parse end1.");
            //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);

            // USI -> RPM 変換を作れていないので、ポジションをもう１回初期局面に戻してから、プレイアウトします。
            // TODO できれば USI -> RPM 変換したい。
            // comm.println("#Lib: TODO 'position' command(2).");
            {
                //comm.println("#Lib: 'position' command(2).");
                let mut start = 0;
                if Fen::parse_position(&comm, &line, &mut start, &mut rrecord, &mut position) {
                    //comm.println("#Position parsed.");
                }

                if let Some(urecord) = urecord_opt {
                    UsiConverter::play_out_record(
                        &comm,
                        &mut position,
                        &urecord,
                        &mut rrecord);
                }
                //comm.println("#Record converted1.");
                //HumanInterface::bo(&comm, &rrecord.get_mut_operation_track(), &position);
                //comm.println("#Record converted2.");
            }
        }
    }
}
