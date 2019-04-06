use position::*;
use rpm_conv::*;
use rpm_conv::rpm_identify_track::*;
use rpm_conv::rpm_operation_track::*;
use rpm_conv::rpm_operation_note::*;

/// 対局情報。
pub struct RpmRecordHeader {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}

/// レコードの本体。
pub struct RpmRecordBody {
    pub operation_track: RpmOTrack,
    pub identify_track: RpmITrack,
}

/// Reversible physical move - Record.
pub struct RpmRecord {
    pub header: RpmRecordHeader,
    pub body: RpmRecordBody,
}
impl RpmRecord {
    pub fn default() -> RpmRecord {
        RpmRecord {
            header : RpmRecordHeader {
                date: "".to_string(),
                event: "".to_string(),
                player1: "".to_string(),
                player2: "".to_string(),
                read_file: "".to_string(),
            },
            body : RpmRecordBody {
                operation_track: RpmOTrack::default(),
                identify_track: RpmITrack::default(),
            },
        }
    }

    /// 追加する。
    pub fn add_note(&mut self, rpm_note:&RpmNote, identify:i16) {
        self.body.operation_track.add_element(&rpm_note);
        self.body.identify_track.add_identify(identify);
    }

    pub fn forward(&mut self) -> bool {
        let i = self.body.identify_track.forward();
        let o = self.body.operation_track.forward();
        if i!=o {panic!("Can not forward.");}

        i
    }

    pub fn back(&mut self) {
        self.body.operation_track.back();
        self.body.identify_track.back();
    }

    /*
    pub fn get_operation_track(self) -> RpmOTrack {
        self.operation_track
    }
     */

    pub fn get_mut_operation_track(&mut self) -> &mut RpmOTrack {
        &mut self.body.operation_track
    }

    pub fn get_identify_track(self) -> RpmITrack {
        self.body.identify_track
    }

    pub fn get_mut_identify_track(&mut self) -> &mut RpmITrack {
        &mut self.body.identify_track
    }

    /// 定跡ファイルの保存形式でもある。
    pub fn to_sign(&self, board_size:BoardSize) -> String {
        let mut unused_ply = 0;

        let mut sign = "Rec\n".to_string();
        sign = format!("{}    Tr.0: {}\n", sign, self.body.operation_track.to_sign(board_size, &mut unused_ply));
        sign = format!("{}    Tr.1: {}\n", sign, self.body.identify_track.to_sign(board_size));
        sign
    }
}
