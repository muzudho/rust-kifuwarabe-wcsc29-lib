use instrument::piece_etc::*;
use regex::Regex;
use studio::address::*;
use studio::application::Application;

pub struct KifMove {
    pub destination: Option<Cell>,
    pub is_same: bool,
    pub piece: Option<JsaPieceType>,
    pub is_promote: bool,
    pub is_drop: bool,
    pub source: Option<Cell>,
}
impl KifMove {
    pub fn to_sign(&self) -> String {
        let mut sign = "".to_string();

        if let Some(x) = self.destination {
            sign = format!("{} {}", sign, x.get_file());
            sign = format!("{} {}", sign, x.get_rank());
        };

        sign = format!("{} {}", sign, self.is_same);
        sign = format!("{} {}", sign, jsa_piece_type_to_sign(self.piece));
        sign = format!("{} {}", sign, self.is_promote);
        sign = format!("{} {}", sign, self.is_drop);

        if let Some(x) = self.source {
            sign = format!("{} {}", sign, x.get_file());
            sign = format!("{} {}", sign, x.get_rank());
        };

        sign
    }

    pub fn parse(line: &str, app: &Application) -> Option<KifMove> {
        let re = Regex::new(r"\s*\d+ ((.*))*\([ 0-9:/]+\)")
            .unwrap_or_else(|f| panic!(app.comm.panic(&f.to_string())));
        let caps = re
            .captures(line)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. parse.")));
        let sign = caps.get(1).map_or("", |m| m.as_str());

        let mut mv = KifMove {
            destination: None,
            is_same: false,
            piece: None,
            is_promote: false,
            is_drop: false,
            source: None,
        };

        /*
        Phase   0  1   2   3   4   5  6   7  8  9
               筋  段  同  駒  成  打  (  筋  段  )
               --  --  --  --  --  --  -  -  -  -
               ６  五      歩          (   6  6  )
               ７  七      角  成      (   3  3  )
                       同　桂          (   8  9  )
               ６  六      角      打
                 */
        let mut nth = 0;

        let dfile = if 0 < sign.len() - nth {
            // Phase 0.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sing.")))
                .to_string();
            nth += 1;
            match ch.as_str() {
                "１" => Some(1),
                "２" => Some(2),
                "３" => Some(3),
                "４" => Some(4),
                "５" => Some(5),
                "６" => Some(6),
                "７" => Some(7),
                "８" => Some(8),
                "９" => Some(9),
                _ => {
                    nth -= 1;
                    None
                }
            }
        } else {
            None
        };

        mv.destination = if 0 < sign.len() - nth {
            // Phase 1.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                .to_string();
            nth += 1;
            match ch.as_str() {
                "一" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    1,
                )),
                "二" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    2,
                )),
                "三" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    3,
                )),
                "四" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    4,
                )),
                "五" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    5,
                )),
                "六" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    6,
                )),
                "七" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    7,
                )),
                "八" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    8,
                )),
                "九" => Some(Cell::from_file_rank(
                    dfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. dfile."))),
                    9,
                )),
                _ => {
                    nth -= 1;
                    None
                }
            }
        } else {
            None
        };

        if 0 < sign.len() - nth {
            // Phase 2.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                .to_string();
            nth += 1;
            mv.is_same = match ch.as_str() {
                "同" => {
                    let ch = sign
                        .chars()
                        .nth(nth)
                        .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                        .to_string();
                    nth += 1;
                    match ch.as_str() {
                        "　" => true,
                        _ => panic!("Unexpected same ch: '{}'.", ch),
                    }
                }
                _ => {
                    nth -= 1;
                    false
                }
            };
        }

        if 0 < sign.len() - nth {
            // Phase 3.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                .to_string();
            nth += 1;
            use instrument::piece_etc::JsaPieceType::*;
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
                    let ch = sign
                        .chars()
                        .nth(nth)
                        .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                        .to_string();
                    nth += 1;
                    match ch.as_str() {
                        "香" => Some(PL),
                        "桂" => Some(PN),
                        "銀" => Some(PS),
                        _ => panic!("Unexpected promoted ch: '{}'.", ch),
                    }
                }
                _ => {
                    nth -= 1;
                    None
                }
            };
        }

        if 0 < sign.len() - nth {
            // Phase 4.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                .to_string();
            nth += 1;
            mv.is_promote = match ch.as_str() {
                "成" => true,
                _ => {
                    nth -= 1;
                    false
                }
            };
        }

        if 0 < sign.len() - nth {
            // Phase 5.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                .to_string();
            nth += 1;
            mv.is_drop = match ch.as_str() {
                "打" => true,
                _ => {
                    nth -= 1;
                    false
                }
            };
        }

        if 0 < sign.len() - nth {
            // Phase 6.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                .to_string();
            nth += 1;
            match ch.as_str() {
                "(" => {}
                _ => nth -= 1,
            };
        }

        let sfile = if 0 < sign.len() - nth {
            // Phase 7.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile.")))
                .to_string();
            nth += 1;
            match ch.as_str() {
                "1" => Some(1),
                "2" => Some(2),
                "3" => Some(3),
                "4" => Some(4),
                "5" => Some(5),
                "6" => Some(6),
                "7" => Some(7),
                "8" => Some(8),
                "9" => Some(9),
                _ => {
                    nth -= 1;
                    None
                }
            }
        } else {
            None
        };

        mv.source = if 0 < sign.len() - nth {
            // Phase 8.
            let ch = sign
                .chars()
                .nth(nth)
                .unwrap_or_else(|| panic!(app.comm.panic("Fail. sign.")))
                .to_string();
            /*nth += 1;*/
            match ch.as_str() {
                "1" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    1,
                )),
                "2" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    2,
                )),
                "3" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    3,
                )),
                "4" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    4,
                )),
                "5" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    5,
                )),
                "6" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    6,
                )),
                "7" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    7,
                )),
                "8" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    8,
                )),
                "9" => Some(Cell::from_file_rank(
                    sfile.unwrap_or_else(|| panic!(app.comm.panic("Fail. sfile."))),
                    9,
                )),
                _ => {
                    /*nth -= 1;*/
                    None
                }
            }
        } else {
            None
        };

        /*
        if 0 < sign.len() - nth {
            // Phase 9.
            let ch = sign.chars().nth(nth).unwrap_or_else(|| panic!(app.comm.panic("Fail. sign."))).to_string();
            /* nth += 1; */
            match ch.as_str() {
                ")" => {},
                _ => {/*nth -= 1*/},
            };
        }
        */

        Some(mv)
    }
}
