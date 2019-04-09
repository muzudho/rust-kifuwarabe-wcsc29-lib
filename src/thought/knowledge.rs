use communication::*;
use conf::kifuwarabe_wcsc29_config::*;
use std::fs;
use piece_etc::*;
use position::*;
use rpm_conv::rpm_record::*;
use rpm_model::rpm_book_file::*;
use thought::best_move_picker::*;

pub struct Knowledge {
}
impl Knowledge {
    pub fn new() -> Knowledge {
        Knowledge {            
        }
    }

    pub fn match_thread(&self, comm:&Communication, kw29config:&KifuwarabeWcsc29Config, _position:&Position, _id:&PieceIdentify) -> ThreadsOfPiece {
        println!("#match_thread start. Dir: {}", kw29config.rpm_record);

        // TODO とりあえず -rpmrec.json ファイルを１個読む。
        for path in fs::read_dir(&kw29config.rpm_record).unwrap() {
            let file = path.unwrap().path().display().to_string();
            // comm.println(&format!("file: {}", file));
            let book_file = RpmBookFile::load(&file);

            // ファイルの中身をすこし見てみる。
            comm.println(&format!("file: {}, Book len: {}.", file, book_file.book.len() ));
            if !book_file.book.is_empty() {
                comm.println(&format!("Ope len: {}, Num len: {}.", book_file.book[0].body.operation.len(), book_file.book[0].body.piece_number.len() ));
            }
        }        
        println!("#match_thread loop end.");

        ThreadsOfPiece {
            max_ply: 0,
            record: RpmRecord::default(),
        }
    }
}