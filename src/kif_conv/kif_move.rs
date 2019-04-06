use parser::*;
use piece_etc::*;
use regex::Regex;

pub struct KifMove {
    pub destination_file : i8,
    pub destination_rank : i8,
    pub is_same : bool,
    pub piece : Option<JsaPieceType>,
    pub is_promote : bool,
    pub is_drop : bool,
    pub source_file : i8,
    pub source_rank : i8,
}
impl KifMove {
    pub fn to_sign(&self) -> String {
        let mut sign = "".to_string();

        sign = format!("{} {}", sign, self.destination_file);
        sign = format!("{} {}", sign, self.destination_rank);
        sign = format!("{} {}", sign, self.is_same);
        sign = format!("{} {}", sign, jsa_piece_type_to_sign(self.piece));
        sign = format!("{} {}", sign, self.is_promote);
        sign = format!("{} {}", sign, self.is_drop);
        sign = format!("{} {}", sign, self.source_file);
        sign = format!("{} {}", sign, self.source_rank);

        sign
    }

    pub fn parse(line:&str) -> Option<KifMove> {
        let re = Regex::new(r"\s*\d+ ((.*))*\([ 0-9:/]+\)").unwrap();
        let caps = re.captures(line).unwrap();
        let sign = caps.get(1).map_or("", |m| m.as_str());
        print!("{} -> ", sign);

        let mut mv = KifMove {
            destination_file : 0,
            destination_rank : 0,
            is_same : false,
            piece : None,
            is_promote : false,
            is_drop : false,
            source_file : 0,
            source_rank : 0,
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
            mv.destination_file = match ch.as_str() {
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
            mv.destination_rank = match ch.as_str() {
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
            use piece_etc::JsaPieceType::*;
            mv.piece = match ch.as_str() {
                "歩" => Some(P),
                "香" => Some(L),
                "桂" => Some(N),
                "銀" => Some(S),
                "金" => Some(G),
                "玉" => Some(K),
                "角" => Some(B),
                "飛" => Some(R),
                "と" => Some(PP),
                "馬" => Some(PB),
                "龍" => Some(PR),
                "成" => {
                    let ch = sign.chars().nth(nth).unwrap().to_string();
                    nth += 1;
                    match ch.as_str() {
                        "香" => Some(PL),
                        "桂" => Some(PN),
                        "銀" => Some(PS),
                        _ => panic!("Unexpected promoted ch: '{}'.", ch),
                    }
                },
                _ => {nth -= 1; None},
            };
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
            mv.source_file = match ch.as_str() {
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
            mv.source_rank = match ch.as_str() {
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