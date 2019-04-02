use position::*;
use rpm_conv::*;
use rpm_conv::rpm_identify_track::*;
use rpm_conv::rpm_operation_track::*;
use rpm_conv::rpm_operation_note::*;

/// Reversible physical move - Record.
pub struct RpmRecord {
    pub operation_track: RpmOTrack,
    identify_track: RpmITrack,
}
impl RpmRecord {
    pub fn default() -> RpmRecord {
        RpmRecord {
            operation_track: RpmOTrack::default(),
            identify_track: RpmITrack::default(),
        }
    }

    pub fn add(&mut self, rpm_note:&RpmNote, identify:i16) {
        self.operation_track.add_element(&rpm_note);
        self.identify_track.add_element(identify);
    }

    pub fn forward(&mut self) -> bool {
        let i = self.identify_track.forward();
        let o = self.operation_track.forward();
        if i!=o {panic!("Can not forward.");}

        i
    }

    pub fn back(&mut self) {
        self.operation_track.back();
        self.identify_track.back();
    }

    /*
    pub fn get_operation_track(self) -> RpmOTrack {
        self.operation_track
    }
     */

    pub fn get_mut_operation_track(&mut self) -> &mut RpmOTrack {
        &mut self.operation_track
    }

    pub fn get_identify_track(self) -> RpmITrack {
        self.identify_track
    }

    pub fn get_mut_identify_track(&mut self) -> &mut RpmITrack {
        &mut self.identify_track
    }

    /// 定跡ファイルの保存形式でもある。
    pub fn to_sign(&self, board_size:BoardSize) -> String {
        let mut unused_ply = 0;

        let mut sign = "Rec\n".to_string();
        sign = format!("{}    Tr.0: {}\n", sign, self.operation_track.to_sign(board_size, &mut unused_ply));
        sign = format!("{}    Tr.1: {}\n", sign, self.identify_track.to_sign(board_size));
        sign
    }
}
