impl RpmOTrack {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_last_move(&self) -> Option<RpmNoteOpe> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items[self.items.len()-1])
        }
    }
}