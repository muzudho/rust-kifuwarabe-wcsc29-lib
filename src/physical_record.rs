use position::*;
use physical_move::*;

#[derive(Default)]
pub struct PhysicalRecord {
    items : Vec<PhysicalMove>,
}
impl PhysicalRecord {
    pub fn new() -> PhysicalRecord {
        PhysicalRecord {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, physical_move:&PhysicalMove) {
        self.items.push(*physical_move);
    }

    pub fn pop(&mut self) -> Option<PhysicalMove> {
        if self.items.is_empty() {
            None
        } else {
            let last = self.items.len()-1;
            let deleted = self.items[last];
            self.items.remove(last);
            Some(deleted)
        }
    }

    pub fn to_sign(&self, board_size:BoardSize) -> String {
        let mut sign = "".to_string();
        let mut ply = 1;
        for physical_move in &self.items {
            sign = format!("{} {}", sign, physical_move.to_sign(board_size, &mut ply));
        }
        sign
    }
}