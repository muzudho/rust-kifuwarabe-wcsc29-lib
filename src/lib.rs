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
extern crate serde_json;

use std::io;

pub mod address;
pub mod common_operation;
pub mod communication;
pub mod config_file;
pub mod csa_conv;
pub mod learn;
pub mod parser;
pub mod piece_etc;
pub mod position;
pub mod rpm_conv;
pub mod usi_conv;
pub mod thought;

use address::*;
use common_operation::*;
use communication::*;
use config_file::*;
use rpm_conv::rpm_operation_note::*;
use rpm_conv::rpm_operation_track::*;
use rpm_conv::rpm_record::*;
use rpm_conv::rpm_sheet::*;
use usi_conv::fen::*;
use usi_conv::usi_record::*;
use piece_etc::*;
use parser::*;
use position::*;
use usi_conv::usi_converter::*;
use thought::Thought;

pub fn main_loop() {
    // Logging.
    let comm = Communication::new();

    // Config.
    let config = &Config::load();
    // comm.println(&format!("my_record_directory: '{}'.", &config.get_my_record_directory()));

    let rpm_sheet = RpmSheet::new();
    let mut rpm_record = RpmRecord::default();

    let mut position = Position::default();
    let mut thought = Thought::new();
    thought.load();

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
            read_tape(&comm, &line, &mut rpm_record, &mut position);

        // #####
        // # B #
        // #####

        } else if line == "b" {
            // Back 1mark.
            CommonOperation::back_1mark(&comm, &mut rpm_record, &mut position);
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "bb" {
            // Back 1ply.
            CommonOperation::back_1ply(&comm, &mut rpm_record, &mut position);
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "bbb" {
            // Back 10ply.
            for _i in 0..10 {
                CommonOperation::back_1ply(&comm, &mut rpm_record, &mut position);
            }
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "bbbb" {
            // Back 400ply.
            for _i in 0..400 {
                CommonOperation::back_1ply(&comm, &mut rpm_record, &mut position);
            }
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line.starts_with("bo") {
            // Board.
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        // #####
        // # D #
        // #####

        } else if line == "d" {
            // Delete 1mark.
            CommonOperation::pop_current_1mark(&comm, &mut rpm_record.get_mut_operation_track(), &mut position);
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "dd" {
            // Delete 1ply.
            CommonOperation::pop_current_1ply(&comm, &mut rpm_record.get_mut_operation_track(), &mut position);
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "ddd" {
            // Delete 10ply.
            for _i in 0..10 {
                CommonOperation::pop_current_1ply(&comm, &mut rpm_record.get_mut_operation_track(), &mut position);
            }
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "dddd" {
            // Delete 400ply.
            for _i in 0..400 {
                CommonOperation::pop_current_1ply(&comm, &mut rpm_record.get_mut_operation_track(), &mut position);
            }
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        // #####
        // # F #
        // #####

        } else if line == "f" {
            // Forward 1mark.
            CommonOperation::forward_1mark(&comm, &mut rpm_record, &mut position);
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "ff" {
            // Forward 1ply.
            CommonOperation::forward_1ply(&comm, &mut rpm_record, &mut position);
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "fff" {
            // Forward 10ply.
            for _i in 0..10 {
                CommonOperation::forward_1ply(&comm, &mut rpm_record, &mut position);
            }
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        } else if line == "ffff" {
            // Forward 400ply.
            for _i in 0..400 {
                CommonOperation::forward_1ply(&comm, &mut rpm_record, &mut position);
            }
            CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);

        // #####
        // # G #
        // #####

        } else if line.starts_with("go") {
            let best_logical_move = thought.get_best_move(
                &position,
                &mut rpm_record.get_mut_operation_track());
            // Examples.
            // println!("bestmove 7g7f");
            // println!("bestmove win");
            // println!("bestmove resign");
            comm.println(&format!("bestmove {}", best_logical_move.to_sign()));

            let best_rpm_operation_move = UsiConverter::convert_move(
                best_logical_move,
                &position);
            for rpm_operation_note in best_rpm_operation_move {
                CommonOperation::go(&comm, &mut rpm_record, &rpm_operation_note, &mut position);
            }

        } else if line.starts_with("gameover") {
            // TODO lose とか win とか。

            // TODO 物理レコードを１行にして保存したい。
            //let dir = &config.get_my_record_directory();
            let dir = &config.my_record_directory;
            rpm_sheet.append(&comm, position.get_board_size(), &dir, &mut rpm_record);

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
            read_tape(&comm, &line, &mut rpm_record, &mut position);

        // #####
        // # Q #
        // #####
        } else if line == "quit" {
            break;

        // #####
        // # U #
        // #####
        } else if line == "usi" {
            comm.println("id name Kifuwarabe Build.16");
            comm.println("id author Satoshi TAKAHASHI");
            comm.println("usiok");
        } else if line == "usinewgame" {
        // #####
        // # P #
        // #####
        } else if line.starts_with("position") {
            // 相手が指したあとの局面まで進める。
            let mut urecord = UsiRecord::new();
            let mut start = 0;
            if Fen::parse_position(&comm, &line, &mut start, &mut position) {
                if let Some(parsed_urecord) = CommonOperation::read_usi_moves(&comm, &line, &mut start, &mut position) {
                    urecord = parsed_urecord;
                };
            }
            //comm.println("#Position parse end1.");
            //CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);
            //comm.println("#Position parse end2.");

            // ポジションをもう１回初期局面に戻す。
            let mut start = 0;
            if Fen::parse_position(&comm, &line, &mut start, &mut position) {
                //comm.println("#Position parsed.");
            }

            UsiConverter::convert_record(
                &comm,
                &mut position,
                &urecord,
                &mut rpm_record);
            //comm.println("#Record converted1.");
            //CommonOperation::bo(&comm, &rpm_record.get_mut_operation_track(), &position);
            //comm.println("#Record converted2.");
        }
    }
}

/// 
fn read_tape(comm:&Communication, line:&str, rpm_record:&mut RpmRecord, position:&mut Position) {
    let mut start = 0;

    loop {
        if line.len() <= start {
            return;
        }

        let ch1 = line[start..=start].chars().nth(0).unwrap();
        let rpm_note_opt = match ch1 {
            ' ' => {
                comm.print(&ch1.to_string());
                start += 1;
                None
            }
            '0' => {
                // 駒台。
                start += 1;

                let ch2 = line[start..=start].chars().nth(0).unwrap();
                start += 1;

                let text15;
                match ch2 {
                    'P' | 'p' | 'ﾅ' => {
                        // 成り駒は、不成駒と同じところに置くので、成りのマークは読み飛ばす。
                        text15 = line[start..=start].chars().nth(0).unwrap().to_string();
                        start += 1;
                    },
                    _ => {
                        // Ignored.
                        text15 = "".to_string();
                    },
                };

                // 駒の種類、フェーズ。
                let piece = PhysicalSign::default(ch2.to_string()).to_piece();

                comm.print(&format!("{}{}{}", ch1, text15, ch2));
                let address = Address::create_by_hand(
                    piece_to_phase(Some(piece)),
                    piece_to_piece_type(piece));
                comm.println(&format!("address index = {}.", address.get_index()));
                Some(RpmNote::create_by_address(address))
            },
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // セル
                start += 1;
                let ch2 = line[start..=start].chars().nth(0).unwrap();
                start += 1;
                comm.print(&format!("{}{}", ch1, ch2));
                let file = Parser::file_char_to_i8(ch1);
                let rank = Parser::rank_char_to_i8(ch2);
                let address = Address::create_by_cell(file, rank, position.get_board_size());
                Some(RpmNote::create_by_address(address))
            },
            '+' => {
                // 成り。
                comm.print(&ch1.to_string());
                start += 1;
                Some(RpmNote::turn_over())
            },
            '-' => {
                // １８０°回転。
                comm.print(&ch1.to_string());
                start += 1;
                Some(RpmNote::rotate())
            },
            '|' => {
                // フェーズ交代。
                comm.print(&ch1.to_string());
                start += 1;
                Some(RpmNote::change_phase())
            },
            '[' => {
                // フェーズ交代。 ']' まで読み飛ばす。
                comm.print(&ch1.to_string());
                start += 1;
                loop {
                    if line.len() <= start {
                        break;
                    }
                    
                    let sub_ch = line[start..=start].chars().nth(0).unwrap();
                    comm.print(&sub_ch.to_string());
                    start += 1;

                    if sub_ch == ']' {
                        break;
                    }
                };
                Some(RpmNote::change_phase())
            },
            _ => {
                let last = line.len();
                panic!("Unexpected line '{}'.", &line[start..last]);
            }
        };

        if let Some(rpm_note) = rpm_note_opt {
            CommonOperation::touch(comm, rpm_record, &rpm_note, position);
        }
    }
}
