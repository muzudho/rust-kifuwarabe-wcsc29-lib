use address::*;
use communication::*;
use parser::*;
use piece_etc::*;
use rpm_conv::rpm_operation_note::*;
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
    pub board: [Option<IdentifiedPiece>; DEFAULT_BOARD_SIZE],
    pub hands: [Vec<IdentifiedPiece>; HANDS_LEN],
}
impl Position {
    pub fn default() -> Position {
        // このあと すぐリセットする。
        let mut instance = Position {
            phase: Phase::First,
            board_size: BoardSize::create_hon_shogi(),
            board: [None; DEFAULT_BOARD_SIZE],
            hands: [
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            ],
        };

        instance.reset_default();
        instance
    }

    pub fn reset_default(&mut self) {
        self.phase = Phase::First;
        self.board_size = BoardSize::create_hon_shogi();
        self.board = [None; DEFAULT_BOARD_SIZE];
        self.hands = [
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
        ];
        // 使わない駒
        use piece_etc::Piece::*;
        use piece_etc::PieceIdentify::*;
        use piece_etc::IdentifiedPiece;
        // 玉2枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(K3)];
            vec.push(IdentifiedPiece::create(None, false, K00));
            vec.push(IdentifiedPiece::create(None, false, K01));
        }
        // 飛2枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(R3)];
            vec.push(IdentifiedPiece::create(None, false, R20));
            vec.push(IdentifiedPiece::create(None, false, R21));
        }
        // 角2枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(B3)];
            vec.push(IdentifiedPiece::create(None, false, B18));
            vec.push(IdentifiedPiece::create(None, false, B19));
        }
        // 金4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(G3)];
            vec.push(IdentifiedPiece::create(None, false, G02));
            vec.push(IdentifiedPiece::create(None, false, G03));
            vec.push(IdentifiedPiece::create(None, false, G04));
            vec.push(IdentifiedPiece::create(None, false, G05));
        }
        // 銀4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(S3)];
            vec.push(IdentifiedPiece::create(None, false, S06));
            vec.push(IdentifiedPiece::create(None, false, S07));
            vec.push(IdentifiedPiece::create(None, false, S08));
            vec.push(IdentifiedPiece::create(None, false, S09));
        }
        // 桂4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(N3)];
            vec.push(IdentifiedPiece::create(None, false, N10));
            vec.push(IdentifiedPiece::create(None, false, N11));
            vec.push(IdentifiedPiece::create(None, false, N12));
            vec.push(IdentifiedPiece::create(None, false, N13));
        }
        // 香4枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(L3)];
            vec.push(IdentifiedPiece::create(None, false, L14));
            vec.push(IdentifiedPiece::create(None, false, L15));
            vec.push(IdentifiedPiece::create(None, false, L16));
            vec.push(IdentifiedPiece::create(None, false, L17));
        }
        // 歩18枚。
        {
            let vec = &mut self.hands[hand_piece_to_hand_index(P3)];
            vec.push(IdentifiedPiece::create(None, false, P22));
            vec.push(IdentifiedPiece::create(None, false, P23));
            vec.push(IdentifiedPiece::create(None, false, P24));
            vec.push(IdentifiedPiece::create(None, false, P25));
            vec.push(IdentifiedPiece::create(None, false, P26));
            vec.push(IdentifiedPiece::create(None, false, P27));
            vec.push(IdentifiedPiece::create(None, false, P28));
            vec.push(IdentifiedPiece::create(None, false, P29));
            vec.push(IdentifiedPiece::create(None, false, P30));
            vec.push(IdentifiedPiece::create(None, false, P31));
            vec.push(IdentifiedPiece::create(None, false, P32));
            vec.push(IdentifiedPiece::create(None, false, P33));
            vec.push(IdentifiedPiece::create(None, false, P34));
            vec.push(IdentifiedPiece::create(None, false, P35));
            vec.push(IdentifiedPiece::create(None, false, P36));
            vec.push(IdentifiedPiece::create(None, false, P37));
            vec.push(IdentifiedPiece::create(None, false, P38));
            vec.push(IdentifiedPiece::create(None, false, P39));
        }
    }

    pub fn reset_startpos(&mut self) {
        use piece_etc::IdentifiedPiece;
        use piece_etc::Phase::*;
        use piece_etc::PieceIdentify::*;
        self.board_size = BoardSize::create_hon_shogi();
        // Sente view to flip upside down.
        self.board = [
            // rank 1, file 1.
            IdentifiedPiece::some(Some(Second), false, L14), IdentifiedPiece::some(Some(Second), false, N10), IdentifiedPiece::some(Some(Second), false, S06), IdentifiedPiece::some(Some(Second), false, G02), IdentifiedPiece::some(Some(Second), false, K00), IdentifiedPiece::some(Some(Second), false, G04), IdentifiedPiece::some(Some(Second), false, S08), IdentifiedPiece::some(Some(Second), false, N12), IdentifiedPiece::some(Some(Second), false, L16),
            None, IdentifiedPiece::some(Some(Second), false, B18), None, None, None, None, None, IdentifiedPiece::some(Some(Second), false, R20), None,
            IdentifiedPiece::some(Some(Second), false, P36), IdentifiedPiece::some(Some(Second), false, P32), IdentifiedPiece::some(Some(Second), false, P28), IdentifiedPiece::some(Some(Second), false, P24), IdentifiedPiece::some(Some(Second), false, P22), IdentifiedPiece::some(Some(Second), false, P26), IdentifiedPiece::some(Some(Second), false, P30), IdentifiedPiece::some(Some(Second), false, P34), IdentifiedPiece::some(Some(Second), false, P38),
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            IdentifiedPiece::some(Some(First), false, P39), IdentifiedPiece::some(Some(First), false, P35), IdentifiedPiece::some(Some(First), false, P31), IdentifiedPiece::some(Some(First), false, P27), IdentifiedPiece::some(Some(First), false, P23), IdentifiedPiece::some(Some(First), false, P25), IdentifiedPiece::some(Some(First), false, P29), IdentifiedPiece::some(Some(First), false, P33), IdentifiedPiece::some(Some(First), false, P37),
            None, IdentifiedPiece::some(Some(First), false, R21), None, None, None, None, None, IdentifiedPiece::some(Some(First), false, B19), None,
            IdentifiedPiece::some(Some(First), false, L17), IdentifiedPiece::some(Some(First), false, N13), IdentifiedPiece::some(Some(First), false, S09), IdentifiedPiece::some(Some(First), false, G05), IdentifiedPiece::some(Some(First), false, K01), IdentifiedPiece::some(Some(First), false, G03), IdentifiedPiece::some(Some(First), false, S07), IdentifiedPiece::some(Some(First), false, N11), IdentifiedPiece::some(Some(First), false, L15),
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

    pub fn get_id_piece(&self, file:i8, rank:i8) -> Option<IdentifiedPiece> {
        let address = self.board_size.file_rank_to_cell(file, rank);
        self.board[address]
    }

    pub fn get_id_piece_by_address(&self, address:usize) -> Option<IdentifiedPiece> {
        self.board[address]
    }

    pub fn get_cell_thing_by_address(&self, address:Address) -> CellThing {
        CellThing::create(self.board[address.get_index()])
    }

    pub fn move_finger_to_hand(&mut self) {
        let id_piece = self.board[SKY_ADDRESS];
        // comm.println(&format!("hand_index = {}.", address.get_hand_index()));
        self.add_hand(id_piece);
        self.board[SKY_ADDRESS] = None;
    }

    pub fn move_hand_to_finger(&mut self, address:Address) {
        let hand_index = hand_piece_to_hand_index(address.get_hand_piece().unwrap());
        let id_piece = self.hands[hand_index].pop();
        self.board[SKY_ADDRESS] = id_piece;
    }

    /// TODO 識別子を追加していいのか？
    /// Obsolute. new --> add().
    pub fn set_id_piece(&mut self, file:i8, rank:i8, id_piece:Option<IdentifiedPiece>) {
        let cell = self.board_size.file_rank_to_cell(file, rank);
        self.board[cell] = id_piece;
    }

    /// TODO 識別子を消していいのか？
    pub fn remove_id_piece(&mut self, file:i8, rank:i8) -> Option<IdentifiedPiece> {
        let cell = self.get_board_size().file_rank_to_cell(file, rank);
        let id_piece = self.board[cell];
        self.set_id_piece(file, rank, None);
        id_piece
    }

    pub fn get_hand(&self, piece:Piece) -> i8 {
        let hand_index = hand_piece_to_hand_index(piece);
        self.hands[hand_index].len() as i8
    }

    pub fn add_hand(&mut self, id_piece_opt:Option<IdentifiedPiece>) {
        if let Some(id_piece) = id_piece_opt {
            let hand_index = hand_id_piece_to_hand_index(id_piece);
            self.hands[hand_index].push(id_piece)
        }
    }

    pub fn remove_hand(&mut self, piece:Piece) -> IdentifiedPiece {
        let hand_index = hand_piece_to_hand_index(piece);
        self.hands[hand_index].pop().unwrap()
    }

    /// USI position 読込時に使う。使ってない駒を盤上に置く。
    pub fn activate_piece(&mut self, piece_opt:Option<Piece>, file:i8, rank:i8) {
        if let Some(piece) = piece_opt {
            let disactivate_piece = piece_to_disactivate(piece);
            let hand_index = hand_piece_to_hand_index(disactivate_piece);
            let id_piece = self.hands[hand_index].pop().unwrap();

            let destination = self.board_size.file_rank_to_cell(file, rank);
            self.board[destination] = Some(id_piece);
        }
    }

    /// # Returns
    /// 
    /// Piece identify.
    pub fn touch(&mut self, _comm:&Communication, rpm_operation_note:&RpmNote) -> i16 {
        match rpm_operation_note.address {
            Some(address) => {
                // どこかを指定した。
                if address.is_on_board(self.board_size) {
                    // 盤上。
                    match self.board[address.get_index()] {
                        Some(board_id_piece) => {
                            // 駒の場所を指定した。
                            match self.board[SKY_ADDRESS] {
                                Some(sky_id_piece) => {
                                    // 指に何か持ってた。
                                    sky_id_piece.get_id_number()
                                },
                                None => {
                                    // 指が空いてたので駒をつかむ。
                                    self.board[SKY_ADDRESS] = Some(board_id_piece);
                                    self.board[address.get_index()] = None;
                                    board_id_piece.get_id_number()
                                },
                            }
                        },
                        None => {
                            // 空き升を指定した。
                            if let Some(sky_id_piece) = self.board[SKY_ADDRESS] {
                                // 指につまんでいる駒を置く。
                                self.board[SKY_ADDRESS] = None;
                                self.board[address.get_index()] = Some(sky_id_piece);
                                sky_id_piece.get_id_number()
                            } else {
                                -1
                            }
                        },
                    }
                // 駒台。
                } else if let Some(sky_id_piece) = self.board[SKY_ADDRESS] {
                    // 指に何か持っているので、駒台に置く。
                    self.move_finger_to_hand();
                    sky_id_piece.get_id_number()
                } else {
                    // 指には何も持ってないので、駒台の駒をつかむ。
                    self.move_hand_to_finger(address);
                    if let Some(sky_id_piece) = self.board[SKY_ADDRESS] {
                        sky_id_piece.get_id_number()
                    } else {
                        -1
                    }
                }
            },
            None => {
                if rpm_operation_note.is_phase_change() {
                    // TODO phase change.
                    use piece_etc::Phase::*;
                    self.phase = match self.phase {
                        First => {Second},
                        Second => {First},
                    };
                    -1
                } else if let Some(mut id_piece) = self.board[SKY_ADDRESS] {
                    if rpm_operation_note.sky_turn {
                        id_piece.turn_over();
                        self.board[SKY_ADDRESS] = Some(id_piece);
                    } else if rpm_operation_note.sky_rotate {
                        id_piece.rotate();
                        self.board[SKY_ADDRESS] = Some(id_piece);
                    };
                    id_piece.get_id_number()
                } else {
                    -1
                }
            }
        }
    }

    /// 持ち駒の１行表示
    pub fn to_hand_text(&self, phase_opt:Option<Phase>) -> String {
        let mut text = String::new();

        use piece_etc::Phase::*;
        use piece_etc::Piece::*;

        let array = if let Some(phase) = phase_opt {
            match phase {
                First => {
                    [K1, R1, B1, G1, S1, N1, L1, P1]
                },
                Second => {
                    [K2, R2, B2, G2, S2, N2, L2, P2]
                },
            }
        } else {
            [K3, R3, B3, G3, S3, N3, L3, P3]
        };

        for piece in &array {
            let hand_index = hand_piece_to_hand_index(*piece);
            for id_piece in self.hands[hand_index].iter() {
                text = format!("{} {}", text, id_piece.to_physical_display())
            }
        }

        text
    }

    /// Point of symmetory.
    pub fn to_text(&self, _comm:&Communication, phase:Phase) -> String {
        use piece_etc::Phase::*;
        let mut content = String::new();

        // First phase hand.
        Parser::appendln(&mut content, &format!("               {}",
            self.to_hand_text(Some(Phase::First))));

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
            // let rank = row / 2 + 1;
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
                        5 => {Parser::append(&mut content, &format!("     {}   ", self.get_cell_thing_by_address(Address::create_as_sky()).to_display()))},
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
                    self.get_cell_thing_by_address(Address::create_by_cell(1, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(2, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(3, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(4, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(5, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(6, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(7, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(8, rank, self.board_size)).to_display(),
                    self.get_cell_thing_by_address(Address::create_by_cell(9, rank, self.board_size)).to_display()));
            } else {
                Parser::append(&mut content, &" +----+----+----+----+----+----+----+----+----+".to_string());
            }

            // Second player finger.
            match phase {
                First => {},
                Second => {
                    match row {
                        0|1|2|3|4|5|6|7|8|9|10 => {},
                        11 => {Parser::append(&mut content, &format!("  {}", self.get_cell_thing_by_address(Address::create_as_sky()).to_display()))},
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
                Parser::appendln(&mut content, &"             +----+----+----+----+----+----+----+----+----+ |         |".to_string());
            },
        }

        Parser::appendln(&mut content, &"              1    2    3    4    5    6    7    8    9".to_string());

        // Second phase hand.
        Parser::appendln(&mut content, &format!("               {}",
            self.to_hand_text(Some(Phase::Second))));

        content
    }
}