use parser::*;
use piece_etc::*;
use regex::Regex;

pub struct KifMove {
    pub dst_file : i8,
    pub dst_rank : i8,
    pub is_same : bool,
    pub piece : String,
    pub is_promote : bool,
    pub is_drop : bool,
    pub src_file : i8,
    pub src_rank : i8,
}
impl KifMove {
    pub fn to_sign(&self) -> String {
        let mut sign = "".to_string();

        sign = format!("{} {}", sign, self.dst_file);
        sign = format!("{} {}", sign, self.dst_rank);
        sign = format!("{} {}", sign, self.is_same);
        sign = format!("{} {}", sign, self.piece);
        sign = format!("{} {}", sign, self.is_promote);
        sign = format!("{} {}", sign, self.is_drop);
        sign = format!("{} {}", sign, self.src_file);
        sign = format!("{} {}", sign, self.src_rank);

        sign
    }

    pub fn parse(line:&str) -> Option<KifMove> {
        let re = Regex::new(r"\s*\d+ ((.*))*\([ 0-9:/]+\)").unwrap();
        let caps = re.captures(line).unwrap();
        let sign = caps.get(1).map_or("", |m| m.as_str());
        print!("{} -> ", sign);

        let mut mv = KifMove {
            dst_file : 0,
            dst_rank : 0,
            is_same : false,
            piece : "".to_string(),
            is_promote : false,
            is_drop : false,
            src_file : 0,
            src_rank : 0,
        };

        /**
Phase   0  1   2   3   4   5  6   7  8  9
       筋  段  同  駒  成  打  (  筋  段  )
       --  --  --  --  --  --  -  -  -  -
       ６  五      歩          (   6  6  )
       ７  七      角  成      (   3  3  )
               同　桂          (   8  9  )
       ６  六      角      打
         */
        let mut nth = 0;

        if 0 < sign.len() - nth {
            // Phase 0.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.dst_file = match ch.as_str() {
                "１" => 1,
                "２" => 2,
                "３" => 3,
                "４" => 4,
                "５" => 5,
                "６" => 6,
                "７" => 7,
                "８" => 8,
                "９" => 9,
                _ => {nth -= 1; 0},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 1.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.dst_rank = match ch.as_str() {
                "一" => 1,
                "二" => 2,
                "三" => 3,
                "四" => 4,
                "五" => 5,
                "六" => 6,
                "七" => 7,
                "八" => 8,
                "九" => 9,
                _ => {nth -= 1; 0},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 2.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.is_same = match ch.as_str() {
                "同" => {
                    let ch = sign.chars().nth(nth).unwrap().to_string();
                    nth += 1;
                    match ch.as_str() {
                        "　" => true,
                        _ => panic!("Unexpected same ch: '{}'.", ch),
                    }
                },
                _ => {nth -= 1; false},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 3.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.piece = match ch.as_str() {
                "歩" => "歩",
                "香" => "香",
                "桂" => "桂",
                "銀" => "銀",
                "金" => "金",
                "玉" => "玉",
                "角" => "角",
                "飛" => "飛",
                "と" => "と",
                "馬" => "馬",
                "龍" => "龍",
                "成" => {
                    let ch = sign.chars().nth(nth).unwrap().to_string();
                    nth += 1;
                    match ch.as_str() {
                        "香" => "成香",
                        "桂" => "成桂",
                        "銀" => "成銀",
                        "金" => "成金",
                        _ => panic!("Unexpected promoted ch: '{}'.", ch),
                    }
                },
                _ => {nth -= 1; ""},
            }.to_string();
        }

        if 0 < sign.len() - nth {
            // Phase 4.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.is_promote = match ch.as_str() {
                "成" => true,
                _ => {nth -= 1; false},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 5.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.is_drop = match ch.as_str() {
                "打" => true,
                _ => {nth -= 1; false},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 6.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            match ch.as_str() {
                "(" => {},
                _ => {nth -= 1},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 7.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.src_file = match ch.as_str() {
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                _ => {nth -= 1; 0},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 8.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            mv.src_rank = match ch.as_str() {
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                _ => {nth -= 1; 0},
            };
        }

        if 0 < sign.len() - nth {
            // Phase 9.
            let ch = sign.chars().nth(nth).unwrap().to_string();
            nth += 1;
            match ch.as_str() {
                ")" => {},
                _ => {nth -= 1},
            };
        }

        Some(mv)
    }
}