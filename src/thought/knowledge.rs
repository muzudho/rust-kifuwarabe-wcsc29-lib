use piece_etc::*;
use position::*;
use rpm_conv::rpm_record::*;
use thought::best_move_picker::*;

pub struct Knowledge {
}
impl Knowledge {
    pub fn new() -> Knowledge {
        Knowledge {            
        }
    }

    pub fn match_thread(&self, _position:&Position, _id:&PieceIdentify) -> ThreadsOfPiece {

        // TODO とりあえず -rpmrec.json ファイルを１個読む。

        ThreadsOfPiece {
            max_ply: 0,
            record: RpmRecord::default(),
        }
    }
}