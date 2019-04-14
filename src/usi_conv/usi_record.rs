use board_size::*;
use communication::*;
use parser::*;
use piece_etc::*;
use position::*;
use std::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use usi_conv::fen::*;
use usi_conv::usi_move::*;

 #[derive(Default)]
pub struct UsiRecord {
    pub moves : Vec<UsiMove>,
}
impl UsiRecord {
    pub fn new_usi_record() -> UsiRecord {
        UsiRecord {
            moves: Vec::new(),
        }
    }

    ///
    pub fn parse_usi_1record(comm:&Communication, line:&str, start:&mut usize, board_size:BoardSize) -> Option<Self> {
        let mut urecord = UsiRecord {
            moves: Vec::new(),
        };

        Parser::skip_spaces(&comm, &line, start);

        // `position startpos moves `. [0]p, [1]o, ...
        // Examples.
        // position startpos moves 2g2f 8c8d

        // ex.) Parses 7g7f 3c3d.
        loop {
            let umove = Fen::parse_usi_1move(&comm, &line, start, board_size);
            // comm.println(&format!("#Umove: `{}`.", umove.to_sign()));

            // TODO 内部形式としては RPM で持ちたい。
            urecord.moves.push(umove);

            if *start + 1 < line.len() {
                *start += 1;
            } else {
                break;
            }
        }
        // comm.println(&format!("#Usi record len: {}", urecord.moves.len()));
        //comm.println(&position.to_text(comm, urecord.get_current_phase()));

        Some(urecord)
    }

    /// 1行目のテキストを返す。
    pub fn read_first_line(comm:&Communication, file:&str) -> String {
        if let Some(first_line_result) = BufReader::new(File::open(file).unwrap()).lines().next() {
            let first_line = first_line_result.unwrap();
            comm.println(&format!("Read first line: `{}`.", first_line));
            first_line
        } else {
            "".to_string()
        }
    }

    /*
    pub fn parse_line(comm:&Communication, line:&str) -> UsiRecord {
        let mut position = Position::default();
        let mut urecord = UsiRecord::new();

        comm.println(&format!("#Parse line: `{}`.", line));
        let mut start = 0;
        if Fen::parse_position(&comm, &line, &mut start, &mut position) {
            comm.println("Position parsed.");

            if let Some(parsed_urecord) = CommonOperation::parse_usi_1record(&comm, &line, &mut start, position.get_board_size()) {
                comm.println("Moves parsed.");
                urecord = parsed_urecord;
            };
        }

        urecord
    }
     */

    /*
    pub fn get_items(self) -> Vec<UsiMove> {
        self.moves
    }
     */

    /*
    pub fn clear(&mut self) {
        self.moves.clear();
    }
     */

    pub fn get_current_phase(&self) -> Phase {
        match self.moves.len() % 2 {
            0 => Phase::First,
            _ => Phase::Second,
        }
    }

    pub fn make_usi_move(&mut self, umove:UsiMove, position:&mut Position){
        if umove.is_drop() {
            // TODO drop

        } else {
            let source_id_piece_opt = position.remove_id_piece(umove.source.unwrap());
            if umove.promotion {
                if let Some(mut source_id_piece) = source_id_piece_opt {
                    source_id_piece.turn_over();
                }
            }
            position.set_id_piece(umove.destination.unwrap(), source_id_piece_opt);
            self.moves.push(umove);
        }
    }
}
