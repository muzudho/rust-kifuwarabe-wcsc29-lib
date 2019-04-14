use board_size::*;
use communication::*;
use parser::*;
use std::*;
use usi_conv::usi_record::*;

pub struct UsiPosition {

}
impl UsiPosition {
    pub fn parse_usi_line_moves(comm:&Communication, line:&str, start:&mut usize, board_size:BoardSize) -> Option<UsiRecord> {
        if Parser::match_keyword(&comm, &line, "moves", start) || 
            Parser::match_keyword(&comm, &line, " moves", start) {
        } else {
            // comm.println(&format!("#Moves not matched. line: '{}', start: {}.", line, start));
            return None;
        }

        UsiRecord::parse_usi_1record(&comm, line, start, board_size)
    }
}