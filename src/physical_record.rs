use board::*;
use physical_move::*;
use position::*;

pub struct PhysicalRecord {
    phase: Phase,
    position: Position,
}
impl PhysicalRecord {
    pub fn new() -> PhysicalRecord {
        PhysicalRecord {
            phase: Phase::First,
            position: Position::new(),
        }
    }

    pub fn get_phase(&self) -> Phase {
        self.phase
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }
    pub fn get_mut_position(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn touch(&self, physical_move:PhysicalMove, board:&mut Board) {
        board.touch(physical_move);
    }
}