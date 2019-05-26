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
pub mod audio_compo;
pub mod conf;
pub mod conv;
pub mod human;
pub mod instrument;
pub mod lib_sub;
pub mod live;
pub mod media;
pub mod musician;
pub mod sheet_music_format;
pub mod sound;
pub mod studio;
use audio_compo::audio_rack::*;
use audio_compo::cassette_deck::*;
use human::human_interface::*;
use instrument::position::*;
use lib_sub::*;
use live::base_performer::*;
use live::best_move_picker::*;
use live::computer_performer::*;
use live::ohashi_performer::*;
use live::tuner::*;
use std::io;
use studio::application::*;
use studio::common::closed_interval::*;

pub fn main_loop() {
    // The application contains all immutable content.
    // 大会用フラグをこっそっといれたのでミュータブル☆（*＾～＾*）
    let mut app = Application::new();

    // Position.
    let mut position = Position::new_honshogi_origin(&app);

    // Audio rack.
    let mut rack = AudioRack::new(&app);

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
        app.comm.logln(&format!("------> {}", line));

        // #############
        // # 重要なやつ #
        // #############
        if line == "kw" {
            if app.kifuwarabe_flag {
                app.kifuwarabe_flag = false;
            // app.comm.println("Debug off!");
            } else {
                app.kifuwarabe_flag = true;
                app.comm.activate_standard_output(true);
                app.comm.println("Debug on!");
            }

        // ######
        // # 数 #
        // ######
        // ########
        // # 記号 #
        // ########
        } else if line.starts_with('0')
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
            BasePerformer::improvise_by_line(&line, &mut rack, true, &mut position, &app);

        // #####
        // # B #
        // #####
        } else if line == "b" {
            Tuner::back_1_note(&mut rack, &mut position, &app);
        } else if line == "bb" {
            Tuner::back_1_move(&mut rack, &mut position, &app);
        } else if line == "bbb" {
            Tuner::back_10_move(&mut rack, &mut position, &app);
        } else if line == "bbbb" {
            Tuner::back_400_move(&mut rack, &mut position, &app);
        } else if line.starts_with("bo") {
            // Board.

            HumanInterface::bo(&rack, &position, &app);

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
            BasePerformer::rollback_note(&mut rack, Slot::Training, &mut position, &app);
        /*
        } else if line == "dd" {
            // Delete 1ply.
            BasePerformer::delete_1move(&mut rack, &mut position, &app);
            HumanInterface::bo(&rack, &position, &app);
        } else if line == "ddd" {
            // Delete 10ply.
            for _i in 0..10 {
                BasePerformer::delete_1move(&mut rack, &mut position, &app);
            }

            HumanInterface::bo(&rack, &position, &app);
        } else if line == "dddd" {
            // Delete 400ply.
            for _i in 0..400 {
                BasePerformer::delete_1move(&mut rack, &mut position, &app);
            }

            HumanInterface::bo(&rack, &position, &app);
            */
        } else if line == "rack-info" {
            app.comm.println(&rack.to_human_presentable());

        // #####
        // # F #
        // #####
        } else if line == "f" {
            Tuner::replay_a_note(&mut rack, &mut position, &app);
        } else if line == "ff" {
            Tuner::forward_1_move(&mut rack, &mut position, &app);
        } else if line == "fff" {
            Tuner::forward_10_move(&mut rack, &mut position, &app);
        } else if line == "ffff" {
            Tuner::forward_400_move(&mut rack, &mut position, &app);

        // #####
        // # G #
        // #####
        } else if line.starts_with("go") {
            ComputerPerformer::go(&mut best_move_picker, &mut rack, &mut position, &app);
        } else if line.starts_with("gameover") {
            // TODO lose とか win とか。
            LibSub::gameover(&mut rack, position.get_board_size(), &app);
        // #####
        // # H #
        // #####
        } else if line == "hand1" {
            LibSub::hand1(&position, &app);
        } else if line == "hand2" {
            LibSub::hand2(&position, &app);
        } else if line == "hand3" {
            LibSub::hand3(&position, &app);
        } else if line == "heikukan" {
            // 閉区間のテスト☆（＾～＾）
            let mut v: Vec<(i16, i16, i16)> = Vec::new();
            // min, max, expected len.
            v.push((0, 0, 1));
            v.push((-1, 0, 2));
            v.push((0, 1, 2));
            v.push((0, -1, 0));
            for e in v {
                let ans = ClosedInterval::from_all(e.0, e.1, true);
                app.comm.println(&format!(
                    "min {:>2}, max {:>2}. len {:>2}, empty {:>5}, expected {:>2}. msg: {}.",
                    e.0,
                    e.1,
                    ans.len(),
                    ans.is_empty(),
                    e.2,
                    ans.to_human_presentable()
                ));
            }

        // #####
        // # I #
        // #####
        } else if line == "isready" {
            app.comm.println("readyok");

        // #####
        // # L #
        // #####
        } else if line == "lbl" {
            // Look back learing
            LibSub::look_back(&mut rack, Slot::Learning, &app);
        } else if line == "lbt" {
            // Look back training
            LibSub::look_back(&mut rack, Slot::Training, &app);

        // #####
        // # O #
        // #####
        } else if line == "ohashi" {
            // Ohashi mode.
            OhashiPerformer::improvise_ohashi_starting(&mut rack, &mut position, &app)

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
            BasePerformer::improvise_by_line(&line, &mut rack, true, &mut position, &app);

        // #####
        // # P #
        // #####
        } else if line.starts_with("position") {
            // 相手が指したあとの局面まで進める。
            LibSub::position(line, &mut rack, &mut position, &app);

        // #####
        // # Q #
        // #####
        } else if line == "quit" {
            break;

        // #####
        // # S #
        // #####
        } else if line.starts_with("scan-pid") {
            BasePerformer::scan_pid(&line, &mut rack, &mut position, &app);

        // #####
        // # T #
        // #####
        } else if line == "tape-t" {
            if !rack.is_none_current_tape(Slot::Training) {
                HumanInterface::show_tape(&rack, Slot::Training, &position, &app);
            } else {
                app.comm.println("スロットt が空っぽです。");
            }
        } else if line == "tape-l" {
            if !rack.is_none_current_tape(Slot::Learning) {
                HumanInterface::show_tape(&rack, Slot::Learning, &position, &app);
            } else {
                app.comm.println("スロットl が空っぽです。");
            }
        } else if line.starts_with("test-2heads-vec") {
            LibSub::test_2heads_vec(position.get_board_size(), &app);

        // #####
        // # U #
        // #####
        } else if line == "usi" {
            app.comm.activate_standard_output(true);
            app.comm.println("id name Kifuwarabe Build.24");
            app.comm.println("id author Satoshi TAKAHASHI");
            app.comm.println("usiok");
        } else if line == "usinewgame" {
            LibSub::usi_new_game(&mut rack, &app);
        }
    }
}
