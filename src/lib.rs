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
///
/// extern crate は main.rs か lib.rs に入れる。
/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate chrono;
extern crate getopts;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;
pub mod conf;
pub mod human;
pub mod instrument;
pub mod lib_sub;
pub mod live;
pub mod musician;
pub mod sheet_music_format;
pub mod sound;
pub mod studio;
pub mod video_recorder;
use human::human_interface::*;
use instrument::game_player::*;
use instrument::position::*;
use lib_sub::*;
use live::best_move_picker::*;
use std::io;
use studio::application::*;
use video_recorder::cassette_deck::*;

pub fn main_loop() {
    // The application contains all immutable content.
    let app = Application::new();

    // Position.
    let mut position = Position::new_honshogi_origin();

    // Deck.
    let mut deck = CassetteDeck::new_change(None, position.get_board_size(), &app);

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

            HumanInterface::bo(&mut deck, Slot::Learning, &position, &app);

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
            deck.pop_1note(Slot::Learning, &mut position, &app);

            HumanInterface::bo(&mut deck, Slot::Learning, &position, &app);
        } else if line == "dd" {
            // Delete 1ply.
            deck.pop_1move(Slot::Learning, &mut position, &app);

            HumanInterface::bo(&mut deck, Slot::Learning, &position, &app);
        } else if line == "ddd" {
            // Delete 10ply.
            for _i in 0..10 {
                deck.pop_1move(Slot::Learning, &mut position, &app);
            }

            HumanInterface::bo(&mut deck, Slot::Learning, &position, &app);
        } else if line == "dddd" {
            // Delete 400ply.
            for _i in 0..400 {
                deck.pop_1move(Slot::Learning, &mut position, &app);
            }

            HumanInterface::bo(&mut deck, Slot::Learning, &position, &app);

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
            LibSub::position(line, &mut position, &mut deck, &app);

        // #####
        // # Q #
        // #####
        } else if line == "quit" {
            break;

        // #####
        // # U #
        // #####
        } else if line == "usi" {
            app.comm.println("id name Kifuwarabe Build.20");
            app.comm.println("id author Satoshi TAKAHASHI");
            app.comm.println("usiok");
        } else if line == "usinewgame" {
        }
    }
}
