use communication::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;
use position::*;
use rpm_conv::rpm_operation_track::*;
use rpm_conv::rpm_record::*;

pub struct RpmSheet {
    records: Vec<RpmRecord>,
}
impl RpmSheet {
    pub fn new() -> RpmSheet {
        RpmSheet {
            records: Vec::new(),
        }
    }

    pub fn read(&mut self, directory:&str) {
        for result in BufReader::new(File::open(Path::new(directory).join("sheet.txt")).unwrap()).lines() {
            let line = result.unwrap();
            println!("Read line: `{}`.", line);
            // TODO self.records.push(record);
        }
    }

    /// 物理レコードを追加する。
    pub fn append(&self, comm:&Communication, board_size:BoardSize, directory:&str, rpm_record:&RpmRecord) {
        comm.println("#Sheet saving...");
        let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(Path::new(directory).join("sheet.txt"))
        .unwrap();

        if let Err(e) = writeln!(file, "{}", rpm_record.to_sign(board_size)) {
            eprintln!("Couldn't write to file: {}", e);
        }
        comm.println("#Sheet saved.");
    }
}