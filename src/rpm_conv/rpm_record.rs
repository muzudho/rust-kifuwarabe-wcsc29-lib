/// Reversible physical move.
use board_size::*;
use communication::*;
use human::human_interface::*;
//use piece_etc::*;
use position::*;
use rpm_conv::rpm_tape::*;
//use rpm_conv::thread::rpm_note::*;
use rpm_conv::thread::rpm_note_operation::*;
use rpm_play::rpm_note_player::*;

/// 対局情報。
pub struct RpmRecordHeader {
    pub date: String,
    pub event: String,
    pub player1: String,
    pub player2: String,
    pub read_file: String,
}
impl RpmRecordHeader {
    pub fn clear(&mut self) {
        self.date = "".to_string();
        self.event = "".to_string();
        self.player1 = "".to_string();
        self.player2 = "".to_string();
        self.read_file = "".to_string();
    }
}

/// レコードの本体。
pub struct RpmRecordBody {
    /// 何も指していない状態で 1。
    pub ply: i16,
    pub rpm_tape: RpmTape,
}
impl RpmRecordBody {
    pub fn default() -> Self {
        let mut instance = RpmRecordBody {
            ply: 1,
            rpm_tape: RpmTape::default(),
        };

        // 共通処理にする。
        instance.clear();

        instance
    }
    pub fn clear(&mut self) {
        self.ply = 1;
        self.rpm_tape.clear();
    }
    pub fn append_tape(&mut self, tape:&mut RpmTape) {
        self.rpm_tape.append_tape(tape);
    }
}

pub struct RpmRecord {
    pub header: RpmRecordHeader,
    pub body: RpmRecordBody,
}
impl RpmRecord {
    pub fn default() -> Self {
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

    pub fn clear(&mut self) {
        self.header.clear();
        self.body.clear();
    }

    /// 後ろにレコードを連結する。
    /// TODO ヘッダーも連結したい。
    pub fn append_record(&mut self, record:&mut RpmRecord){
        self.body.append_tape(&mut record.body.rpm_tape);
    }

    /*
    /// 追加する。
    pub fn add_note_to_tape(&mut self, pid:Option<PieceIdentify>, rpm_note:&RpmNoteOpe) {
        self.body.rpm_tape.add_note(RpmNote::from_id_ope(pid, *rpm_note), &mut self.body.ply);
    }
     */

    pub fn forward(&mut self) -> bool {
        self.body.rpm_tape.forward(&mut self.body.ply)
    }

    pub fn back(&mut self) {
        self.body.rpm_tape.back(&mut self.body.ply);
    }

    pub fn get_tape(self) -> RpmTape {
        self.body.rpm_tape
    }

    pub fn get_mut_tape(&mut self) -> &mut RpmTape {
        &mut self.body.rpm_tape
    }

    /// JSONのオブジェクト形式。
    pub fn to_json_object(&self, board_size:BoardSize) -> String {
        let mut unused_ply = 0;
        let (numbers, operations) = self.body.rpm_tape.to_json(board_size, &mut unused_ply);

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
        text = format!("{}            {}\n", text, operations);
        text = format!("{}        ],\n", text);
        text = format!("{}        \"piece_number\" : [\n", text);
        text = format!("{}            {}\n", text, numbers);
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
                RpmNotePlayer::touch_brandnew_note(comm, &mut rpm_record.body.rpm_tape, &rpm_note, position);
                HumanInterface::bo(comm, &rpm_record, &position);
            }
        }
    }
}
