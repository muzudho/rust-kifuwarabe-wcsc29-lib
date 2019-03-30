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
pub mod csa_conv;
pub mod parser;
pub mod physical_move;
pub mod physical_record;
pub mod piece_etc;
pub mod position;
pub mod usi_conv;
pub mod thought;

use address::*;
use common_operation::*;
use communication::*;
use usi_conv::fen::*;
use usi_conv::usi_record::*;
use physical_move::*;
use physical_record::*;
use piece_etc::*;
use parser::*;
use position::*;
use usi_conv::usi_converter::*;
use thought::Thought;

pub fn main_loop() {
    let comm = Communication::new();
    let mut physical_record = PhysicalRecord::new();
    let mut position = Position::default();

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
        if line.starts_with('1') || 
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
            read_tape(&comm, &line, &mut physical_record, &mut position);

        // #####
        // # B #
        // #####
        } else if line.starts_with("bo") {
            // board.
            CommonOperation::bo(&comm, &physical_record, &position);

        // #####
        // # D #
        // #####
        } else if line == "d" {
            // Delete.
            CommonOperation::detouch(&comm, &mut physical_record, &mut position);

        // #####
        // # G #
        // #####
        } else if line.starts_with("go") {
            let thought = Thought::new();
            let best_logical_move = thought.get_best_move(
                &position,
                &mut physical_record);
            // Examples.
            // println!("bestmove 7g7f");
            // println!("bestmove win");
            // println!("bestmove resign");
            comm.println(&format!("bestmove {}", best_logical_move.to_sign()));

            let best_physical_moves = UsiConverter::convert_move(
                best_logical_move,
                &position);
            for physical_move in best_physical_moves {
                CommonOperation::go(&comm, &mut physical_record, &physical_move, &mut position);
            }
            
        // #####
        // # H #
        // #####

        } else if line == "hand1" {
            // TODO 先手の持ち駒を表示。

        } else if line == "hand2" {
            // TODO 後手の持ち駒を表示。

        } else if line == "hand3" {
            // TODO 使っていない駒を表示。

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
            read_tape(&comm, &line, &mut physical_record, &mut position);

        // #####
        // # Q #
        // #####
        } else if line == "quit" {
            break;

        // #####
        // # U #
        // #####
        } else if line == "usi" {
            comm.println("id name Kifuwarabe Build.12");
            comm.println("id author Satoshi TAKAHASHI");
            comm.println("usiok");
        } else if line == "usinewgame" {
        // #####
        // # P #
        // #####
        } else if line.starts_with("position") {
            let mut logical_record = UsiRecord::new();
            let mut start = 0;
            if Fen::parse_position(&line, &mut start, &mut position) {
                if let Some(lrecords) = Fen::parse_moves(&comm, &line, &mut start, &mut position) {
                    logical_record = lrecords;
                };
            }

            UsiConverter::convert_record(
                &comm,
                &mut position,
                &logical_record,
                &mut physical_record);
        }
    }
}

/// 
fn read_tape(comm:&Communication, line:&str, physical_record:&mut PhysicalRecord, position:&mut Position) {
    let mut start = 0;

    loop {
        if line.len() <= start {
            return;
        }

        let ch1 = line[start..=start].chars().nth(0).unwrap();
        let pmove_opt = match ch1 {
            ' ' => {
                comm.print(&ch1.to_string());
                start += 1;
                None
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // セル
                start += 1;
                let ch2 = line[start..=start].chars().nth(0).unwrap();
                start += 1;
                comm.print(&format!("{}{}", ch1, ch2));
                let file = Parser::file_char_to_i8(ch1);
                let rank = Parser::rank_char_to_i8(ch2);
                let address = Address::create_by_cell(file, rank, position.get_board_size());
                Some(PhysicalMove::create_by_address(address))
            },
            '+' => {
                // 成り。
                comm.print(&ch1.to_string());
                start += 1;
                Some(PhysicalMove::turn_over())
            },
            '-' => {
                // １８０°回転。
                comm.print(&ch1.to_string());
                start += 1;
                Some(PhysicalMove::rotate())
            },
            '|' => {
                // フェーズ交代。
                comm.print(&ch1.to_string());
                start += 1;
                Some(PhysicalMove::change_phase())
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
                Some(PhysicalMove::change_phase())
            },
            'K' | 'k' |
            'R' | 'r' |
            'B' | 'b' |
            'G' | 'g' |
            'S' | 's' |
            'N' | 'n' |
            'L' | 'l' |
            'P' | 'p' => {
                // ドロップ。
                start += 1;
                let ch2 = line[start..=start].chars().nth(0).unwrap();
                start += 1;
                if ch2 != '*' {
                    panic!("Unexpected drop '{}'.", line)
                };
                comm.print(&format!("{}{}", ch1, ch2));
                let piece_type = sign_to_piece_type(&ch1.to_string());
                let address = Address::create_by_hand(Some(position.get_phase()), piece_type);
                comm.println(&format!("address index = {}.", address.get_index()));
                Some(PhysicalMove::create_by_address(address))
            },
            _ => {
                let last = line.len();
                panic!("Unexpected line '{}'.", &line[start..last]);
            }
        };

        if let Some(pmove) = pmove_opt {
            CommonOperation::touch(comm, physical_record, &pmove, position);
        }
    }
}
