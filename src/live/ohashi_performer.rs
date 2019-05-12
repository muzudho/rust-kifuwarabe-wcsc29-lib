use audio_compo::cassette_deck::CassetteDeck;
use instrument::half_player_phase::*;
use instrument::piece_etc::*;
use instrument::position::*;
use sound::shogi_note_operation::*;
use std::*;
use studio::address::*;
use studio::application::Application;
use studio::board_size::*;

/// 大橋流のノート数は 120。1手3ノートの40駒。
pub const OHASHI_NOTE_LEN: usize = 3 * 40;

/// 大橋流だけ指してくれるプレイヤー。
pub struct OhashiPerformer {}
impl OhashiPerformer {
    /// 初期化に使う。
    fn init_note(
        ply: i16,
        phase_value: HalfPlayerPhaseValue,
        file: i8,
        rank: i8,
        pid: PieceIdentify,
        bs: BoardSize,
    ) -> (ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe) {
        (
            ShogiNoteOpe::change_phase(ply),
            ShogiNoteOpe::from_address(Address::from_hand_pi(Piece::from_ph_pid(phase_value, pid))),
            ShogiNoteOpe::from_address(Address::from_cell(Cell::from_file_rank(file, rank), bs)),
            ShogiNoteOpe::change_phase(ply),
        )
    }

    /// オリジン・ポジションになっている前提です。
    /// 平手初期局面に進めます。
    /// 盤上の局面だけではなく、トレーニング・テープ、ラーニング・テープの両方のキャレットも同期して進めます。
    pub fn improvise_ohashi_starting(
        deck: &mut CassetteDeck,
        position: &mut Position,
        app: &Application,
    ) {
        use instrument::half_player_phase::HalfPlayerPhaseValue::*;
        use instrument::piece_etc::PieceIdentify::*;

        // 大橋流の順序にしてください。
        // しかし きふわらべ は駒台から逆順に駒を取っていくので（スタック構造のポップ）、
        // 局面作成の時点で、駒台の駒は　背番号の逆順に追加しておいてください。
        let bs = position.get_board_size();
        let array: [(ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe); 40] = [
            OhashiPerformer::init_note(-39, Second, 5, 1, K00, bs),
            OhashiPerformer::init_note(-38, First, 5, 9, K01, bs),
            OhashiPerformer::init_note(-37, Second, 4, 1, G02, bs),
            OhashiPerformer::init_note(-36, First, 6, 9, G03, bs),
            OhashiPerformer::init_note(-35, Second, 6, 1, G04, bs),
            OhashiPerformer::init_note(-34, First, 4, 9, G05, bs),
            OhashiPerformer::init_note(-33, Second, 3, 1, S06, bs),
            OhashiPerformer::init_note(-32, First, 7, 9, S07, bs),
            OhashiPerformer::init_note(-31, Second, 7, 1, S08, bs),
            OhashiPerformer::init_note(-30, First, 3, 9, S09, bs),
            OhashiPerformer::init_note(-29, Second, 2, 1, N10, bs),
            OhashiPerformer::init_note(-28, First, 8, 9, N11, bs),
            OhashiPerformer::init_note(-27, Second, 8, 1, N12, bs),
            OhashiPerformer::init_note(-26, First, 2, 9, N13, bs),
            OhashiPerformer::init_note(-25, Second, 1, 1, L14, bs),
            OhashiPerformer::init_note(-24, First, 9, 9, L15, bs),
            OhashiPerformer::init_note(-23, Second, 9, 1, L16, bs),
            OhashiPerformer::init_note(-22, First, 1, 9, L17, bs),
            OhashiPerformer::init_note(-21, Second, 2, 2, B18, bs),
            OhashiPerformer::init_note(-20, First, 8, 8, B19, bs),
            OhashiPerformer::init_note(-19, Second, 8, 2, R20, bs),
            OhashiPerformer::init_note(-18, First, 2, 8, R21, bs),
            OhashiPerformer::init_note(-17, Second, 5, 3, P22, bs),
            OhashiPerformer::init_note(-16, First, 5, 7, P23, bs),
            OhashiPerformer::init_note(-15, Second, 4, 3, P24, bs),
            OhashiPerformer::init_note(-14, First, 6, 7, P25, bs),
            OhashiPerformer::init_note(-13, Second, 6, 3, P26, bs),
            OhashiPerformer::init_note(-12, First, 4, 7, P27, bs),
            OhashiPerformer::init_note(-11, Second, 3, 3, P28, bs),
            OhashiPerformer::init_note(-10, First, 7, 7, P29, bs),
            OhashiPerformer::init_note(-9, Second, 7, 3, P30, bs),
            OhashiPerformer::init_note(-8, First, 3, 7, P31, bs),
            OhashiPerformer::init_note(-7, Second, 2, 3, P32, bs),
            OhashiPerformer::init_note(-6, First, 8, 7, P33, bs),
            OhashiPerformer::init_note(-5, Second, 8, 3, P34, bs),
            OhashiPerformer::init_note(-4, First, 2, 7, P35, bs),
            OhashiPerformer::init_note(-3, Second, 1, 3, P36, bs),
            OhashiPerformer::init_note(-2, First, 9, 7, P37, bs),
            OhashiPerformer::init_note(-1, Second, 9, 3, P38, bs),
            OhashiPerformer::init_note(0, First, 1, 7, P39, bs),
        ];

        for element in array.iter() {
            // 大橋流で指している☆（＾～＾）１手に４ノート使う☆（＾～＾）
            // キャレットを動かして、盤をタッチする、というのを繰り返せだぜ☆（＾～＾）
            {
                // タッチすれば、ラーニング・テープに１ノート挿入される。勝手にシークしてくれる☆（＾～＾）
                position.touch_1note_ope_no_log(deck, &element.0, false, bs, &app);
                //HumanInterface::bo(deck, position, &app);
            }

            {
                position.touch_1note_ope_no_log(deck, &element.1, false, bs, &app);
                //HumanInterface::bo(deck, position, &app);
            }

            {
                position.touch_1note_ope_no_log(deck, &element.2, false, bs, &app);
                //HumanInterface::bo(deck, position, &app);
            }

            {
                position.touch_1note_ope_no_log(deck, &element.3, false, bs, &app);
                //HumanInterface::bo(deck, position, &app);
            }
        }
    }
}
