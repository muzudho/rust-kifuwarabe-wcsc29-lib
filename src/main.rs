use std::io;

mod communication;
mod record;
mod position;
mod thought;

use communication::*;
use position::Position;
use thought::Thought;

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
/// cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29
/// cls
/// 
/// ### Compile.
/// cargo clippy
/// 
/// ### Run.
/// cargo run --release
/// ```
/// 
/// Execution file.
/// C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/rust-kifuwarabe-wcsc29.exe
fn main() {

    let mut comm = Communication::new();
    let mut position = Position::new();

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

        if line == "quit" {
            break;
        } else if line == "usi" {
            comm.println("id name Kifuwarabe Build.8");
            comm.println("id author Satoshi TAKAHASHI");
            comm.println("usiok");
        } else if line == "isready" {
            comm.println("readyok");
        } else if line == "usinewgame" {
        } else if line.starts_with("position") {
            comm.println("info What is position?");
            position.parse(&line);
            position.show_board();
        } else if line.starts_with("go") {
            let thought = Thought::new();
            comm.println(&format!("bestmove {}", thought.get_best_move(&mut position).to_sign()));
            // Examples.
            // println!("bestmove 7g7f");
            // println!("bestmove win");
            // println!("bestmove resign");
        }
    }
}
