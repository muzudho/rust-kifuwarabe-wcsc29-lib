use rpm_conv::thread::rpm_operation_note::*;

#[derive(Debug)]
pub struct RpmNote {
    operation: RpmOpeNote,
    // 駒の背番号。
    identify: i8,
}
impl RpmNote {
    pub fn create(operation_note: RpmOpeNote, identify_num: i8) -> RpmNote {
        RpmNote {
            operation: operation_note,
            identify: identify_num,
        }
    }

    pub fn get_ope(&self) -> RpmOpeNote {
        self.operation
    }

    pub fn get_id(&self) -> i8 {
        self.identify
    }
}
