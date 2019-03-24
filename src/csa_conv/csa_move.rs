use parser::*;
use piece_etc::*;

pub struct CsaMove {
    pub phase:Phase,
    pub source_file:i8,
    pub source_rank:i8,
    pub destination_file:i8,
    pub destination_rank:i8,
    pub promotion:bool,
    pub koma:Option<PieceType>,
}
impl CsaMove {

    pub fn parse(line:&str) -> Option<CsaMove> {
        if line.len() < 7 {
            return None
        };

        let ch0 = line.chars().nth(0).unwrap();
        let ch1 = line.chars().nth(1).unwrap();
        let ch2 = line.chars().nth(2).unwrap();
        let ch3 = line.chars().nth(3).unwrap();
        let ch4 = line.chars().nth(4).unwrap();
        let str5 = &line[5..=6];

        let (src_f, src_r) = if ch1 == '0' {
            // drop.
            (0, 0)
        } else {
            (Parser::file_char_to_i8(ch1), Parser::rank_char_to_i8(ch2))
        };

        let piece_type = CsaMove::koma_to_piece_type(str5);

        Some(CsaMove {
            phase : match ch0 {
                '+' => {Phase::First}
                '-' => {Phase::Second}
                _ => panic!("Unexpected phase: '{}'.", ch1)
            },
            source_file: src_f,
            source_rank: src_r,
            destination_file: Parser::file_char_to_i8(ch3),
            destination_rank: Parser::rank_char_to_i8(ch4),
            promotion:false, // TODO
            koma: piece_type,
        })
    }

    pub fn is_drop(&self) -> bool {
        self.source_file == 0 && self.source_rank == 0
    }

    pub fn get_drop(&self) -> Option<PieceType> {
        if self.is_drop() {
            self.koma
        } else {
            None
        }
    }

    pub fn koma_to_piece_type(koma:&str) -> Option<PieceType> {
        use piece_etc::PieceType::*;
        match koma {
            "FU" => { Some(P) },
            "KY" => { Some(L) },
            "KE" => { Some(N) },
            "GI" => { Some(S) },
            "KI" => { Some(G) },
            "KA" => { Some(B) },
            "HI" => { Some(R) },
            "OU" => { Some(K) },
            "TO" => { Some(PP) },
            "NY" => { Some(PL) },
            "NK" => { Some(PN) },
            "NG" => { Some(PS) },
            "UM" => { Some(PB) },
            "RY" => { Some(PR) },
            // _ => { None },
            _ => panic!("Unexpected koma: '{}'.", koma)
        }
    }

    pub fn to_text(&self) -> String {
        format!("{} {} {} {} {} {}",
            phase_to_sign(self.phase),
            self.source_file,
            self.source_rank,
            self.destination_file,
            self.destination_rank,
            piece_type_to_sign(self.koma))
    }
}