pub struct Parser {
}
impl Parser {
    pub fn match_keyword(line:&str, keyword:&str, start:&mut usize) -> bool {
        // スタートが文字列の終端を読み終わっていれば、結果は空。
        if line.len() <= *start {
            false
        } else if &line[*start..(*start+(keyword.len()-1))] == keyword {
            *start += keyword.len();
            true
        } else {
            false
        }
    }
}