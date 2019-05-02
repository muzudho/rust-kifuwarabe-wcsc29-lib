use instrument::piece_etc::*;
use studio::address::*;
use studio::application::Application;
use studio::parser::*;

pub struct CsaMove {
    pub phase: Phase,
    pub source: Option<Cell>,
    pub destination: Cell,
    // 成ったかどうかは、盤上で駒を動かしてみないと分からない。
    pub koma: Option<PieceType>,
}
impl CsaMove {
    pub fn parse(line: &str, app: &Application) -> Option<CsaMove> {
        // comm.println(&format!("Parse   : line: '{}'.", line));

        if line.len() < 7 {
            return None;
        };

        let ch0 = line
            .chars()
            .nth(0)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. ch0.")));
        let ch1 = line
            .chars()
            .nth(1)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. ch1.")));
        let ch2 = line
            .chars()
            .nth(2)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. ch2.")));
        let ch3 = line
            .chars()
            .nth(3)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. ch3.")));
        let ch4 = line
            .chars()
            .nth(4)
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. ch4.")));
        let str5 = &line[5..=6];

        if ch0 == '%' {
            // '%CHUDAN' かも。
            return None;
        }

        let src_opt = if ch1 == '0' {
            // drop.
            None
        } else {
            Some(Cell::from_file_rank(
                Parser::file_char_to_i8(ch1),
                Parser::rank_char_to_i8(ch2),
            ))
        };

        let piece_type = CsaMove::koma_to_piece_type(str5);

        Some(CsaMove {
            phase: match ch0 {
                '+' => Phase::First,
                '-' => Phase::Second,
                _ => panic!("Unexpected phase: '{}'.", ch1),
            },
            source: src_opt,
            destination: Cell::from_file_rank(
                Parser::file_char_to_i8(ch3),
                Parser::rank_char_to_i8(ch4),
            ),
            koma: piece_type,
        })
    }

    pub fn is_drop(&self) -> bool {
        if let Some(_src) = self.source {
            false
        } else {
            true
        }
    }

    pub fn get_drop(&self) -> Option<PieceType> {
        if self.is_drop() {
            self.koma
        } else {
            None
        }
    }

    pub fn koma_to_piece_type(koma: &str) -> Option<PieceType> {
        use instrument::piece_etc::PieceType::*;
        match koma {
            "FU" => Some(P),
            "KY" => Some(L),
            "KE" => Some(N),
            "GI" => Some(S),
            "KI" => Some(G),
            "KA" => Some(B),
            "HI" => Some(R),
            "OU" => Some(K),
            "TO" => Some(PP),
            "NY" => Some(PL),
            "NK" => Some(PN),
            "NG" => Some(PS),
            "UM" => Some(PB),
            "RY" => Some(PR),
            // _ => { None },
            _ => panic!("Unexpected koma: '{}'.", koma),
        }
    }

    /*
    pub fn to_text(&self) -> String {
        format!("{} {} {} {} {} {}",
            phase_to_sign(self.phase),
            self.source_file,
            self.source_rank,
            self.destination_file,
            self.destination_rank,
            piece_type_to_sign(self.koma))
    }
    */
}
