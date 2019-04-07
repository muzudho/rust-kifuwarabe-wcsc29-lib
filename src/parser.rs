use communication::*;

pub struct Parser {
}
impl Parser {
    pub fn match_keyword(_comm:&Communication, line:&str, keyword:&str, start:&mut usize) -> bool {
        // スタートが文字列の終端を読み終わっていれば、結果は空。
        if line.len() <= *start {
            return false;
        }

        if &line[*start..(*start+keyword.len())] == keyword {
            *start += keyword.len();
            true
        } else {
            false
        }
    }

    // 半角空白を読み飛ばします。
    pub fn skip_spaces(_comm:&Communication, line:&str, start:&mut usize) {
        if line.len() <= *start {
            return;
        }

        while let Some(ch) = line.chars().nth(*start) {
            if ch == ' ' {
                // Skip space.
                *start += 1;
            } else {
                break;
            }
        }
    }

    pub fn append(base:&mut String, appends:&str) {
        let start = base.len();
        base.insert_str(start, appends);
    }

    pub fn append_ln(base:&mut String) {
        Parser::append(base, &String::from("\r\n"));
    }

    pub fn appendln(base:&mut String, appends:&str) {
        Parser::append(base, appends);
        Parser::append(base, &String::from("\r\n"));
    }

    pub fn file_char_to_i8(ch:char) -> i8 {
        match ch {
            '1' => {1},
            '2' => {2},
            '3' => {3},
            '4' => {4},
            '5' => {5},
            '6' => {6},
            '7' => {7},
            '8' => {8},
            '9' => {9},
            _ => {panic!("Unexpected file char: '{0}'", ch)},
        }
    }
    pub fn rank_char_to_i8(ch:char) -> i8 {
        match ch {
            '1' | 'a' => {1},
            '2' | 'b' => {2},
            '3' | 'c' => {3},
            '4' | 'd' => {4},
            '5' | 'e' => {5},
            '6' | 'f' => {6},
            '7' | 'g' => {7},
            '8' | 'h' => {8},
            '9' | 'i' => {9},
            _ => {panic!("Unexpected rank char: '{0}'", ch)},
        }
    }
    pub fn i8_to_rank_char(rank:i8) -> char {
        match rank {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            9 => 'i',
            _ => {panic!("Unexpected rank: {0}", rank)},
        }
    }
}