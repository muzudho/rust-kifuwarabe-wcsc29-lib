use studio::address::*;

pub const DEFAULT_FILE_LEN: usize = 9;
pub const DEFAULT_RANK_LEN: usize = 9;

#[derive(Clone, Copy, PartialEq)]
pub struct BoardSize {
    pub file_len: i8,
    pub rank_len: i8,
}
impl BoardSize {
    pub fn create_hon_shogi() -> BoardSize {
        BoardSize {
            file_len: DEFAULT_FILE_LEN as i8,
            rank_len: DEFAULT_RANK_LEN as i8,
        }
    }

    pub fn cell_to_address(self, cell: Cell) -> usize {
        ((cell.get_rank() - 1) * self.file_len + (cell.get_file() - 1)) as usize
    }

    pub fn address_to_cell(self, address: usize) -> Cell {
        Cell::from_file_rank(
            (address % self.file_len as usize) as i8 + 1,
            (address / self.file_len as usize) as i8 + 1,
        )
    }
    pub fn len(self) -> usize {
        (self.file_len * self.rank_len) as usize
    }
    pub fn is_empty(self) -> bool {
        self.file_len * self.rank_len < 1
    }

    pub fn get_file_len(self) -> i8 {
        self.file_len
    }
    pub fn get_rank_len(self) -> i8 {
        self.rank_len
    }
}
