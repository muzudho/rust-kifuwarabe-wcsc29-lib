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

    pub fn append(base:&mut String, appends:&String) {
        let start = base.len();
        base.insert_str(start, appends);
    }

    pub fn append_ln(base:&mut String) {
        Parser::append(base, &String::from("\r\n"));
    }

    pub fn appendln(base:&mut String, appends:&String) {
        Parser::append(base, appends);
        Parser::append(base, &String::from("\r\n"));
    }
}