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
pub mod kifu_usi;
pub mod learn;
pub mod lib_sub;
pub mod object_rpm;
pub mod parser;
pub mod piece_etc;
pub mod shogi_ban;
pub mod thought;
use application::*;
use human::human_interface::*;
use kifu_usi::fen::*;
use kifu_usi::usi_converter::*;
use kifu_usi::usi_position::*;
use lib_sub::*;
use object_rpm::cassette_deck::*;
use shogi_ban::game_player::*;
use shogi_ban::position::*;
use std::io;
use thought::best_move_picker::*;

pub fn main_loop() {
    // The application contains all immutable content.
    let app = Application::new();

    // Record.
    let mut deck = CassetteDeck::new_empty();

    let mut position = Position::default();
    let mut best_move_picker = BestMovePicker::default();

    loop {
        // Standard input.
        // Be sure to add "info" before the output message.
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("info Failed: stdin.read_line.");

        // Excludes trailing newlines. The surrounding whitespace delete_1notes.
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
            GamePlayer::read_ope_track(&line, &mut position, &mut deck, &app);

        // #####
        // # B #
        // #####
        } else if line == "b" {
            LibSub::back_1_note(&mut position, &mut deck, &app);
        } else if line == "bb" {
            LibSub::back_1_move(&mut position, &mut deck, &app);
        } else if line == "bbb" {
            LibSub::back_10_move(&mut position, &mut deck, &app);
        } else if line == "bbbb" {
            LibSub::back_400_move(&mut position, &mut deck, &app);
        } else if line.starts_with("bo") {
            // Board.

            HumanInterface::bo(&mut deck, &position, &app);

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
            deck.pop_1note(&mut position, &app);

            HumanInterface::bo(&mut deck, &position, &app);
        } else if line == "dd" {
            // Delete 1ply.
            deck.pop_1move(&mut position, &app);

            HumanInterface::bo(&mut deck, &position, &app);
        } else if line == "ddd" {
            // Delete 10ply.
            for _i in 0..10 {
                deck.pop_1move(&mut position, &app);
            }

            HumanInterface::bo(&mut deck, &position, &app);
        } else if line == "dddd" {
            // Delete 400ply.
            for _i in 0..400 {
                deck.pop_1move(&mut position, &app);
            }

            HumanInterface::bo(&mut deck, &position, &app);

        // #####
        // # N #
        // #####
        } else if line == "f" {
            LibSub::forward_1_note(&mut position, &mut deck, &app);
        // Forward 1note.
        } else if line == "ff" {
            LibSub::forward_1_move(&mut position, &mut deck, &app);
        } else if line == "fff" {
            LibSub::forward_10_move(&mut position, &mut deck, &app);
        } else if line == "ffff" {
            LibSub::forward_400_move(&mut position, &mut deck, &app);

        // #####
        // # G #
        // #####
        } else if line.starts_with("go") {
            LibSub::go(&mut best_move_picker, &mut position, &mut deck, &app);
        } else if line.starts_with("gameover") {
            // TODO lose とか win とか。
            LibSub::gameover(position.get_board_size(), &mut deck, &app);
        // #####
        // # H #
        // #####
        } else if line == "hand1" {
            LibSub::hand1(&position, &app);
        } else if line == "hand2" {
            LibSub::hand2(&position, &app);
        } else if line == "hand3" {
            LibSub::hand3(&position, &app);

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
            GamePlayer::read_ope_track(&line, &mut position, &mut deck, &app);

        // #####
        // # P #
        // #####
        } else if line.starts_with("position") {
            // 相手が指したあとの局面まで進める。
            let mut urecord_opt = None;
            let mut start = 0;

            //comm.println("#Lib: 'position' command(1).");
            if Fen::parse_initial_position(&line, &mut start, &mut position, &mut deck, &app) {
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
                if Fen::parse_initial_position(&line, &mut start, &mut position, &mut deck, &app) {
                    //comm.println("#Position parsed.");
                }

                if let Some(urecord) = urecord_opt {
                    // 差し替え。
                    deck.change(None, position.get_board_size(), &app);
                    UsiConverter::play_out_usi_tape(&mut position, &urecord, &mut deck, &app);
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
            app.comm.println("id name Kifuwarabe Build.19");
            app.comm.println("id author Satoshi TAKAHASHI");
            app.comm.println("usiok");
        } else if line == "usinewgame" {
        }
    }
}
