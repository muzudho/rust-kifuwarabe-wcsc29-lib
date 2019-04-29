use instrument::position::*;
use std::fmt;
use std::slice::Iter;
use studio::address::*;
use studio::board_size::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    /// Starting first.
    First,
    /// Starting second.
    Second,
}
impl Phase {
    /// Human presentalbe.
    pub fn to_log(self) -> String {
        use instrument::piece_etc::Phase::*;
        match self {
            First => "▼",
            Second => "△",
        }
        .to_string()
    }

    pub fn to_sign(self) -> String {
        use instrument::piece_etc::Phase::*;
        match self {
            First => "b",
            Second => "w",
        }
        .to_string()
    }
}

/// Piece identify. Order of "大橋"(Ohashi) mode.
/// With out phase.
#[derive(Clone, Copy, PartialEq)]
pub enum PieceIdentify {
    K00,
    K01,
    G02,
    G03,
    G04,
    G05,
    S06,
    S07,
    S08,
    S09,
    N10,
    N11,
    N12,
    N13,
    L14,
    L15,
    L16,
    L17,
    B18,
    B19,
    R20,
    R21,
    P22,
    P23,
    P24,
    P25,
    P26,
    P27,
    P28,
    P29,
    P30,
    P31,
    P32,
    P33,
    P34,
    P35,
    P36,
    P37,
    P38,
    P39,
}
impl fmt::Display for PieceIdentify {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_human_presentable())
    }
}
impl PieceIdentify {
    pub fn iterator() -> Iter<'static, PieceIdentify> {
        use instrument::piece_etc::PieceIdentify::*;
        static PIECE_IDENTIFIES: [PieceIdentify; 40] = [
            K00, K01, G02, G03, G04, G05, S06, S07, S08, S09, N10, N11, N12, N13, L14, L15, L16,
            L17, B18, B19, R20, R21, P22, P23, P24, P25, P26, P27, P28, P29, P30, P31, P32, P33,
            P34, P35, P36, P37, P38, P39,
        ];
        PIECE_IDENTIFIES.iter()
    }

    pub fn from_number(number: i8) -> Option<PieceIdentify> {
        use instrument::piece_etc::PieceIdentify::*;
        match number {
            0 => Some(K00),
            1 => Some(K01),
            2 => Some(G02),
            3 => Some(G03),
            4 => Some(G04),
            5 => Some(G05),
            6 => Some(S06),
            7 => Some(S07),
            8 => Some(S08),
            9 => Some(S09),
            10 => Some(N10),
            11 => Some(N11),
            12 => Some(N12),
            13 => Some(N13),
            14 => Some(L14),
            15 => Some(L15),
            16 => Some(L16),
            17 => Some(L17),
            18 => Some(B18),
            19 => Some(B19),
            20 => Some(R20),
            21 => Some(R21),
            22 => Some(P22),
            23 => Some(P23),
            24 => Some(P24),
            25 => Some(P25),
            26 => Some(P26),
            27 => Some(P27),
            28 => Some(P28),
            29 => Some(P29),
            30 => Some(P30),
            31 => Some(P31),
            32 => Some(P32),
            33 => Some(P33),
            34 => Some(P34),
            35 => Some(P35),
            36 => Some(P36),
            37 => Some(P37),
            38 => Some(P38),
            39 => Some(P39),
            _ => None,
        }
    }

    pub fn get_number(self) -> i8 {
        use instrument::piece_etc::PieceIdentify::*;
        match self {
            K00 => 0,
            K01 => 1,
            G02 => 2,
            G03 => 3,
            G04 => 4,
            G05 => 5,
            S06 => 6,
            S07 => 7,
            S08 => 8,
            S09 => 9,
            N10 => 10,
            N11 => 11,
            N12 => 12,
            N13 => 13,
            L14 => 14,
            L15 => 15,
            L16 => 16,
            L17 => 17,
            B18 => 18,
            B19 => 19,
            R20 => 20,
            R21 => 21,
            P22 => 22,
            P23 => 23,
            P24 => 24,
            P25 => 25,
            P26 => 26,
            P27 => 27,
            P28 => 28,
            P29 => 29,
            P30 => 30,
            P31 => 31,
            P32 => 32,
            P33 => 33,
            P34 => 34,
            P35 => 35,
            P36 => 36,
            P37 => 37,
            P38 => 38,
            P39 => 39,
        }
    }

    /// 背番号からは、先後は分からない。
    pub fn get_piece_type(self) -> PieceType {
        use instrument::piece_etc::PieceIdentify::*;
        use instrument::piece_etc::PieceType::*;
        match self {
            K00 | K01 => K,
            R20 | R21 => R,
            B18 | B19 => B,
            G02 | G03 | G04 | G05 => G,
            S06 | S07 | S08 | S09 => S,
            N10 | N11 | N12 | N13 => N,
            L14 | L15 | L16 | L17 => L,
            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32 | P33 | P34 | P35
            | P36 | P37 | P38 | P39 => P,
        }
    }

    pub fn to_human_presentable(self) -> String {
        use instrument::piece_etc::PieceIdentify::*;
        match self {
            K00 => "王00",
            K01 => "玉01",
            G02 => "金02",
            G03 => "金03",
            G04 => "金04",
            G05 => "金05",
            S06 => "銀06",
            S07 => "銀07",
            S08 => "銀08",
            S09 => "銀09",
            N10 => "桂10",
            N11 => "桂11",
            N12 => "桂12",
            N13 => "桂13",
            L14 => "香14",
            L15 => "香15",
            L16 => "香16",
            L17 => "香17",
            B18 => "角18",
            B19 => "角19",
            R20 => "飛20",
            R21 => "飛21",
            P22 => "歩22",
            P23 => "歩23",
            P24 => "歩24",
            P25 => "歩25",
            P26 => "歩26",
            P27 => "歩27",
            P28 => "歩28",
            P29 => "歩29",
            P30 => "歩30",
            P31 => "歩31",
            P32 => "歩32",
            P33 => "歩33",
            P34 => "歩34",
            P35 => "歩35",
            P36 => "歩36",
            P37 => "歩37",
            P38 => "歩38",
            P39 => "歩39",
            /*
            K00 => "K00",
            K01 => "K01",
            G02 => "G02",
            G03 => "G03",
            G04 => "G04",
            G05 => "G05",
            S06 => "S06",
            S07 => "S07",
            S08 => "S08",
            S09 => "S09",
            N10 => "N10",
            N11 => "N11",
            N12 => "N12",
            N13 => "N13",
            L14 => "L14",
            L15 => "L15",
            L16 => "L16",
            L17 => "L17",
            B18 => "B18",
            B19 => "B19",
            R20 => "R20",
            R21 => "R21",
            P22 => "P22",
            P23 => "P23",
            P24 => "P24",
            P25 => "P25",
            P26 => "P26",
            P27 => "P27",
            P28 => "P28",
            P29 => "P29",
            P30 => "P30",
            P31 => "P31",
            P32 => "P32",
            P33 => "P33",
            P34 => "P34",
            P35 => "P35",
            P36 => "P36",
            P37 => "P37",
            P38 => "P38",
            P39 => "P39",
            */
        }
        .to_string()
    }
}

/// For cell display.
pub struct CellDisplay {
    id_piece_opt: Option<IdentifiedPiece>,
    // 指先に、どこから取った駒か覚えておく。
    previous_address: Option<Address>,
}
impl CellDisplay {
    pub fn from_idp(idp_opt: Option<IdentifiedPiece>) -> CellDisplay {
        CellDisplay {
            id_piece_opt: idp_opt,
            previous_address: None,
        }
    }

    pub fn from_idp_prev(idp_opt: Option<IdentifiedPiece>, prev: Address) -> CellDisplay {
        CellDisplay {
            id_piece_opt: idp_opt,
            previous_address: Some(prev),
        }
    }

    pub fn from_empty_fingertip() -> CellDisplay {
        CellDisplay {
            id_piece_opt: None,
            previous_address: None,
        }
    }

    /// 横幅は半角4文字。
    /// 逆さにできないから、半角カナにしているだけ☆（＾～＾）右側のスペースに18進数の背番号が入る予定☆（＾～＾）
    pub fn to_display(&self) -> String {
        if let Some(id_piece) = self.id_piece_opt {
            id_piece.to_human_presentable()
        } else {
            // 空セル☆（＾～＾）
            "    ".to_string()
        }
    }

    /// 3桁追加して 7桁に揃える。
    pub fn to_fingertip_display(&self, board_size: BoardSize) -> String {
        if let Some(prev) = self.previous_address {
            // 駒のIDの方は桁数指定すると、ずれるからしてない。
            format!(
                "{}:{:>2}",
                self.to_display(),
                prev.to_physical_sign(board_size)
            )
        } else {
            format!("{}   ", self.to_display())
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct IdentifiedPiece {
    phase: Option<Phase>,
    promoted: bool,
    id: PieceIdentify,
}
impl IdentifiedPiece {
    pub fn from_phase_pro_id(
        phase_opt: Option<Phase>,
        promoted_flag: bool,
        piece_id: PieceIdentify,
    ) -> IdentifiedPiece {
        IdentifiedPiece {
            phase: phase_opt,
            promoted: promoted_flag,
            id: piece_id,
        }
    }

    pub fn some(
        phase_opt: Option<Phase>,
        promoted_flag: bool,
        piece_id: PieceIdentify,
    ) -> Option<IdentifiedPiece> {
        Some(IdentifiedPiece::from_phase_pro_id(
            phase_opt,
            promoted_flag,
            piece_id,
        ))
    }

    pub fn turn_over(&mut self) {
        self.promoted = !self.promoted;
    }

    pub fn rotate(&mut self) {
        use instrument::piece_etc::Phase::*;
        if let Some(phase) = self.phase {
            self.phase = match phase {
                First => Some(Second),
                Second => Some(First),
            }
        }
    }

    pub fn get_phase(self) -> Option<Phase> {
        self.phase
    }

    pub fn is_promoted(self) -> bool {
        self.promoted
    }

    // 相手の駒なら真。
    pub fn is_opponent(self, position: &Position) -> Option<bool> {
        if let Some(ph) = self.get_phase() {
            Some(ph != position.get_phase())
        } else {
            None
        }
    }

    pub fn get_id(self) -> PieceIdentify {
        self.id
    }

    pub fn get_type(self) -> PieceType {
        use instrument::piece_etc::PieceIdentify::*;
        use instrument::piece_etc::PieceType::*;
        if self.promoted {
            match self.id {
                K00 | K01 => PK,
                R20 | R21 => PR,
                B18 | B19 => PB,
                G02 | G03 | G04 | G05 => PG,
                S06 | S07 | S08 | S09 => PS,
                N10 | N11 | N12 | N13 => PN,
                L14 | L15 | L16 | L17 => PL,
                P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32 | P33 | P34
                | P35 | P36 | P37 | P38 | P39 => PP,
            }
        } else {
            match self.id {
                K00 | K01 => K,
                R20 | R21 => R,
                B18 | B19 => B,
                G02 | G03 | G04 | G05 => G,
                S06 | S07 | S08 | S09 => S,
                N10 | N11 | N12 | N13 => N,
                L14 | L15 | L16 | L17 => L,
                P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32 | P33 | P34
                | P35 | P36 | P37 | P38 | P39 => P,
            }
        }
    }

    pub fn to_human_presentable(self) -> String {
        use instrument::piece_etc::Phase::*;
        use instrument::piece_etc::PieceIdentify::*;
        if let Some(phase) = self.get_phase() {
            match phase {
                First => {
                    if self.is_promoted() {
                        match self.get_id() {
                            K00 => "IK00",
                            K01 => "IK01",
                            G02 => "IM02",
                            G03 => "IM03",
                            G04 => "IM04",
                            G05 => "IM05",
                            S06 => "NG06",
                            S07 => "NG07",
                            S08 => "NG08",
                            S09 => "NG09",
                            N10 => "NK10",
                            N11 => "NK11",
                            N12 => "NK12",
                            N13 => "NK13",
                            L14 => "NY14",
                            L15 => "NY15",
                            L16 => "NY16",
                            L17 => "NY17",
                            B18 => "UM18",
                            B19 => "UM19",
                            R20 => "RY20",
                            R21 => "RY21",
                            P22 => "TO22",
                            P23 => "TO23",
                            P24 => "TO24",
                            P25 => "TO25",
                            P26 => "TO26",
                            P27 => "TO27",
                            P28 => "TO28",
                            P29 => "TO29",
                            P30 => "TO30",
                            P31 => "TO31",
                            P32 => "TO32",
                            P33 => "TO33",
                            P34 => "TO34",
                            P35 => "TO35",
                            P36 => "TO36",
                            P37 => "TO37",
                            P38 => "TO38",
                            P39 => "TO39",
                        }
                        .to_string()
                    } else {
                        match self.get_id() {
                            K00 => "OU00",
                            K01 => "OU01",
                            G02 => "KI02",
                            G03 => "KI03",
                            G04 => "KI04",
                            G05 => "KI05",
                            S06 => "GI06",
                            S07 => "GI07",
                            S08 => "GI08",
                            S09 => "GI09",
                            N10 => "KE10",
                            N11 => "KE11",
                            N12 => "KE12",
                            N13 => "KE13",
                            L14 => "KY14",
                            L15 => "KY15",
                            L16 => "KY16",
                            L17 => "KY17",
                            B18 => "KA18",
                            B19 => "KA19",
                            R20 => "HI20",
                            R21 => "HI21",
                            P22 => "FU22",
                            P23 => "FU23",
                            P24 => "FU24",
                            P25 => "FU25",
                            P26 => "FU26",
                            P27 => "FU27",
                            P28 => "FU28",
                            P29 => "FU29",
                            P30 => "FU30",
                            P31 => "FU31",
                            P32 => "FU32",
                            P33 => "FU33",
                            P34 => "FU34",
                            P35 => "FU35",
                            P36 => "FU36",
                            P37 => "FU37",
                            P38 => "FU38",
                            P39 => "FU39",
                        }
                        .to_string()
                    }
                }
                Second => {
                    if self.is_promoted() {
                        match self.get_id() {
                            K00 => "生00",
                            K01 => "生01",
                            G02 => "今02",
                            G03 => "今03",
                            G04 => "今04",
                            G05 => "今05",
                            S06 => "全06",
                            S07 => "全07",
                            S08 => "全08",
                            S09 => "全09",
                            N10 => "圭10",
                            N11 => "圭11",
                            N12 => "圭12",
                            N13 => "圭13",
                            L14 => "杏14",
                            L15 => "杏15",
                            L16 => "杏16",
                            L17 => "杏17",
                            B18 => "馬18",
                            B19 => "馬19",
                            R20 => "竜20",
                            R21 => "竜21",
                            P22 => "と22",
                            P23 => "と23",
                            P24 => "と24",
                            P25 => "と25",
                            P26 => "と26",
                            P27 => "と27",
                            P28 => "と28",
                            P29 => "と29",
                            P30 => "と30",
                            P31 => "と31",
                            P32 => "と32",
                            P33 => "と33",
                            P34 => "と34",
                            P35 => "と35",
                            P36 => "と36",
                            P37 => "と37",
                            P38 => "と38",
                            P39 => "と39",
                        }
                        .to_string()
                    } else {
                        // 成りや、先後を含まない表示。
                        self.get_id().to_human_presentable()
                    }
                }
            }
        } else if self.is_promoted() {
            // 使っていない駒として表示するぜ☆（＾～＾）
            match self.get_id() {
                // ナリオウ
                K00 => "ﾅｵ00",
                K01 => "ﾅｵ01",
                // ナリキン
                G02 => "ﾅｷ02",
                G03 => "ﾅｷ03",
                G04 => "ﾅｷ04",
                G05 => "ﾅｷ05",
                // ナリシルバー
                S06 => "ﾅｼ06",
                S07 => "ﾅｼ07",
                S08 => "ﾅｼ08",
                S09 => "ﾅｼ09",
                // ナリケイ
                N10 => "ﾅｹ10",
                N11 => "ﾅｹ11",
                N12 => "ﾅｹ12",
                N13 => "ﾅｹ13",
                // ナリヤリ
                L14 => "ﾅﾔ14",
                L15 => "ﾅﾔ15",
                L16 => "ﾅﾔ16",
                L17 => "ﾅﾔ17",
                // ナリカク
                B18 => "ﾅｶ18",
                B19 => "ﾅｶ19",
                // ナリヒ
                R20 => "ﾅﾋ20",
                R21 => "ﾅﾋ21",
                // ナリフ
                P22 => "ﾅﾌ22",
                P23 => "ﾅﾌ23",
                P24 => "ﾅﾌ24",
                P25 => "ﾅﾌ25",
                P26 => "ﾅﾌ26",
                P27 => "ﾅﾌ27",
                P28 => "ﾅﾌ28",
                P29 => "ﾅﾌ29",
                P30 => "ﾅﾌ30",
                P31 => "ﾅﾌ31",
                P32 => "ﾅﾌ32",
                P33 => "ﾅﾌ33",
                P34 => "ﾅﾌ34",
                P35 => "ﾅﾌ35",
                P36 => "ﾅﾌ36",
                P37 => "ﾅﾌ37",
                P38 => "ﾅﾌ38",
                P39 => "ﾅﾌ39",
            }
            .to_string()
        } else {
            match self.get_id() {
                // オウ
                K00 => "ｵｳ00",
                K01 => "ｵｳ01",
                // キン
                G02 => "ｷﾝ02",
                G03 => "ｷﾝ03",
                G04 => "ｷﾝ04",
                G05 => "ｷﾝ05",
                // シルバー
                S06 => "ｼﾙ06",
                S07 => "ｼﾙ07",
                S08 => "ｼﾙ08",
                S09 => "ｼﾙ09",
                // ケイ
                N10 => "ｹｲ10",
                N11 => "ｹｲ11",
                N12 => "ｹｲ12",
                N13 => "ｹｲ13",
                // ヤリ
                L14 => "ﾔﾘ14",
                L15 => "ﾔﾘ15",
                L16 => "ﾔﾘ16",
                L17 => "ﾔﾘ17",
                // カク
                B18 => "ｶｸ18",
                B19 => "ｶｸ19",
                // ヒ
                R20 => "ヒ20",
                R21 => "ヒ21",
                // フ
                P22 => "フ22",
                P23 => "フ23",
                P24 => "フ24",
                P25 => "フ25",
                P26 => "フ26",
                P27 => "フ27",
                P28 => "フ28",
                P29 => "フ29",
                P30 => "フ30",
                P31 => "フ31",
                P32 => "フ32",
                P33 => "フ33",
                P34 => "フ34",
                P35 => "フ35",
                P36 => "フ36",
                P37 => "フ37",
                P38 => "フ38",
                P39 => "フ39",
            }
            .to_string()
        }
    }

    /// 成り玉とかあって、USI としては使えない。
    pub fn to_extended_usi_text(self) -> String {
        use instrument::piece_etc::Phase::*;
        use instrument::piece_etc::PieceIdentify::*;
        if let Some(phase) = self.get_phase() {
            match phase {
                First => {
                    if self.is_promoted() {
                        // 先手の成り駒。
                        match self.get_id() {
                            K00 | K01 => "+K",
                            R20 | R21 => "+R",
                            B18 | B19 => "+B",
                            G02 | G03 | G04 | G05 => "+G",
                            S06 | S07 | S08 | S09 => "+S",
                            N10 | N11 | N12 | N13 => "+N",
                            L14 | L15 | L16 | L17 => "+L",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "+P",
                        }
                    } else {
                        // 先手の不成駒。
                        match self.get_id() {
                            K00 | K01 => "K",
                            R20 | R21 => "R",
                            B18 | B19 => "B",
                            G02 | G03 | G04 | G05 => "G",
                            S06 | S07 | S08 | S09 => "S",
                            N10 | N11 | N12 | N13 => "N",
                            L14 | L15 | L16 | L17 => "L",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "P",
                        }
                    }
                }
                Second => {
                    if self.is_promoted() {
                        // 後手の成り駒。
                        match self.get_id() {
                            K00 | K01 => "+k",
                            R20 | R21 => "+r",
                            B18 | B19 => "+b",
                            G02 | G03 | G04 | G05 => "+g",
                            S06 | S07 | S08 | S09 => "+S",
                            N10 | N11 | N12 | N13 => "+n",
                            L14 | L15 | L16 | L17 => "+l",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "+p",
                        }
                    } else {
                        // 後手の不成駒。
                        match self.get_id() {
                            K00 | K01 => "k",
                            R20 | R21 => "r",
                            B18 | B19 => "b",
                            G02 | G03 | G04 | G05 => "g",
                            S06 | S07 | S08 | S09 => "s",
                            N10 | N11 | N12 | N13 => "n",
                            L14 | L15 | L16 | L17 => "l",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "p",
                        }
                    }
                }
            }
        } else {
            // 使っていない駒を USI符号 に変換しようとしてはいけないぜ☆（＾～＾）
            panic!("Unexpected physical sign.")
        }
        .to_string()
    }

    pub fn to_usi_sign(self) -> String {
        use instrument::piece_etc::Phase::*;
        use instrument::piece_etc::PieceIdentify::*;
        if let Some(phase) = self.get_phase() {
            match phase {
                First => {
                    if self.is_promoted() {
                        // 先手の成り駒。
                        match self.get_id() {
                            K00 | K01 => "K",
                            R20 | R21 => "+R",
                            B18 | B19 => "+B",
                            G02 | G03 | G04 | G05 => "G",
                            S06 | S07 | S08 | S09 => "+S",
                            N10 | N11 | N12 | N13 => "+N",
                            L14 | L15 | L16 | L17 => "+L",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "+P",
                        }
                    } else {
                        // 先手の不成駒。
                        match self.get_id() {
                            K00 | K01 => "K",
                            R20 | R21 => "R",
                            B18 | B19 => "B",
                            G02 | G03 | G04 | G05 => "G",
                            S06 | S07 | S08 | S09 => "S",
                            N10 | N11 | N12 | N13 => "N",
                            L14 | L15 | L16 | L17 => "L",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "P",
                        }
                    }
                }
                Second => {
                    if self.is_promoted() {
                        // 後手の成り駒。
                        match self.get_id() {
                            K00 | K01 => "k",
                            R20 | R21 => "+r",
                            B18 | B19 => "+b",
                            G02 | G03 | G04 | G05 => "g",
                            S06 | S07 | S08 | S09 => "+S",
                            N10 | N11 | N12 | N13 => "+n",
                            L14 | L15 | L16 | L17 => "+l",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "+p",
                        }
                    } else {
                        // 後手の不成駒。
                        match self.get_id() {
                            K00 | K01 => "k",
                            R20 | R21 => "r",
                            B18 | B19 => "b",
                            G02 | G03 | G04 | G05 => "g",
                            S06 | S07 | S08 | S09 => "s",
                            N10 | N11 | N12 | N13 => "n",
                            L14 | L15 | L16 | L17 => "l",
                            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32
                            | P33 | P34 | P35 | P36 | P37 | P38 | P39 => "p",
                        }
                    }
                }
            }
        } else {
            // 使っていない駒を USI符号 に変換しようとしてはいけないぜ☆（＾～＾）
            panic!("Unexpected usi sign.")
        }
        .to_string()
    }
}

/// Piece type with phase.
/// First phase is 1.
/// Second phase is 2.
/// None phase is 3.
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    // King is 玉.
    K1 = 0,
    K2,
    K3,
    // 生.
    PK1,
    PK2,
    PK3,

    // Rook is 飛.
    R1,
    R2,
    R3,
    // Promoted rook is 竜.
    PR1,
    PR2,
    PR3,

    // Bishop is 角.
    B1,
    B2,
    B3,
    // Promoted bishop is 馬.
    PB1,
    PB2,
    PB3,

    // Gold is 金.
    G1,
    G2,
    G3,
    // 生.
    PG1,
    PG2,
    PG3,

    // Silver is 銀.
    S1,
    S2,
    S3,
    // Promoted silver is 成銀.
    PS1,
    PS2,
    PS3,

    // kNight is 桂.
    N1,
    N2,
    N3,
    // Promoted knight is 成桂.
    PN1,
    PN2,
    PN3,

    // Lance is 香.
    L1,
    L2,
    L3,
    // Promoted lance is 成香.
    PL1,
    PL2,
    PL3,

    // Pawn is 歩.
    P1,
    P2,
    P3,
    // Promoted pawn is と.
    PP1,
    PP2,
    PP3,
}
impl Piece {
    pub fn from_ph_pid(phase_opt: Option<Phase>, pid: PieceIdentify) -> Self {
        Piece::from_ph_pt(phase_opt, pid.get_piece_type())
    }

    pub fn from_ph_pt(phase_opt: Option<Phase>, piece_type: PieceType) -> Self {
        use instrument::piece_etc::Phase::*;
        use instrument::piece_etc::Piece::*;
        use instrument::piece_etc::PieceType::*;
        match phase_opt {
            Some(phase) => match phase {
                First => match piece_type {
                    K => K1,
                    PK => PK1,

                    R => R1,
                    PR => PR1,

                    B => B1,
                    PB => PB1,

                    G => G1,
                    PG => PG1,

                    S => S1,
                    PS => PS1,

                    N => N1,
                    PN => PN1,

                    L => L1,
                    PL => PL1,

                    P => P1,
                    PP => PP1,
                },
                Second => match piece_type {
                    K => K2,
                    PK => PK2,

                    R => R2,
                    PR => PR2,

                    B => B2,
                    PB => PB2,

                    G => G2,
                    PG => PG2,

                    S => S2,
                    PS => PS2,

                    N => N2,
                    PN => PN2,

                    L => L2,
                    PL => PL2,

                    P => P2,
                    PP => PP2,
                },
            },
            None => match piece_type {
                K => K3,
                PK => PK3,

                R => R3,
                PR => PR3,

                B => B3,
                PB => PB3,

                G => G3,
                PG => PG3,

                S => S3,
                PS => PS3,

                N => N3,
                PN => PN3,

                L => L3,
                PL => PL3,

                P => P3,
                PP => PP3,
            },
        }
    }

    pub fn to_disactivate(self) -> Self {
        use instrument::piece_etc::Piece::*;
        match self {
            K1 | K2 | K3 => K3,
            PK1 | PK2 | PK3 => PK3,

            R1 | R2 | R3 => R3,
            PR1 | PR2 | PR3 => PR3,

            B1 | B2 | B3 => B3,
            PB1 | PB2 | PB3 => PB3,

            G1 | G2 | G3 => G3,
            PG1 | PG2 | PG3 => PG3,

            S1 | S2 | S3 => S3,
            PS1 | PS2 | PS3 => PS3,

            N1 | N2 | N3 => N3,
            PN1 | PN2 | PN3 => PN3,

            L1 | L2 | L3 => L3,
            PL1 | PL2 | PL3 => PL3,

            P1 | P2 | P3 => P3,
            PP1 | PP2 | PP3 => PP3,
        }
    }

    pub fn promote(self) -> Self {
        use instrument::piece_etc::Piece::*;
        match self {
            K1 => PK1,
            K2 => PK2,
            K3 => PK3,
            PK1 => K1,
            PK2 => K2,
            PK3 => K3,

            R1 => PR1,
            R2 => PR2,
            R3 => PR3,
            PR1 => R1,
            PR2 => R2,
            PR3 => R3,

            B1 => PB1,
            B2 => PB2,
            B3 => PB3,
            PB1 => B1,
            PB2 => B2,
            PB3 => B3,

            G1 => PG1,
            G2 => PG2,
            G3 => PG3,
            PG1 => G1,
            PG2 => G2,
            PG3 => G3,

            S1 => PS1,
            S2 => PS2,
            S3 => PS3,
            PS1 => S1,
            PS2 => S2,
            PS3 => S3,

            N1 => PN1,
            N2 => PN2,
            N3 => PN3,
            PN1 => N1,
            PN2 => N2,
            PN3 => N3,

            L1 => PL1,
            L2 => PL2,
            L3 => PL3,
            PL1 => L1,
            PL2 => L2,
            PL3 => L3,

            P1 => PP1,
            P2 => PP2,
            P3 => PP3,
            PP1 => P1,
            PP2 => P2,
            PP3 => P3,
        }
    }

    pub fn rotate(self) -> Self {
        use instrument::piece_etc::Piece::*;
        match self {
            // K
            K1 => K2,
            K2 => K1,
            K3 => K3,
            PK1 => PK2,
            PK2 => PK1,
            PK3 => PK3,

            // R
            R1 => R2,
            R2 => R1,
            R3 => R3,
            PR1 => PR2,
            PR2 => PR1,
            PR3 => PR3,

            B1 => B2,
            B2 => B1,
            B3 => B3,
            PB1 => PB2,
            PB2 => PB1,
            PB3 => PB3,

            G1 => PG2,
            G2 => PG1,
            G3 => PG3,
            PG1 => G2,
            PG2 => G1,
            PG3 => G3,

            S1 => S2,
            S2 => S1,
            S3 => S3,
            PS1 => PS2,
            PS2 => PS1,
            PS3 => PS3,

            N1 => N2,
            N2 => N1,
            N3 => N3,
            PN1 => PN2,
            PN2 => PN1,
            PN3 => PN3,

            L1 => L2,
            L2 => L1,
            L3 => L3,
            PL1 => PL2,
            PL2 => PL1,
            PL3 => PL3,

            P1 => P2,
            P2 => P1,
            P3 => P3,
            PP1 => PP2,
            PP2 => PP1,
            PP3 => PP3,
        }
    }

    pub fn get_phase(self) -> Option<Phase> {
        use instrument::piece_etc::Piece::*;
        match self {
            K1 | PK1 | R1 | PR1 | B1 | PB1 | G1 | PG1 | S1 | PS1 | N1 | PN1 | L1 | PL1 | P1
            | PP1 => Some(Phase::First),
            K2 | PK2 | R2 | PR2 | B2 | PB2 | G2 | PG2 | S2 | PS2 | N2 | PN2 | L2 | PL2 | P2
            | PP2 => Some(Phase::Second),
            K3 | PK3 | R3 | PR3 | B3 | PB3 | G3 | PG3 | S3 | PS3 | N3 | PN3 | L3 | PL3 | P3
            | PP3 => None,
        }
    }

    pub fn get_type(self) -> PieceType {
        use instrument::piece_etc::Piece::*;
        use instrument::piece_etc::PieceType::*;
        match self {
            K1 | K2 | K3 => K,
            PK1 | PK2 | PK3 => PK,
            R1 | R2 | R3 => R,
            PR1 | PR2 | PR3 => PR,
            B1 | B2 | B3 => B,
            PB1 | PB2 | PB3 => PB,
            G1 | G2 | G3 => G,
            PG1 | PG2 | PG3 => PG,
            S1 | S2 | S3 => S,
            PS1 | PS2 | PS3 => PS,
            N1 | N2 | N3 => N,
            PN1 | PN2 | PN3 => PN,
            L1 | L2 | L3 => L,
            PL1 | PL2 | PL3 => PL,
            P1 | P2 | P3 => P,
            PP1 | PP2 | PP3 => PP,
        }
    }

    // Human presentable.
    pub fn to_human_presentable(self) -> String {
        use instrument::piece_etc::Piece::*;
        match self {
            K1 => "▼玉",
            K2 => "△玉",
            K3 => "□玉",
            PK1 => "▼生",
            PK2 => "△生",
            PK3 => "□生",

            R1 => "▼飛",
            R2 => "△飛",
            R3 => "□飛",
            PR1 => "▼竜",
            PR2 => "△竜",
            PR3 => "□竜",

            B1 => "▼角",
            B2 => "△角",
            B3 => "□角",
            PB1 => "▼馬",
            PB2 => "△馬",
            PB3 => "□馬",

            G1 => "▼金",
            G2 => "△金",
            G3 => "□金",
            PG1 => "▼今",
            PG2 => "△今",
            PG3 => "□今",

            S1 => "▼銀",
            S2 => "△銀",
            S3 => "□銀",
            PS1 => "▼全",
            PS2 => "△全",
            PS3 => "□全",

            N1 => "▼桂",
            N2 => "△桂",
            N3 => "□桂",
            PN1 => "▼圭",
            PN2 => "△圭",
            PN3 => "□圭",

            L1 => "▼香",
            L2 => "△香",
            L3 => "□香",
            PL1 => "▼杏",
            PL2 => "△杏",
            PL3 => "□杏",

            P1 => "▼歩",
            P2 => "△歩",
            P3 => "□歩",
            PP1 => "▼と",
            PP2 => "△と",
            PP3 => "□と",
        }
        .to_string()
    }

    // Computer readable.
    pub fn to_sign(self) -> String {
        use instrument::piece_etc::Piece::*;
        match self {
            K1 => "K",
            K2 => "k",
            K3 => "K",
            PK1 => "+K",
            PK2 => "+k",
            PK3 => "+K",

            R1 => "R",
            R2 => "r",
            R3 => "R",
            PR1 => "+R",
            PR2 => "+r",
            PR3 => "+R",

            B1 => "B",
            B2 => "b",
            B3 => "B",
            PB1 => "+B",
            PB2 => "+b",
            PB3 => "+B",

            G1 => "G",
            G2 => "g",
            G3 => "G",
            PG1 => "+G",
            PG2 => "+g",
            PG3 => "+G",

            S1 => "S",
            S2 => "s",
            S3 => "S",
            PS1 => "+S",
            PS2 => "+s",
            PS3 => "+S",

            N1 => "N",
            N2 => "n",
            N3 => "N",
            PN1 => "+N",
            PN2 => "+n",
            PN3 => "+N",

            L1 => "L",
            L2 => "l",
            L3 => "L",
            PL1 => "+L",
            PL2 => "+l",
            PL3 => "+L",

            P1 => "P",
            P2 => "p",
            P3 => "P",
            PP1 => "+P",
            PP2 => "+p",
            PP3 => "+P",
        }
        .to_string()
    }
}

/*
pub fn piece_to_sign(piece_opt: Option<Piece>) -> String {
    if let Some(piece) = piece_opt {
        piece.to_sign()
    } else {
        "".to_string()
    }
}
*/

#[derive(Clone, Copy, PartialEq)]
pub enum JsaPieceType {
    // King is 玉.
    K = 0,
    // Rook is 飛.
    R,
    // Promoted rook is 竜.
    PR,
    // Bishop is 角.
    B,
    // Promoted bishop is 馬.
    PB,
    // Gold is 金.
    G,
    // Silver is 銀.
    S,
    // Promoted silver is 成銀.
    PS,
    // kNight is 桂.
    N,
    // Promoted knight is 成桂.
    PN,
    // Lance is 香.
    L,
    // Promoted lance is 成香.
    PL,
    // Pawn is 歩.
    P,
    // Promoted pawn is と.
    PP,
}
/// Perfect piece type.
#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    // King is 玉.
    K = 0,
    // 生
    PK,

    // Rook is 飛.
    R,
    // Promoted rook is 竜.
    PR,

    // Bishop is 角.
    B,
    // Promoted bishop is 馬.
    PB,

    // Gold is 金.
    G,
    // 今
    PG,

    // Silver is 銀.
    S,
    // Promoted silver is 成銀.
    PS,

    // kNight is 桂.
    N,
    // Promoted knight is 成桂.
    PN,

    // Lance is 香.
    L,
    // Promoted lance is 成香.
    PL,

    // Pawn is 歩.
    P,
    // Promoted pawn is と.
    PP,
}
impl PieceType {
    pub fn from_piece(piece: Piece) -> PieceType {
        use instrument::piece_etc::Piece::*;
        use instrument::piece_etc::PieceType::*;
        match piece {
            K1 => K,
            K2 => K,
            K3 => K,
            PK1 => PK,
            PK2 => PK,
            PK3 => PK,

            R1 => R,
            R2 => R,
            R3 => R,
            PR1 => PR,
            PR2 => PR,
            PR3 => PR,

            B1 => B,
            B2 => B,
            B3 => B,
            PB1 => PB,
            PB2 => PB,
            PB3 => PB,

            G1 => G,
            G2 => G,
            G3 => G,
            PG1 => PG,
            PG2 => PG,
            PG3 => PG,

            S1 => S,
            S2 => S,
            S3 => S,
            PS1 => PS,
            PS2 => PS,
            PS3 => PS,

            N1 => N,
            N2 => N,
            N3 => N,
            PN1 => PN,
            PN2 => PN,
            PN3 => PN,

            L1 => L,
            L2 => L,
            L3 => L,
            PL1 => PL,
            PL2 => PL,
            PL3 => PL,

            P1 => P,
            P2 => P,
            P3 => P,
            PP1 => PP,
            PP2 => PP,
            PP3 => PP,
        }
    }

    pub fn from_jsa_piece_type(jsa_pt: JsaPieceType) -> PieceType {
        use instrument::piece_etc::JsaPieceType;
        use instrument::piece_etc::PieceType;
        match jsa_pt {
            JsaPieceType::K => PieceType::K,

            JsaPieceType::R => PieceType::R,
            JsaPieceType::PR => PieceType::PR,

            JsaPieceType::B => PieceType::B,
            JsaPieceType::PB => PieceType::PB,

            JsaPieceType::G => PieceType::G,

            JsaPieceType::S => PieceType::S,
            JsaPieceType::PS => PieceType::PS,

            JsaPieceType::N => PieceType::N,
            JsaPieceType::PN => PieceType::PN,

            JsaPieceType::L => PieceType::L,
            JsaPieceType::PL => PieceType::PL,

            JsaPieceType::P => PieceType::P,
            JsaPieceType::PP => PieceType::PP,
        }
    }

    pub fn to_sign(self) -> String {
        use instrument::piece_etc::PieceType::*;
        match self {
            K => "K",
            PK => "K",

            R => "R",
            PR => "+R",

            B => "B",
            PB => "+B",

            G => "G",
            PG => "G",

            S => "S",
            PS => "+S",

            N => "N",
            PN => "+N",

            L => "L",
            PL => "+L",

            P => "P",
            PP => "+P",
        }
        .to_string()
    }
}

pub fn hand_id_piece_to_hand_index(id_piece: IdentifiedPiece) -> usize {
    use instrument::piece_etc::Phase::*;
    use instrument::piece_etc::PieceIdentify::*;
    if let Some(phase) = id_piece.phase {
        match phase {
            First => match id_piece.get_id() {
                K00 | K01 => 0,
                R20 | R21 => 1,
                B18 | B19 => 2,
                G02 | G03 | G04 | G05 => 3,
                S06 | S07 | S08 | S09 => 4,
                N10 | N11 | N12 | N13 => 5,
                L14 | L15 | L16 | L17 => 6,
                P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32 | P33 | P34
                | P35 | P36 | P37 | P38 | P39 => 7,
            },
            Second => match id_piece.get_id() {
                K00 | K01 => 8,
                R20 | R21 => 9,
                B18 | B19 => 10,
                G02 | G03 | G04 | G05 => 11,
                S06 | S07 | S08 | S09 => 12,
                N10 | N11 | N12 | N13 => 13,
                L14 | L15 | L16 | L17 => 14,
                P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32 | P33 | P34
                | P35 | P36 | P37 | P38 | P39 => 15,
            },
        }
    } else {
        match id_piece.get_id() {
            K00 | K01 => 16,
            R20 | R21 => 17,
            B18 | B19 => 18,
            G02 | G03 | G04 | G05 => 19,
            S06 | S07 | S08 | S09 => 20,
            N10 | N11 | N12 | N13 => 21,
            L14 | L15 | L16 | L17 => 22,
            P22 | P23 | P24 | P25 | P26 | P27 | P28 | P29 | P30 | P31 | P32 | P33 | P34 | P35
            | P36 | P37 | P38 | P39 => 23,
        }
    }
}

#[derive(Copy, Clone)]
pub enum HandIndex {
    HndK1,
    HndR1,
    HndB1,
    HndG1,
    HndS1,
    HndN1,
    HndL1,
    HndP1,

    HndK2,
    HndR2,
    HndB2,
    HndG2,
    HndS2,
    HndN2,
    HndL2,
    HndP2,

    HndK3,
    HndR3,
    HndB3,
    HndG3,
    HndS3,
    HndN3,
    HndL3,
    HndP3,
}
impl HandIndex {
    pub fn from_piece(piece: Piece) -> HandIndex {
        use instrument::piece_etc::HandIndex::*;
        use instrument::piece_etc::Piece::*;
        match piece {
            K1 | PK1 => HndK1,
            K2 | PK2 => HndK2,
            K3 | PK3 => HndK3,

            R1 | PR1 => HndR1,
            R2 | PR2 => HndR2,
            R3 | PR3 => HndR3,

            B1 | PB1 => HndB1,
            B2 | PB2 => HndB2,
            B3 | PB3 => HndB3,

            G1 | PG1 => HndG1,
            G2 | PG2 => HndG2,
            G3 | PG3 => HndG3,

            S1 | PS1 => HndS1,
            S2 | PS2 => HndS2,
            S3 | PS3 => HndS3,

            N1 | PN1 => HndN1,
            N2 | PN2 => HndN2,
            N3 | PN3 => HndN3,

            L1 | PL1 => HndL1,
            L2 | PL2 => HndL2,
            L3 | PL3 => HndL3,

            P1 | PP1 => HndP1,
            P2 | PP2 => HndP2,
            P3 | PP3 => HndP3,
        }
    }

    pub fn get_index(self) -> usize {
        self as usize
    }
}

pub struct PhysicalSign {
    text: String,
}
impl PhysicalSign {
    pub fn default(sign_text: String) -> PhysicalSign {
        PhysicalSign { text: sign_text }
    }

    pub fn to_piece(&self) -> Piece {
        use instrument::piece_etc::Piece::*;
        match self.text.as_str() {
            // 成りごまの場合は頭に半角のＰ、ｐ、ナを付ける。
            // Ｋｉｎｇ，オウ．
            "K" => K1,
            "k" => K2,
            "ｵ" => K3,
            "PK" => PK1,
            "pk" => PK2,
            "ﾅｵ" => PK3,

            // Ｒｏｏｋ，ヒシャ．
            "R" => R1,
            "r" => R2,
            "ﾋ" => R3,
            "PR" => PR1,
            "pr" => PR2,
            "ﾅﾋ" => R3,

            // Ｂｉｓｈｏｐ，カク．
            "B" => B1,
            "b" => B2,
            "ｶ" => B3,
            "PB" => PB1,
            "pb" => PB2,
            "ﾅｶ" => PB3,

            // Ｇｏｌｄ，キン．
            "G" => G1,
            "g" => G2,
            "ｷ" => G3,
            "PG" => PG1,
            "pg" => PG2,
            "ﾅｷ" => PG3,

            // Ｓｉｌｖｅｒ，シルバー．
            "S" => S1,
            "s" => S2,
            "ｼ" => S3,
            "PS" => PS1,
            "ps" => PS2,
            "ﾅｼ" => PS3,

            // Ｋｎｉｇｈｔ，ケイ．
            "N" => N1,
            "n" => N2,
            "ｹ" => N3,
            "PN" => PN1,
            "pn" => PN2,
            "ﾅｹ" => PN3,

            // Ｌａｎｃｅ，ヤリ．
            "L" => L1,
            "l" => L2,
            "ﾔ" => L3,
            "PL" => PL1,
            "pl" => PL2,
            "ﾅﾔ" => PL3,

            // Ｐａｗｎ，フ．
            "P" => P1,
            "p" => P2,
            "ﾌ" => P3,
            "PP" => PP1,
            "pp" => PP2,
            "ﾅﾌ" => PP3,
            _ => panic!("Unexpected physical sign: '{}'.", self.text),
        }
    }

    pub fn to_piece_type(&self) -> PieceType {
        use instrument::piece_etc::PieceType::*;
        match self.text.as_str() {
            "K" | "k" => K,
            "R" | "r" => R,
            "B" | "b" => B,
            "G" | "g" => G,
            "S" | "s" => S,
            "N" | "n" => N,
            "L" | "l" => L,
            "P" | "p" => P,
            "PR" | "pr" => PR,
            "PB" | "pb" => PB,
            "PS" | "ps" => PS,
            "PN" | "pn" => PN,
            "PL" | "pl" => PL,
            "PP" | "pp" => PP,
            _ => panic!("Unexpected sign: '{}'.", self.text),
        }
    }
}

/*
pub fn promotion_piece(piece_opt: Option<Piece>) -> Option<Piece> {
    if let Some(piece) = piece_opt {
        Some(piece.promote())
    } else {
        None
    }
}

pub fn rotate_piece(piece_opt: Option<Piece>) -> Option<Piece> {
    if let Some(piece) = piece_opt {
        Some(piece.rotate())
    } else {
        None
    }
}
*/
pub fn is_promoted_piece(piece_opt: Option<Piece>) -> bool {
    if let Some(piece) = piece_opt {
        use instrument::piece_etc::Piece::*;
        match piece {
            PK1 | PR1 | PB1 | PG1 | PS1 | PN1 | PL1 | PP1 | PK2 | PR2 | PB2 | PG2 | PS2 | PN2
            | PL2 | PP2 | PK3 | PR3 | PB3 | PG3 | PS3 | PN3 | PL3 | PP3 => true,
            _ => false,
        }
    } else {
        false
    }
}
pub fn is_promoted_piece_type(piece_type_opt: Option<PieceType>) -> bool {
    if let Some(piece_type) = piece_type_opt {
        use instrument::piece_etc::PieceType::*;
        match piece_type {
            PK | PR | PB | PG | PS | PN | PL | PP => true,
            _ => false,
        }
    } else {
        false
    }
}

pub fn jsa_piece_type_to_perfect(jsa_pt_opt: Option<JsaPieceType>) -> Option<PieceType> {
    match jsa_pt_opt {
        Some(jsa_pt) => Some(PieceType::from_jsa_piece_type(jsa_pt)),
        None => None,
    }
}
pub fn jsa_piece_type_to_sign(piece_type_opt: Option<JsaPieceType>) -> String {
    use instrument::piece_etc::JsaPieceType::*;
    match piece_type_opt {
        Some(piece_type) => match piece_type {
            K => "K",

            R => "R",
            PR => "+R",

            B => "B",
            PB => "+B",

            G => "G",

            S => "S",
            PS => "+S",

            N => "N",
            PN => "+N",

            L => "L",
            PL => "+L",

            P => "P",
            PP => "+P",
        },
        None => "",
    }
    .to_string()
}

pub fn parse_sign_to_drop(line: &str, start: &mut usize) -> Option<PieceType> {
    use instrument::piece_etc::PieceType::*;

    if line.len() < *start + 2 {
        return None;
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    let piece_type = match sign {
        'R' => R,
        'B' => B,
        'G' => G,
        'S' => S,
        'N' => N,
        'L' => L,
        'P' => P,
        _ => {
            return None;
        }
    };

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    if sign == '*' {
        *start += 2;
        Some(piece_type)
    } else {
        panic!("Failed: Sfen unexpected drop.");
    }
}

pub fn parse_sign_to_rank(line: &str, start: &mut usize) -> i8 {
    if line.len() < *start + 1 {
        panic!(
            "Failed: Unexpected file. len: {}, start: {}.",
            line.len(),
            start
        );
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    *start += 1;
    match sign {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        _ => panic!(
            "Failed: Unexpected rank. line `{}` at {}, `{}`.",
            line,
            *start - 1,
            sign
        ),
    }
}

pub fn rank_to_sign(sign: i8) -> char {
    match sign {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        9 => 'i',
        _ => panic!("Failed: Unexpected rank number `{}`.", sign),
    }
}

pub fn parse_sign_to_file(line: &str, start: &mut usize) -> i8 {
    if line.len() < *start as usize + 1 {
        panic!("Failed: Nothing file.");
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    *start += 1;
    match sign {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!(
            "Unexpected file: `{}`. line `{}` at {}.",
            sign,
            line,
            *start - 1
        ),
    }
}

pub fn parse_sign_to_promotion(line: &str, start: &mut usize) -> bool {
    if line.len() < *start as usize + 1 {
        return false;
    }

    let v: Vec<char> = line.to_string().chars().collect();
    let sign = v[*start];
    if sign == '+' {
        *start += 1;
        true
    } else {
        false
    }
}
