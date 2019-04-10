/// １手分。
pub struct RpmMove {
    pub operation_notes: Vec<String>,
    pub piece_number_notes: Vec<i8>,
}
impl RpmMove {
    pub fn new() -> RpmMove {
        RpmMove {
            operation_notes: Vec::new(),
            piece_number_notes: Vec::new(),
        }
    }

    pub fn len_note(&self) -> usize {
        self.operation_notes.len()
    }

    pub fn is_empty_note(&self) -> bool {
        self.operation_notes.is_empty()
    }
}