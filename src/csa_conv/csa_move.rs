use parser::*;
use position::*;

pub struct CsaMove {
    pub phase:Phase,
    pub source_file:i8,
    pub source_rank:i8,
    pub destination_file:i8,
    pub destination_rank:i8,
    pub koma:String,
}
impl CsaMove {
    pub fn parse(line:&str) -> CsaMove {
        let ch0 = line.chars().nth(0).unwrap();
        let ch1 = line.chars().nth(1).unwrap();
        let ch2 = line.chars().nth(2).unwrap();
        let ch3 = line.chars().nth(3).unwrap();
        let ch4 = line.chars().nth(4).unwrap();
        let str5 = &line[5..=6];

        CsaMove {
            phase : match ch0 {
                '+' => {Phase::First}
                '-' => {Phase::Second}
                _ => panic!("Unexpected phase: '{}'.", ch1)
            },
            source_file: Parser::file_char_to_i8(ch1),
            source_rank: Parser::rank_char_to_i8(ch2),
            destination_file: Parser::file_char_to_i8(ch3),
            destination_rank: Parser::rank_char_to_i8(ch4),
            koma: str5.to_string(),
        }
    }

    pub fn to_text(&self) -> String {
        format!("{} {} {} {} {} {}",
            phase_to_sign(self.phase),
            self.source_file,
            self.source_rank,
            self.destination_file,
            self.destination_rank,
            self.koma)
    }
}