use std::io;

mod position;
use position::Position;
mod thought;
use thought::Thought;

/// Computer shogi engine Kifuwarabe for WCSC29.
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
            println!("id name Kifuwarabe WCSC29");
            println!("id author Satoshi TAKAHASHI");
            println!("usiok");
        } else if line == "isready" {
            println!("readyok");
        } else if line == "usinewgame" {
        } else if line.starts_with("position") {
            println!("info What is position?");
            position.parse(&line);
        } else if line.starts_with("go") {
            let thought = Thought::new();
            println!("{}", thought.think());
            // println!("bestmove resign");
        }
    }
}
