//use communication::*;
//use conf::kifuwarabe_wcsc29_config::*;
//use human::human_interface::*;
//use piece_etc::*;
//use position::*;
use rpm_conv::thread::rpm_move::*;
//use rpm_for_json::rpm_book_file::*;
//use std::collections::HashMap;
//use std::fs;
//use usi_conv::usi_move::*;

/// シーケンスな手筋１個分。
#[derive(Default)]
pub struct RpmThread {
    pub moves: Vec<RpmMove>,
}
impl RpmThread {
    pub fn new() -> Self {
        RpmThread {
            moves: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    pub fn push_move(&mut self, rmove:RpmMove) {
        self.moves.push(rmove);
    }
}
