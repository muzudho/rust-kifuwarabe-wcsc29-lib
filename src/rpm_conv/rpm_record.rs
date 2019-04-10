use position::*;
use rpm_conv::rpm_identify_track::*;
use rpm_conv::rpm_operation_track::*;
use rpm_conv::rpm_operation_note::*;

use address::*;
use common_operation::*;
use communication::*;
use piece_etc::*;
use parser::*;

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
impl RpmRecordBody {
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
            body : RpmRecordBody {
                operation_track: RpmOTrack::default(),
                identify_track: RpmITrack::default(),
            },
        }
    }

    /// 後ろにレコードを連結する。
    /// TODO ヘッダーも連結したい。
    pub fn append_record(&mut self, record:&mut RpmRecord){
        self.body.append_track(&mut record.body);
    }

    /// 追加する。
    pub fn add_note(&mut self, rpm_note:&RpmOpeNote, identify:i8) {
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

    /// 棋譜読取。
    pub fn read_tape(comm:&Communication, line:&str, rpm_record:&mut RpmRecord, position:&mut Position) {
        let mut start = 0;

        loop {
            if line.len() <= start {
                return;
            }

            let ch1 = line[start..=start].chars().nth(0).unwrap();
            let rpm_note_opt = match ch1 {
                ' ' => {
                    comm.print(&ch1.to_string());
                    start += 1;
                    None
                }
                '0' => {
                    // 駒台。
                    start += 1;

                    let ch2 = line[start..=start].chars().nth(0).unwrap();
                    start += 1;

                    let text15;
                    match ch2 {
                        'P' | 'p' | 'ﾅ' => {
                            // 成り駒は、不成駒と同じところに置くので、成りのマークは読み飛ばす。
                            text15 = line[start..=start].chars().nth(0).unwrap().to_string();
                            start += 1;
                        },
                        _ => {
                            // Ignored.
                            text15 = "".to_string();
                        },
                    };

                    // 駒の種類、フェーズ。
                    let piece = PhysicalSign::default(ch2.to_string()).to_piece();

                    comm.print(&format!("{}{}{}", ch1, text15, ch2));
                    let address = Address::create_by_hand(
                        piece_to_phase(Some(piece)),
                        piece_to_piece_type(piece));
                    comm.println(&format!("address index = {}.", address.get_index()));
                    Some(RpmOpeNote::create_by_address(address))
                },
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    // セル
                    start += 1;
                    let ch2 = line[start..=start].chars().nth(0).unwrap();
                    start += 1;
                    comm.print(&format!("{}{}", ch1, ch2));
                    let file = Parser::file_char_to_i8(ch1);
                    let rank = Parser::rank_char_to_i8(ch2);
                    let address = Address::create_by_file_rank(file, rank, position.get_board_size());
                    Some(RpmOpeNote::create_by_address(address))
                },
                '+' => {
                    // 成り。
                    comm.print(&ch1.to_string());
                    start += 1;
                    Some(RpmOpeNote::turn_over())
                },
                '-' => {
                    // １８０°回転。
                    comm.print(&ch1.to_string());
                    start += 1;
                    Some(RpmOpeNote::rotate())
                },
                '|' => {
                    // フェーズ交代。
                    comm.print(&ch1.to_string());
                    start += 1;
                    Some(RpmOpeNote::change_phase())
                },
                '[' => {
                    // フェーズ交代。 ']' まで読み飛ばす。
                    comm.print(&ch1.to_string());
                    start += 1;
                    loop {
                        if line.len() <= start {
                            break;
                        }
                        
                        let sub_ch = line[start..=start].chars().nth(0).unwrap();
                        comm.print(&sub_ch.to_string());
                        start += 1;

                        if sub_ch == ']' {
                            break;
                        }
                    };
                    Some(RpmOpeNote::change_phase())
                },
                _ => {
                    let last = line.len();
                    panic!("Unexpected line '{}'.", &line[start..last]);
                }
            };

            if let Some(rpm_note) = rpm_note_opt {
                CommonOperation::touch_talking_beautifle_world(comm, rpm_record, &rpm_note, position);
            }
        }
    }
}
