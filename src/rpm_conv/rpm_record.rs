use position::*;
use rpm_conv::rpm_identify_track::*;
use rpm_conv::rpm_operation_track::*;
use rpm_conv::thread::rpm_note_operation::*;

use common_operation::*;
use communication::*;

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
    /// 何も進めていない状態で -1。
    pub cursor: i16,
    /// 何も指していない状態で 1。
    pub ply: i16,
    pub operation_track: RpmOTrack,
    pub identify_track: RpmITrack,
}
impl RpmRecordBody {
    pub fn default() -> RpmRecordBody {
        RpmRecordBody {
            cursor: -1,
            ply: 1,
            operation_track: RpmOTrack::default(),
            identify_track: RpmITrack::default(),
        }
    }
    pub fn append_track(&mut self, body:&mut RpmRecordBody) {
        self.operation_track.append_track(&mut body.operation_track);
        self.identify_track.append_track(&mut body.identify_track);
    }
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
            body : RpmRecordBody::default(),
        }
    }

    /// 後ろにレコードを連結する。
    /// TODO ヘッダーも連結したい。
    pub fn append_record(&mut self, record:&mut RpmRecord){
        self.body.append_track(&mut record.body);
    }

    /// 追加する。
    pub fn add_note(&mut self, rpm_note:&RpmNoteOpe, identify:i8) {
        let mut cursor_clone = self.body.cursor; // .clone();
        self.body.operation_track.add_element(&rpm_note, &mut self.body.cursor, &mut self.body.ply);
        self.body.identify_track.add_identify(identify, &mut cursor_clone);
    }

    pub fn forward(&mut self) -> bool {
        let mut cursor_clone = self.body.cursor; // .clone();
        let i = self.body.identify_track.forward(&mut self.body.cursor);
        let o = self.body.operation_track.forward(&mut cursor_clone,   &mut self.body.ply);
        if i!=o {panic!("Can not forward.");}

        i
    }

    pub fn back(&mut self) {
        let mut cursor_clone = self.body.cursor; // .clone();
        self.body.operation_track.back(&mut self.body.cursor, &mut self.body.ply);
        self.body.identify_track.back(&mut cursor_clone);
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

    /// 棋譜読取。
    pub fn read_tape(comm:&Communication, line:&str, rpm_record:&mut RpmRecord, position:&mut Position) {
        let mut start = 0;

        loop {
            if line.len() <= start {
                return;
            }

            let rpm_ope_1note_opt = RpmNoteOpe::parse_1note(&comm, &line, &mut start, position.get_board_size());

            if let Some(rpm_note) = rpm_ope_1note_opt {
                CommonOperation::touch_talking_beautifle_world(comm, rpm_record, &rpm_note, position);
            }
        }
    }
}
