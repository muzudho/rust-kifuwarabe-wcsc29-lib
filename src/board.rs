use position::*;

pub const DEFAULT_FILE_LEN: i8 = 9;
pub const DEFAULT_RANK_LEN: i8 = 9;
pub const DEFAULT_BOARD_SIZE: usize = (DEFAULT_FILE_LEN * DEFAULT_RANK_LEN) as usize;

pub struct Board {
    pub file_len: i8,
    pub rank_len: i8,
    pub board_size: usize,
    pub pieces: [Option<Piece>; DEFAULT_BOARD_SIZE],
}
impl Board {
    pub fn new() -> Board {
        Board {
            file_len: DEFAULT_FILE_LEN,
            rank_len: DEFAULT_RANK_LEN,
            board_size: (DEFAULT_RANK_LEN * DEFAULT_FILE_LEN) as usize,
            pieces : [None; DEFAULT_BOARD_SIZE],
        }
    }

    pub fn set_startpos(&mut self) {
        use position::Piece::*;
        self.pieces  = [
            Some(L1), Some(N1), Some(S1), Some(G1), Some(K1), Some(G1), Some(S1), Some(N1), Some(L1),
            None, Some(R1), None, None, None, None, None, Some(B1), None,
            Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1),
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0),
            None, Some(B0), None, None, None, None, None, Some(R0), None,
            Some(L0), Some(N0), Some(S0), Some(G0), Some(K0), Some(G0), Some(S0), Some(N0), Some(L0),
        ];
    }

    pub fn file_rank_to_cell(&self, file:i8, rank:i8) -> usize {
        ((rank-1)*self.file_len + (file-1)) as usize
    }
    pub fn cell_to_file_rank(&self, cell:usize) -> (i8, i8) {
        ((cell%self.file_len as usize) as i8, (cell/self.file_len as usize) as i8)
    }
    pub fn reverse_cell(&self, cell:usize) -> usize {
        self.rank_len as usize * self.file_len as usize - cell
    }
}