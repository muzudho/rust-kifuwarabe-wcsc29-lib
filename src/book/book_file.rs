use communication::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
use position::*;
use rpm_conv::rpm_operation_track::*;

pub struct Book {
    lines: Vec<String>,
}
impl Book {
    pub fn new() -> Book {
        Book {
            lines: Vec::new(),
        }
    }

    pub fn read(&mut self) {
        for result in BufReader::new(File::open("./book/book.txt").unwrap()).lines() {
            let line = result.unwrap();
            println!("Read line: `{}`.", line);
            self.lines.push(line);
        }
    }

    /// 物理レコードを追加する。
    pub fn save_rpm_o_track(&self, comm:&Communication, board_size:BoardSize, rpm_o_track:&RpmOTrack) {
        comm.println("#Book saving...");
        let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("./book/book.txt")
        .unwrap();

        if let Err(e) = writeln!(file, "{}", rpm_o_track.to_sign(board_size)) {
            eprintln!("Couldn't write to file: {}", e);
        }
        comm.println("#Book saved.");
    }
}