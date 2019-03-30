use communication::*;
use parser::*;
use physical_move::*;
use piece_etc::*;
use std::*;

pub const DEFAULT_FILE_LEN: usize = 9;
pub const DEFAULT_RANK_LEN: usize = 9;
pub const SKY_LEN: usize = 1;
pub const SKY_ADDRESS: usize = 81;
pub const DEFAULT_BOARD_SIZE: usize = (DEFAULT_FILE_LEN * DEFAULT_RANK_LEN + SKY_LEN) as usize;
pub const HANDS_LEN: usize = 3 * 8;

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

    pub fn file_rank_to_cell(self, file:i8, rank:i8) -> usize {
        ((rank-1)*self.file_len + (file-1)) as usize
    }
    pub fn cell_to_file_rank(self, cell:usize) -> (i8, i8) {
        ((cell%self.file_len as usize) as i8 + 1, (cell/self.file_len as usize) as i8 + 1)
    }
    pub fn len(self) -> usize {
        (self.file_len * self.rank_len) as usize
    }
    pub fn is_empty(self) -> bool {
        self.file_len * self.rank_len < 1
    }
}

pub struct Position {
    phase: Phase,
    board_size: BoardSize,
    pub board: [Option<Piece>; DEFAULT_BOARD_SIZE],
    pub hands: [Vec<PieceType>; HANDS_LEN],
}
impl Position {
    pub fn default() -> Position {
        Position {
            phase: Phase::First,
            board_size: BoardSize::create_hon_shogi(),
            board: [None; DEFAULT_BOARD_SIZE],
            hands: [
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            ],
        }
    }

    pub fn reset_default(&mut self) {
        self.board_size = BoardSize::create_hon_shogi();
        self.board = [None; DEFAULT_BOARD_SIZE];
        self.hands = [
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
        ];
        // 使わない駒
        use piece_etc::Piece::*;
        use piece_etc::PieceType::*;
        // 玉2枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(K3)];
            vec.push(K);
            vec.push(K);
        }
        // 飛2枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(R3)];
            vec.push(R);
            vec.push(R);
        }
        // 角2枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(B3)];
            vec.push(B);
            vec.push(B);
        }
        // 金4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(G3)];
            vec.push(G);
            vec.push(G);
            vec.push(G);
            vec.push(G);
        }
        // 銀4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(S3)];
            vec.push(S);
            vec.push(S);
            vec.push(S);
            vec.push(S);
        }
        // 桂4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(N3)];
            vec.push(N);
            vec.push(N);
            vec.push(N);
            vec.push(N);
        }
        // 香4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(L3)];
            vec.push(L);
            vec.push(L);
            vec.push(L);
            vec.push(L);
        }
        // 歩18枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(P3)];
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
            vec.push(P);
        }
    }

    pub fn reset_startpos(&mut self) {
        use piece_etc::Piece::*;
        self.board_size = BoardSize::create_hon_shogi();
        // Flip horizontal.
        self.board = [
            Some(L2), Some(N2), Some(S2), Some(G2), Some(K2), Some(G2), Some(S2), Some(N2), Some(L2),
            None, Some(B2), None, None, None, None, None, Some(R2), None,
            Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2), Some(P2),
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1), Some(P1),
            None, Some(R1), None, None, None, None, None, Some(B1), None,
            Some(L1), Some(N1), Some(S1), Some(G1), Some(K1), Some(G1), Some(S1), Some(N1), Some(L1),
            None, // Sky
        ];
        self.hands = [
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
        ];
    }

    pub fn get_phase(&self) -> Phase {
        self.phase
    }

    pub fn get_board_size(&self) -> BoardSize {
        self.board_size
    }

    pub fn get_piece(&self, file:i8, rank:i8) -> Option<Piece> {
        let address = self.board_size.file_rank_to_cell(file, rank);
        self.board[address]
    }

    pub fn get_piece_by_address(&self, address:usize) -> Option<Piece> {
        self.board[address]
    }

    /// Obsolute. new --> add().
    pub fn set_piece(&mut self, file:i8, rank:i8, piece:Option<Piece>) {
        let cell = self.board_size.file_rank_to_cell(file, rank);
        self.board[cell] = piece;
    }

    pub fn remove_piece(&mut self, file:i8, rank:i8) -> Option<Piece> {
        let cell = self.get_board_size().file_rank_to_cell(file, rank);
        let piece = self.board[cell];
        self.set_piece(file, rank, None);
        piece
    }

    pub fn get_hand(&self, piece:Piece) -> i8 {
        let hand_index = hand_piece_to_hand_index(piece);
        self.hands[hand_index].len() as i8
    }

    pub fn add_hand(&mut self, piece:Piece) {
        let hand_index = hand_piece_to_hand_index(piece);
        self.hands[hand_index].push(piece_to_piece_type(piece))
    }

    pub fn remove_hand(&mut self, piece:Piece) -> PieceType {
        let hand_index = hand_piece_to_hand_index(piece);
        self.hands[hand_index].pop().unwrap()
    }

    pub fn touch(&mut self, _comm:&Communication, physical_move:&PhysicalMove) {
        match physical_move.address {
            Some(address) => {
                // どこかを指定した。
                if address.is_on_board(self.board_size) {
                    // 盤上。
                    match self.board[address.get_index()] {
                        Some(piece) => {
                            // 駒の場所を指定した。
                            match self.board[SKY_ADDRESS] {
                                Some(_piece) => {
                                    // 指には何も持ってない。
                                },
                                None => {
                                    // 指で駒をつかむ。
                                    self.board[SKY_ADDRESS] = Some(piece);
                                    self.board[address.get_index()] = None;
                                },
                            }
                        },
                        None => {
                            // 空き升を指定した。
                            if let Some(piece) = self.board[SKY_ADDRESS] {
                                // 指につまんでいる駒を置く。
                                self.board[SKY_ADDRESS] = None;
                                self.board[address.get_index()] = Some(piece);
                            }
                        },
                    }
                } else {
                    // 駒台。
                    match self.board[SKY_ADDRESS] {
                        Some(piece) => {
                            // 指に何か持っているので、駒台に置く。
                            self.board[SKY_ADDRESS] = None;
                            // comm.println(&format!("hand_index = {}.", address.get_hand_index()));
                            self.add_hand(piece);//[address.get_hand_index()] += 1;
                        },
                        None => {
                            // 指には何も持ってないので、駒台の駒をつかむ。
                            let piece_opt = address.get_hand_piece();
                            self.board[SKY_ADDRESS] = piece_opt;
                            self.remove_hand(piece_opt.unwrap());// .hands[address.get_hand_index()] -= 1;
                        },
                    }
                }
            },
            None => {
                if physical_move.phase_change {
                    // TODO phase change.
                    use piece_etc::Phase::*;
                    self.phase = match self.phase {
                        First => {Second},
                        Second => {First},
                    };
                } else if let Some(piece) = self.board[SKY_ADDRESS] {
                    if physical_move.sky_turn {
                        self.board[SKY_ADDRESS] = promotion_piece(Some(piece));
                    } else if physical_move.sky_rotate {
                        self.board[SKY_ADDRESS] = rotate_piece(Some(piece));
                    };
                }
            }
        }
    }

    /// 持ち駒の１行表示
    fn to_hand_text(&self, _comm:&Communication, phase_opt:Option<Phase>, piece_type:PieceType) -> String {
        
        let piece = piece_type_to_piece(phase_opt, piece_type);
        let count = self.get_hand(piece);
        let coefficient = if 1 < count {count.to_string()} else {"".to_string()};
        // comm.println(&format!("piece_type: '{}', hand_count: {}, coefficient: {}.", piece_type_to_sign(Some(piece_type)), count, coefficient));
        let ch = if 0 < count {
            piece_type_to_sign(Some(piece_type))
        } else {
            "".to_string()
        };

        format!("{}{}", coefficient, ch)
    }

    /// Point of symmetory.
    pub fn to_text(&self, comm:&Communication, phase:Phase) -> String {
        use piece_etc::Phase::*;
        let mut content = String::new();

        // First phase hand.
        Parser::appendln(&mut content, &format!("               {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>3}",
            self.to_hand_text(comm, Some(Phase::First), PieceType::K),
            self.to_hand_text(comm, Some(Phase::First), PieceType::R),
            self.to_hand_text(comm, Some(Phase::First), PieceType::B),
            self.to_hand_text(comm, Some(Phase::First), PieceType::G),
            self.to_hand_text(comm, Some(Phase::First), PieceType::S),
            self.to_hand_text(comm, Some(Phase::First), PieceType::N),
            self.to_hand_text(comm, Some(Phase::First), PieceType::L),
            self.to_hand_text(comm, Some(Phase::First), PieceType::P)));

        match phase {
            First => {
                // hand.
                Parser::appendln(&mut content, &"|         |  +----+----+----+----+----+----+----+----+----+".to_string());
            },
            Second => {
                Parser::appendln(&mut content, &"             +----+----+----+----+----+----+----+----+----+".to_string());
            },
        }

        for row in 0..=16 {
            let rank = 9 - (row/2);

            // 先手の手。
            match phase {
                First => {
                    match row {
                        0 => {Parser::append(&mut content, &"|         | ".to_string())},
                        1 => {Parser::append(&mut content, &"+---+ +-+ | ".to_string())},
                        2 => {Parser::append(&mut content, &"    | | | | ".to_string())},
                        3 => {Parser::append(&mut content, &"    | | | | ".to_string())},
                        4 => {Parser::append(&mut content, &"    +-+ +-+ ".to_string())},
                        5 => {Parser::append(&mut content, &format!("     {}   ", piece_to_display(self.get_piece_by_address(SKY_ADDRESS))))},
                        6|7|8|9|10|11|12|13|14|15|16 => {Parser::append(&mut content, &"            ".to_string())},
                        _ => { panic!("Unexpected row: {0}.", row) },
                    };
                },
                Second => {Parser::append(&mut content, &"            ".to_string())},
            }

            if row%2==0 {
                Parser::append(&mut content, &format!(
                    // 全角文字が混ざると、文字数では横幅調整できない。
                    // "{0}|{1:<4}{2:<4}{3:<4}{4:<4}{5:<4}{6:<4}{7:<4}{8:<4}{9:<4}",
                    "{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|",
                    rank, // Parser::i8_to_rank_char(rank),
                    piece_to_display(self.get_piece(1, rank)), // piece_to_sign
                    piece_to_display(self.get_piece(2, rank)),
                    piece_to_display(self.get_piece(3, rank)),
                    piece_to_display(self.get_piece(4, rank)),
                    piece_to_display(self.get_piece(5, rank)),
                    piece_to_display(self.get_piece(6, rank)),
                    piece_to_display(self.get_piece(7, rank)),
                    piece_to_display(self.get_piece(8, rank)),
                    piece_to_display(self.get_piece(9, rank))));
            } else {
                Parser::append(&mut content, &" +----+----+----+----+----+----+----+----+----+".to_string());
            }

            // Right boarder and None phase hands.
            match row {
                0|1|2|3|4|5|6|7|8 => {Parser::append(&mut content, &"   ".to_string())},
                9 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::K)))},
                10 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::R)))},
                11 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::B)))},
                12 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::G)))},
                13 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::S)))},
                14 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::N)))},
                15 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::L)))},
                16 => {Parser::append(&mut content, &format!("{:>3}", self.to_hand_text(comm, None, PieceType::P)))},
                _ => { panic!("Unexpected row: {0}.", row) },
            };

            // Second player finger.
            match phase {
                First => {},
                Second => {
                    match row {
                        0|1|2|3|4|5|6|7|8|9|10 => {},
                        11 => {Parser::append(&mut content, &format!("  {}", piece_to_display(self.get_piece_by_address(SKY_ADDRESS))))},
                        12 => {Parser::append(&mut content, &" +-+ +-+".to_string())},
                        13 => {Parser::append(&mut content, &" | | | |".to_string())},
                        14 => {Parser::append(&mut content, &" | | | |".to_string())},
                        15 => {Parser::append(&mut content, &" | +-+ +---+".to_string())},
                        16 => {Parser::append(&mut content, &" |         |".to_string())},
                        _ => { panic!("Unexpected row: {0}.", row) },
                    };
                },
            }

            Parser::append_ln(&mut content);
        }

        match phase {
            First => {
                Parser::appendln(&mut content, &"             +----+----+----+----+----+----+----+----+----+".to_string());
            },
            Second => {
                // hand.
                Parser::appendln(&mut content, &"             +----+----+----+----+----+----+----+----+----+    |         |".to_string());
            },
        }

        Parser::appendln(&mut content, &"              1    2    3    4    5    6    7    8    9".to_string());

        // Second phase hand.
        Parser::appendln(&mut content, &format!("               {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>3}",
            self.to_hand_text(comm, Some(Phase::Second), PieceType::K),
            self.to_hand_text(comm, Some(Phase::Second), PieceType::R),
            self.to_hand_text(comm, Some(Phase::Second), PieceType::B),
            self.to_hand_text(comm, Some(Phase::Second), PieceType::G),
            self.to_hand_text(comm, Some(Phase::Second), PieceType::S),
            self.to_hand_text(comm, Some(Phase::Second), PieceType::N),
            self.to_hand_text(comm, Some(Phase::Second), PieceType::L),
            self.to_hand_text(comm, Some(Phase::Second), PieceType::P)));

        content
    }
}