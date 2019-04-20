use address::*;
use board_size::*;
use communication::*;
use parser::*;
use piece_etc::*;
use rpm_conv::thread::rpm_note_operation::*;
use std::*;

pub const BOARD_START: usize = 0;
pub const DEFAULT_BOARD_SIZE: usize = (DEFAULT_FILE_LEN * DEFAULT_RANK_LEN) as usize;
pub const HANDS_LEN: usize = 3 * 8;

/// 指先。
pub struct Sky {
    id_piece: IdentifiedPiece,
    previous_address: Address,
}
impl Sky {
    pub fn from_idp_prev(idp: IdentifiedPiece, prev: Address) -> Self {
        Sky {
            id_piece: idp,
            previous_address: prev,
        }
    }

    pub fn set_sky(&mut self, id_piece_opt: IdentifiedPiece, previous_address_opt: Address) {
        self.id_piece = id_piece_opt;
        self.previous_address = previous_address_opt;
    }

    pub fn turn_over(&mut self) {
        self.id_piece.turn_over();
    }

    pub fn rotate(&mut self) {
        self.id_piece.rotate();
    }

    pub fn get_sky(&self) -> (IdentifiedPiece, Address) {
        (self.id_piece, self.previous_address)
    }

    pub fn get_idp(&self) -> IdentifiedPiece {
        self.id_piece
    }

    pub fn get_prev(&self) -> Address {
        self.previous_address
    }
}

pub struct Position {
    phase: Phase,
    board_size: BoardSize,
    pub board: [Option<IdentifiedPiece>; DEFAULT_BOARD_SIZE],
    pub hands: [Vec<IdentifiedPiece>; HANDS_LEN],
    pub sky: Option<Sky>,
}
impl Position {
    pub fn default() -> Position {
        //println!("#Position: default.");
        // このあと すぐリセットする。
        let mut instance = Position {
            phase: Phase::First,
            board_size: BoardSize::create_hon_shogi(),
            board: [None; DEFAULT_BOARD_SIZE],
            hands: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
            sky: None,
        };

        instance.reset_origin_position();
        instance
    }

    /// 自分の駒を持ち駒として持っているところから始めます。
    pub fn reset_origin_position(&mut self) {
        //println!("#Position: reset_origin_position.");
        self.phase = Phase::First;
        self.board_size = BoardSize::create_hon_shogi();
        self.board = [None; DEFAULT_BOARD_SIZE];
        self.hands = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        use piece_etc::IdentifiedPiece;
        use piece_etc::Phase::*;
        use piece_etc::Piece::*;
        use piece_etc::PieceIdentify::*;
        // 玉2枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(K2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, K00));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(K1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, K01));
        }
        // 飛2枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(R2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, R20));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(R1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, R21));
        }
        // 角2枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(B2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, B18));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(B1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, B19));
        }
        // 金4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(G2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, G02));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, G04));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(G1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, G03));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, G05));
        }
        // 銀4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(S2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, S06));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, S08));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(S1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, S07));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, S09));
        }
        // 桂4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(N2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, N10));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, N12));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(N1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, N11));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, N13));
        }
        // 香4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(L2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, L14));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, L16));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(L1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, L15));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, L17));
        }
        // 歩18枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(P2).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P22));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P24));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P26));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P28));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P30));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P32));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P34));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P36));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P38));
        }
        {
            let vec = &mut self.hands[HandIndex::from_piece(P1).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P23));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P25));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P27));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P29));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P31));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P33));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P35));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P37));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P39));
        }
    }

    /// ゲームに使う駒がまだ決まっていないところから始めます。
    pub fn reset_empty_position(&mut self) {
        //println!("#Position: reset_empty_position.");
        self.phase = Phase::First;
        self.board_size = BoardSize::create_hon_shogi();
        self.board = [None; DEFAULT_BOARD_SIZE];
        self.hands = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        use piece_etc::IdentifiedPiece;
        use piece_etc::Phase::*;
        use piece_etc::Piece::*;
        use piece_etc::PieceIdentify::*;
        // 玉2枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(K3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, K00));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, K01));
        }
        // 飛2枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(R3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, R20));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, R21));
        }
        // 角2枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(B3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, B18));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, B19));
        }
        // 金4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(G3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, G02));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, G04));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, G03));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, G05));
        }
        // 銀4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(S3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, S06));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, S08));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, S07));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, S09));
        }
        // 桂4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(N3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, N10));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, N12));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, N11));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, N13));
        }
        // 香4枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(L3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, L14));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, L16));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, L15));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, L17));
        }
        // 歩18枚。
        {
            let vec = &mut self.hands[HandIndex::from_piece(P3).get_index()];
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P22));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P24));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P26));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P28));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P30));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P32));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P34));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P36));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(Second), false, P38));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P23));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P25));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P27));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P29));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P31));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P33));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P35));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P37));
            vec.push(IdentifiedPiece::from_phase_pro_id(Some(First), false, P39));
        }
    }

    pub fn get_phase(&self) -> Phase {
        self.phase
    }

    pub fn get_board_size(&self) -> BoardSize {
        self.board_size
    }

    pub fn get_id_piece(&self, cell: Cell) -> Option<IdentifiedPiece> {
        let address = self.board_size.cell_to_address(cell);
        self.board[address]
    }

    pub fn get_id_piece_by_address(&self, address: usize) -> Option<IdentifiedPiece> {
        self.board[address]
    }

    pub fn get_cell_display_by_address(&self, address: Address) -> CellDisplay {
        if address.is_sky() {
            CellDisplay::from_idp(self.get_sky_idp())
        } else {
            CellDisplay::from_idp(self.board[address.get_index()])
        }
    }

    pub fn move_hand_to_finger(
        &mut self,
        address: Address,
        comm: &Communication,
        board_size: BoardSize,
    ) {
        let hand_index_obj = HandIndex::from_piece(address.get_hand_piece().unwrap());
        if let Some(id_piece) = self.hands[hand_index_obj.get_index()].pop() {
            self.sky = Some(Sky::from_idp_prev(id_piece, address));
        } else {
            let msg = format!(
                "駒台の駒が足りない！{}",
                address.to_log(board_size)
            );
            comm.println(&msg);
            panic!(msg);
        }
    }

    /// TODO 識別子を追加していいのか？
    /// Obsolute. new --> add().
    pub fn set_id_piece(&mut self, cell: Cell, id_piece: Option<IdentifiedPiece>) {
        let address = self.board_size.cell_to_address(cell);
        self.board[address] = id_piece;
    }

    /// TODO 識別子を消していいのか？
    pub fn remove_id_piece(&mut self, cell: Cell) -> Option<IdentifiedPiece> {
        let address = self.get_board_size().cell_to_address(cell);
        let id_piece = self.board[address];
        self.set_id_piece(cell, None);
        id_piece
    }

    pub fn get_hand_count(&self, piece: Piece) -> i8 {
        let hand_index_obj = HandIndex::from_piece(piece);
        self.hands[hand_index_obj.get_index()].len() as i8
    }

    pub fn search_hand(
        &self,
        ph_opt: Option<Phase>,
        pid: PieceIdentify,
    ) -> Option<IdentifiedPiece> {
        let pt = pid.get_piece_type();
        let pi = Piece::from_ph_pt(ph_opt, pt);

        let hand_index_obj = HandIndex::from_piece(pi);

        for idp in &self.hands[hand_index_obj.get_index()] {
            if idp.get_id() == pid {
                return Some(*idp);
            }
        }

        None
    }

    pub fn add_hand(&mut self, id_piece_opt: Option<IdentifiedPiece>) {
        if let Some(id_piece) = id_piece_opt {
            let hand_index = hand_id_piece_to_hand_index(id_piece);
            self.hands[hand_index].push(id_piece)
        }
    }

    pub fn remove_hand(&mut self, piece: Piece) -> IdentifiedPiece {
        let hand_index_obj = HandIndex::from_piece(piece);
        self.hands[hand_index_obj.get_index()].pop().unwrap()
    }

    pub fn peek_hand(&self, piece: Piece) -> Option<IdentifiedPiece> {
        let hand_index_obj = HandIndex::from_piece(piece);
        let stack = &self.hands[hand_index_obj.get_index()];
        if stack.is_empty() {
            None
        } else {
            Some(stack[stack.len() - 1])
        }
    }

    /// USI position 読込時に使う。使ってない駒を盤上に置く。
    pub fn activate_piece(&mut self, piece_opt: Option<Piece>, cell: Cell) {
        if let Some(piece) = piece_opt {
            let disactivate_piece = piece.to_disactivate();
            let hand_index_obj = HandIndex::from_piece(disactivate_piece);
            let id_piece = self.hands[hand_index_obj.get_index()].pop().unwrap();

            let destination = self.board_size.cell_to_address(cell);
            self.board[destination] = Some(id_piece);
        }
    }

    /// 指先に何か持っていれば真。
    pub fn get_sky_idp(&self) -> Option<IdentifiedPiece> {
        if let Some(ref sky) = self.sky {
            Some(sky.get_idp())
        } else {
            None
        }
    }

    /// 盤、駒台（Ａ）と、スカイ升（Ｂ）の間で駒を移動する。
    /// ＡとＢは、両方空っぽか、片方だけ駒があるかの　どちらかとする。両方に駒があるケースはないものとする。
    ///
    /// 棋譜には記録しない。
    ///
    /// トグルと考えてよい。もう一度実行すると、前の状態に戻ります。
    ///
    /// # Returns
    ///
    /// Is legal touch, Identified piece.
    pub fn touch_beautiful_1note(
        &mut self,
        rpm_operation_note: &RpmNoteOpe,
        comm: &Communication,
        board_size: BoardSize,
    ) -> (bool, Option<IdentifiedPiece>) {
        match rpm_operation_note.address {
            Some(address) => {
                // どこかを指定した。
                if address.is_on_board(self.board_size) {
                    // 盤上。
                    match self.board[address.get_index()] {
                        Some(board_id_piece) => {
                            // 盤上の駒と、指先の何かを入れ替えます。何かには None も含まれます。（非合法でも行います）

                            let tuple = if let Some(ref sky) = self.sky {
                                comm.println(&format!(
                                    "<IL-駒重なり{}>",
                                    address.to_log(board_size)
                                ));

                                // 違法。指に既に何か持ってた。
                                // 指に持っている駒を優先します。
                                (false, Some(sky.get_idp()))
                            } else {
                                // 合法。指が空いてたので駒をつかむ。
                                // 盤上の駒の方を優先します。
                                (true, Some(board_id_piece))
                            };

                            // スワップ。
                            // 盤上の何かを退避。
                            let tmp_board_idp_opt = self.board[address.get_index()];
                            // 盤上にスカイの何かを置く。
                            self.board[address.get_index()] = if let Some(ref sky) = self.sky {
                                Some(sky.id_piece)
                            } else {
                                None
                            };
                            // スカイに盤上の何かを置く。
                            self.sky = if let Some(tmp_board_idp) = tmp_board_idp_opt {
                                Some(Sky::from_idp_prev(tmp_board_idp, address))
                            } else {
                                None
                            };

                            tuple
                        }
                        None => {
                            // 盤上の None と、指先の何かを入れ替えます。何かには None も含まれます。（非合法でも行います）
                            let tuple = if let Some(ref sky) = self.sky {
                                // 駒を指につまんでいた。
                                // 合法。指につまんでいる駒を置く。
                                (true, Some(sky.get_idp()))
                            } else {
                                comm.println(&format!(
                                    "<IL-ほこり取り{}>",
                                    address.to_log(board_size)
                                ));
                                // ほこりを取る。
                                // 一応、違法。
                                (false, None)
                            };

                            // スワップ。
                            // 盤上の何かを退避。
                            let tmp_board_idp_opt = self.board[address.get_index()];
                            // 盤上にスカイの何かを置く。
                            self.board[address.get_index()] = if let Some(ref sky) = self.sky {
                                Some(sky.id_piece)
                            } else {
                                None
                            };
                            // スカイに盤上の何かを置く。
                            self.sky = if let Some(tmp_board_idp) = tmp_board_idp_opt {
                                Some(Sky::from_idp_prev(tmp_board_idp, address))
                            } else {
                                None
                            };

                            tuple
                        }
                    }
                // 駒台。
                } else {
                    if let Some(sky_idp) = self.get_sky_idp() {
                        // 指に何か持っていた。
                        // 合法。駒台に置く。
                        let id_piece_opt = Some(sky_idp);
                        // comm.println(&format!("hand_index = {}.", address.get_hand_index()));
                        self.add_hand(id_piece_opt);
                        self.sky = None;

                        (true, Some(sky_idp))
                    } else {
                        // 盤上ではなく、指には何も持ってない。駒台の駒をつかむ。
                        self.move_hand_to_finger(address, comm, board_size);
                        if let Some(ref sky) = self.sky {
                            // 合法。掴んだ駒を返す。
                            (true, Some(sky.get_idp()))
                        } else {
                            comm.println(&format!(
                                "<IL-駒台ほこり取り{}>",
                                address.to_log(board_size)
                            ));
                            // 違法。駒台のほこりを取った。
                            (false, None)
                        }
                    }
                }
            }
            None => {
                // 盤上や駒台の、どこも指していない。
                if rpm_operation_note.is_phase_change() {
                    // 合法。 phase change.
                    use piece_etc::Phase::*;
                    self.phase = match self.phase {
                        First => Second,
                        Second => First,
                    };
                    (true, None)
                } else if let Some(ref mut sky) = self.sky {
                    // 指に何か持っている。
                    if rpm_operation_note.sky_turn {
                        // 合法。成りの操作。
                        sky.turn_over();
                    } else if rpm_operation_note.sky_rotate {
                        // 合法。先後入れ替えの操作。
                        sky.rotate();
                    };
                    (true, Some(sky.get_idp()))
                } else {
                    comm.println("<IL-使っていない空間ほこり取り>");
                    // TODO 未定義の操作。投了とか？
                    // 一応、違法。
                    (false, None)
                }
            }
        }
    }

    /// 駒の検索。
    ///
    /// # Returns
    ///
    /// 識別駒、番地。
    pub fn find_wild(
        &self,
        ph_opt: Option<Phase>,
        pid: PieceIdentify,
    ) -> Option<(IdentifiedPiece, Address)> {
        // 盤上のスキャン。
        for addr in BOARD_START..self.board_size.len() {
            // Id piece.
            if let Some(idp) = self.board[addr] {
                if idp.get_phase() == ph_opt && idp.get_id() == pid {
                    // 駒の先後と、背番号が一致したら。
                    let addr_obj = Address::from_raw(addr);
                    return Some((idp, addr_obj));
                }
            }
        }

        // TODO 駒台のスタックの先頭かどうか分からない。あとで直すことにして　とりあえず。
        if let Some(idp) = self.search_hand(ph_opt, pid) {
            let addr_obj = Address::from_hand_ph_pt(ph_opt, idp.get_type());
            Some((idp, addr_obj))
        } else {
            None
        }
    }

    /// 持ち駒の１行表示
    pub fn to_hand_text(&self, phase_opt: Option<Phase>) -> String {
        let mut text = String::new();

        use piece_etc::Phase::*;
        use piece_etc::Piece::*;

        let array = if let Some(phase) = phase_opt {
            match phase {
                First => [K1, R1, B1, G1, S1, N1, L1, P1],
                Second => [K2, R2, B2, G2, S2, N2, L2, P2],
            }
        } else {
            [K3, R3, B3, G3, S3, N3, L3, P3]
        };

        for piece in &array {
            let hand_index_obj = HandIndex::from_piece(*piece);
            for id_piece in self.hands[hand_index_obj.get_index()].iter() {
                text = format!("{} {}", text, id_piece.to_human_presentable())
            }
        }

        text
    }

    /// Point of symmetory.
    pub fn to_text(&self, _comm: &Communication, phase: Phase) -> String {
        use piece_etc::Phase::*;
        let mut content = String::new();

        // First phase hand.
        Parser::appendln(
            &mut content,
            &format!("               {}", self.to_hand_text(Some(Phase::First))),
        );

        match phase {
            First => {
                // hand.
                Parser::appendln(
                    &mut content,
                    &"|         |  +----+----+----+----+----+----+----+----+----+".to_string(),
                );
            }
            Second => {
                Parser::appendln(
                    &mut content,
                    &"             +----+----+----+----+----+----+----+----+----+".to_string(),
                );
            }
        }

        for row in 0..=16 {
            // let rank = row / 2 + 1;
            let rank = 9 - (row / 2);

            // 先手の手。
            match phase {
                First => {
                    match row {
                        0 => Parser::append(&mut content, &"|         | ".to_string()),
                        1 => Parser::append(&mut content, &"+---+ +-+ | ".to_string()),
                        2 => Parser::append(&mut content, &"    | | | | ".to_string()),
                        3 => Parser::append(&mut content, &"    | | | | ".to_string()),
                        4 => Parser::append(&mut content, &"    +-+ +-+ ".to_string()),
                        5 => Parser::append(
                            &mut content,
                            &format!(
                                "     {}   ",
                                self.get_cell_display_by_address(Address::from_sky())
                                    .to_display()
                            ),
                        ),
                        6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => {
                            Parser::append(&mut content, &"            ".to_string())
                        }
                        _ => panic!("Unexpected row: {0}.", row),
                    };
                }
                Second => Parser::append(&mut content, &"            ".to_string()),
            }

            if row % 2 == 0 {
                Parser::append(
                    &mut content,
                    &format!(
                        // 全角文字が混ざると、文字数では横幅調整できない。
                        // "{0}|{1:<4}{2:<4}{3:<4}{4:<4}{5:<4}{6:<4}{7:<4}{8:<4}{9:<4}",
                        "{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|",
                        rank, // Parser::i8_to_rank_char(rank),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(1, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(2, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(3, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(4, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(5, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(6, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(7, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(8, rank),
                            self.board_size
                        ))
                        .to_display(),
                        self.get_cell_display_by_address(Address::from_cell(
                            Cell::from_file_rank(9, rank),
                            self.board_size
                        ))
                        .to_display()
                    ),
                );
            } else {
                Parser::append(
                    &mut content,
                    &" +----+----+----+----+----+----+----+----+----+".to_string(),
                );
            }

            // Second player finger.
            match phase {
                First => {}
                Second => {
                    match row {
                        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => {}
                        11 => Parser::append(
                            &mut content,
                            &format!(
                                "  {}",
                                self.get_cell_display_by_address(Address::from_sky())
                                    .to_display()
                            ),
                        ),
                        12 => Parser::append(&mut content, &" +-+ +-+".to_string()),
                        13 => Parser::append(&mut content, &" | | | |".to_string()),
                        14 => Parser::append(&mut content, &" | | | |".to_string()),
                        15 => Parser::append(&mut content, &" | +-+ +---+".to_string()),
                        16 => Parser::append(&mut content, &" |         |".to_string()),
                        _ => panic!("Unexpected row: {0}.", row),
                    };
                }
            }

            Parser::append_ln(&mut content);
        }

        match phase {
            First => {
                Parser::appendln(
                    &mut content,
                    &"             +----+----+----+----+----+----+----+----+----+".to_string(),
                );
            }
            Second => {
                // hand.
                Parser::appendln(
                    &mut content,
                    &"             +----+----+----+----+----+----+----+----+----+ |         |"
                        .to_string(),
                );
            }
        }

        Parser::appendln(
            &mut content,
            &"              1    2    3    4    5    6    7    8    9".to_string(),
        );

        // Second phase hand.
        Parser::appendln(
            &mut content,
            &format!("               {}", self.to_hand_text(Some(Phase::Second))),
        );

        content
    }
}
