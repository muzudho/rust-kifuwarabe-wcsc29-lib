use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Default)]
pub struct Communication {
    pub log_file : String,
}
impl Communication {
    pub fn new () -> Communication {
        Communication {
            log_file : "comm.log".to_string(),
        }
    }

    /// Write.
    pub fn print(&self, line:&str) {
        println!("{}", line);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file)
            .unwrap();

        if let Err(e) = write!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    /// Write line.
    pub fn println(&self, line:&str) {
        println!("{}", line);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
