use communication::*;
// use conf::kifuwarabe_wcsc29_lib_config::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
// use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;
use position::*;
// use rpm_conv::rpm_operation_track::*;
use rpm_conv::rpm_record::*;

pub struct RpmSheet {
    output_path: String,
    // records: Vec<RpmRecord>,
}
impl RpmSheet {
    pub fn default(output_file_path:&str) -> RpmSheet {
        RpmSheet {
            output_path: output_file_path.to_string(),
            // records: Vec::new(),
        }
    }

    pub fn read(&mut self, directory:&str) {
        let out_path = &self.output_path;
        for result in BufReader::new(File::open(Path::new(directory).join(out_path)).unwrap()).lines() {
            let line = result.unwrap();
            println!("Read line: `{}`.", line);
            // TODO self.records.push(record);
        }
    }

    /// 物理レコードを追加する。
    pub fn append(&self, comm:&Communication, board_size:BoardSize, directory:&str, rpm_record:&RpmRecord) {
        comm.println("#Sheet saving...");

        // 新規作成、またはレコードを追記。
        let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(Path::new(directory).join(&self.output_path))
        .unwrap();

        // 末尾にカンマを付けて終わる。
        if let Err(e) = writeln!(file, "{},", rpm_record.to_json_object(board_size)) {
            eprintln!("Couldn't write to file: {}", e);
        }
        comm.println("#Sheet saved.");
    }
}