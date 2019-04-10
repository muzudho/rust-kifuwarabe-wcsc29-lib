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

    pub fn to_operation_string(&self) -> String {
        let mut text = String::new();

        for i in 0..self.len_note() {
            text = format!("{} {}", text, &self.operation_notes[i]);
        }

        text
    }

    pub fn to_identify_string(&self) -> String {
        let mut text = String::new();

        for i in 0..self.len_note() {
            text = format!("{} {}", text, &self.piece_number_notes[i]);
        }

        text
    }
}