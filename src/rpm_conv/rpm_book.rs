use communication::*;
use position::*;
use rpm_conv::rpm_record::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// -rpmrec.json ファイルに対応。
pub struct RpmBook {
    file_path: Path,
}
impl RpmBook {
    pub fn default(path:&Path) -> RpmBook {       
        // Path::new(directory).join(file_path_text),
        RpmBook {
            file_path: path
        }
    }

    pub fn read_book(&mut self) {
        for result in BufReader::new(File::open(self.file_path).unwrap()).lines() {
            let line = result.unwrap();
            println!("Read line: `{}`.", line);
            // TODO self.records.push(record);
        }
    }
}