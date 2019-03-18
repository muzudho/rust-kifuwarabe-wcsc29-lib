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
        // Flip horizontal.
        self.pieces  = [
            Some(L1), Some(N1), Some(S1), Some(G1), Some(K1), Some(G1), Some(S1), Some(N1), Some(L1),
            None, Some(B1), None, None, None, None, None, Some(R1), None,
            Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1),
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0), Some(P0),
            None, Some(R0), None, None, None, None, None, Some(B0), None,
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

    pub fn get_piece(&self, file:i8, rank:i8) -> Option<Piece> {
        let cell = self.file_rank_to_cell(file, rank);
        self.pieces[cell]
    }

    /// Point of symmetory.
    pub fn print(&self) {
        let rank_array = ['?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

        for y in 0..=8 {
            let rank = 9 - y;
            println!(
                "{0} {1: >2}{2: >2}{3: >2}{4: >2}{5: >2}{6: >2}{7: >2}{8: >2}{9: >2}",
                rank_array[rank as usize],
                piece_to_sign(&self.get_piece(1, rank)),
                piece_to_sign(&self.get_piece(2, rank)),
                piece_to_sign(&self.get_piece(3, rank)),
                piece_to_sign(&self.get_piece(4, rank)),
                piece_to_sign(&self.get_piece(5, rank)),
                piece_to_sign(&self.get_piece(6, rank)),
                piece_to_sign(&self.get_piece(7, rank)),
                piece_to_sign(&self.get_piece(8, rank)),
                piece_to_sign(&self.get_piece(9, rank)));
        }
        println!("   1 2 3 4 5 6 7 8 9");
    }
}