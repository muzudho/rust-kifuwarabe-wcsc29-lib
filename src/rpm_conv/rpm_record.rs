use position::*;
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

    /// JSONのオブジェクト形式。
    pub fn to_json_object(&self, board_size:BoardSize) -> String {
        let mut unused_ply = 0;

        let mut text = "{\n".to_string();
        text = format!("{}    \"header\" : {{\n", text);
        text = format!("{}        \"date\" : \"{}\",\n", text, self.header.date);
        text = format!("{}        \"event\" : \"{}\",\n", text, self.header.event);
        text = format!("{}        \"player1\" : \"{}\",\n", text, self.header.player1);
        text = format!("{}        \"player2\" : \"{}\",\n", text, self.header.player2);
        text = format!("{}        \"read_file\" : \"{}\"\n", text, self.header.read_file);
        text = format!("{}    }},\n", text);
        text = format!("{}    \"body\" : {{\n", text);
        text = format!("{}        \"operation\" : [\n", text);
        text = format!("{}            {}\n", text, self.body.operation_track.to_json(board_size, &mut unused_ply));
        text = format!("{}        ],\n", text);
        text = format!("{}        \"piece_number\" : [\n", text);
        text = format!("{}            {}\n", text, self.body.identify_track.to_json(board_size));
        text = format!("{}        ]\n", text);
        text = format!("{}    }}\n", text);
        text = format!("{}}}", text);
        text
    }
}
