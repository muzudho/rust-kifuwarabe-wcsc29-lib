use parser::*;
use piece_etc::*;

pub struct KifMove {
    pub origin_line:String,
}
impl KifMove {
    pub fn parse(line:&str) -> Option<KifMove> {
        Some(KifMove {
            origin_line: line.to_string()
        })
    }
}