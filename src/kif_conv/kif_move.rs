use parser::*;
use piece_etc::*;
use regex::Regex;

pub struct KifMove {
    pub origin_line:String,
}
impl KifMove {
    pub fn parse(line:&str) -> Option<KifMove> {
        let re = Regex::new(r"\s*\d+ ((.*))*\([ 0-9:/]+\)").unwrap();
        let caps = re.captures(line).unwrap();
        let text1 = caps.get(1).map_or("", |m| m.as_str());
        println!("{}", text1);

        Some(KifMove {
            origin_line: line.to_string()
        })
    }
}