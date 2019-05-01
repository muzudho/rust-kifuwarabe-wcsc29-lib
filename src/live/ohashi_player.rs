use audio_compo::cassette_deck::CassetteDeck;
use audio_compo::cassette_deck::Slot;
use instrument::piece_etc::*;
use instrument::position::*;
use sound::shogi_note_operation::*;
use std::*;
use studio::address::*;
use studio::application::Application;
use studio::board_size::*;

/// 大橋流だけ指してくれるプレイヤー。
pub struct OhashiPlayer {}
impl OhashiPlayer {
    /// 大橋流を指せるように、クリアーするぜ☆（＾～＾）
    pub fn clear_to_honshogi_origin(
        position: &mut Position,
        deck: &mut CassetteDeck,
        app: &Application,
    ) {
        // オリジン局面に戻す☆（＾～＾）
        deck.change(None, position.get_board_size(), &app);
        position.reset_origin_position();
    }

    /// 初期化に使う。
    fn init_note(
        ply: i16,
        ph: Phase,
        file: i8,
        rank: i8,
        pid: PieceIdentify,
        bs: BoardSize,
    ) -> (ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe) {
        (
            ShogiNoteOpe::from_address(Address::from_hand_pi(Piece::from_ph_pid(Some(ph), pid))),
            ShogiNoteOpe::from_address(Address::from_cell(Cell::from_file_rank(file, rank), bs)),
            ShogiNoteOpe::change_phase(ply),
        )
    }

    /// オリジン・ポジションから、平手初期局面に進めます。
    /// 盤上の局面だけではなく、トレーニング・テープ、ラーニング・テープの両方のキャレットも同期して進めます。
    pub fn play_ohashi_starting(pos: &mut Position, deck: &mut CassetteDeck, app: &Application) {
        use instrument::piece_etc::Phase::*;
        use instrument::piece_etc::PieceIdentify::*;

        // 大橋流の順序にしてください。
        // しかし きふわらべ は駒台から逆順に駒を取っていくので（スタック構造のポップ）、
        // 局面作成の時点で、駒台の駒は　背番号の逆順に追加しておいてください。
        let bs = pos.get_board_size();
        let array: [(ShogiNoteOpe, ShogiNoteOpe, ShogiNoteOpe); 40] = [
            OhashiPlayer::init_note(-39, Second, 5, 1, K00, bs),
            OhashiPlayer::init_note(-38, First, 5, 9, K01, bs),
            OhashiPlayer::init_note(-37, Second, 4, 1, G02, bs),
            OhashiPlayer::init_note(-36, First, 6, 9, G03, bs),
            OhashiPlayer::init_note(-35, Second, 6, 1, G04, bs),
            OhashiPlayer::init_note(-34, First, 4, 9, G05, bs),
            OhashiPlayer::init_note(-33, Second, 3, 1, S06, bs),
            OhashiPlayer::init_note(-32, First, 7, 9, S07, bs),
            OhashiPlayer::init_note(-31, Second, 7, 1, S08, bs),
            OhashiPlayer::init_note(-30, First, 3, 9, S09, bs),
            OhashiPlayer::init_note(-29, Second, 2, 1, N10, bs),
            OhashiPlayer::init_note(-28, First, 8, 9, N11, bs),
            OhashiPlayer::init_note(-27, Second, 8, 1, N12, bs),
            OhashiPlayer::init_note(-26, First, 2, 9, N13, bs),
            OhashiPlayer::init_note(-25, Second, 1, 1, L14, bs),
            OhashiPlayer::init_note(-24, First, 9, 9, L15, bs),
            OhashiPlayer::init_note(-23, Second, 9, 1, L16, bs),
            OhashiPlayer::init_note(-22, First, 1, 9, L17, bs),
            OhashiPlayer::init_note(-21, Second, 2, 2, B18, bs),
            OhashiPlayer::init_note(-20, First, 8, 8, B19, bs),
            OhashiPlayer::init_note(-19, Second, 8, 2, R20, bs),
            OhashiPlayer::init_note(-18, First, 2, 8, R21, bs),
            OhashiPlayer::init_note(-17, Second, 5, 3, P22, bs),
            OhashiPlayer::init_note(-16, First, 5, 7, P23, bs),
            OhashiPlayer::init_note(-15, Second, 4, 3, P24, bs),
            OhashiPlayer::init_note(-14, First, 6, 7, P25, bs),
            OhashiPlayer::init_note(-13, Second, 6, 3, P26, bs),
            OhashiPlayer::init_note(-12, First, 4, 7, P27, bs),
            OhashiPlayer::init_note(-11, Second, 3, 3, P28, bs),
            OhashiPlayer::init_note(-10, First, 7, 7, P29, bs),
            OhashiPlayer::init_note(-9, Second, 7, 3, P30, bs),
            OhashiPlayer::init_note(-8, First, 3, 7, P31, bs),
            OhashiPlayer::init_note(-7, Second, 2, 3, P32, bs),
            OhashiPlayer::init_note(-6, First, 8, 7, P33, bs),
            OhashiPlayer::init_note(-5, Second, 8, 3, P34, bs),
            OhashiPlayer::init_note(-4, First, 2, 7, P35, bs),
            OhashiPlayer::init_note(-3, Second, 1, 3, P36, bs),
            OhashiPlayer::init_note(-2, First, 9, 7, P37, bs),
            OhashiPlayer::init_note(-1, Second, 9, 3, P38, bs),
            OhashiPlayer::init_note(0, First, 1, 7, P39, bs),
        ];

        for element in array.iter() {
            // 大橋流で指しているところはログを省略☆（＾～＾）１手に３ノート使う☆（＾～＾）

            // キャレットを動かして、盤をタッチする、というのを繰り返せだぜ☆（＾～＾）
            {
                if let Some(ref mut tape_box) = &mut deck.slots[Slot::Training as usize].tape_box {
                    tape_box.seek_to_next(&app);
                }
                if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
                    tape_box.seek_to_next(&app);
                }
                pos.touch_1note_ope_no_log(&element.0, deck, &app);
            }

            {
                if let Some(ref mut tape_box) = &mut deck.slots[Slot::Training as usize].tape_box {
                    tape_box.seek_to_next(&app);
                }
                if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
                    tape_box.seek_to_next(&app);
                }
                pos.touch_1note_ope_no_log(&element.1, deck, &app);
            }

            {
                if let Some(ref mut tape_box) = &mut deck.slots[Slot::Training as usize].tape_box {
                    tape_box.seek_to_next(&app);
                }
                if let Some(ref mut tape_box) = &mut deck.slots[Slot::Learning as usize].tape_box {
                    tape_box.seek_to_next(&app);
                }
                pos.touch_1note_ope_no_log(&element.2, deck, &app);
            }
        }
    }
}
