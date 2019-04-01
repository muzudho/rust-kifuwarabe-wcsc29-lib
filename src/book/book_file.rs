use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};

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
}