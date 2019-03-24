impl PhysicalRecord {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_last_move(&self) -> Option<PhysicalMove> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items[self.items.len()-1])
        }
    }
}