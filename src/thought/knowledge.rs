use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use std::fs;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_record::*;
use rpm_model::rpm_book_file::*;
use thought::best_move_picker::*;

pub struct Knowledge {
}
impl Knowledge {
    pub fn new() -> Knowledge {
        Knowledge {            
        }
    }
}