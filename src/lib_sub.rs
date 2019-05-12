use audio_compo::cassette_deck::*;
use instrument::half_player_phase::*;
use instrument::piece_etc::*;
use instrument::position::*;
use media::cassette_tape::*;
use media::two_heads_vec::*;
use sheet_music_format::kifu_rpm::rpm_tape_box::*;
use sheet_music_format::kifu_usi::fen::*;
use sheet_music_format::kifu_usi::usi_converter::*;
use sheet_music_format::kifu_usi::usi_position::*;
use sound::shogi_note::*;
use sound::shogi_note_operation::*;
use studio::address::*;
use studio::application::*;
use studio::board_size::*;
use studio::common::caret::*;

pub struct LibSub {}
impl LibSub {
    pub fn gameover(deck: &mut CassetteDeck, board_size: BoardSize, app: &Application) {
        // TODO とりあえず、テープが１個入った　テープ・ボックス形式で書きだし☆（＾～＾）
        deck.write_tape_box(board_size, &app);
    }

    // #####
    // # H #
    // #####

    pub fn hand1(position: &Position, app: &Application) {
        // TODO 先手の持ち駒を表示。
        let (line0, line1, line2, line3) = position.to_hand_4lines(HalfPlayerPhaseValue::First);
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }
    pub fn hand2(position: &Position, app: &Application) {
        // TODO 後手の持ち駒を表示。
        let (line0, line1, line2, line3) = position.to_hand_4lines(HalfPlayerPhaseValue::Second);
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }
    pub fn hand3(position: &Position, app: &Application) {
        // TODO 使っていない駒を表示。
        let (line0, line1, line2, line3) =
            position.to_hand_4lines(HalfPlayerPhaseValue::ZeroPointFive); // TODO とりあえず 0.5 で。
        app.comm.println(&line0);
        app.comm.println(&line1);
        app.comm.println(&line2);
        app.comm.println(&line3);
    }

    // #####
    // # L #
    // #####

    pub fn look_back(deck: &mut CassetteDeck, slot: Slot, app: &Application) {
        deck.look_back_caret(slot, &app)
    }

    // #####
    // # P #
    // #####

    pub fn position(
        line: String,
        deck: &mut CassetteDeck,
        position: &mut Position,
        app: &Application,
    ) {
        // 相手が指したあとの局面まで進める。
        let mut urecord_opt = None;
        let mut start = 0;

        // 指定局面にリセットするぜ☆（＾～＾）
        if Fen::parse_initial_position(&line, &mut start, position, deck, &app) {
            // USI の moves の文字列を、オブジェクトに直訳するぜ☆（＾～＾）局面は指定局面から動かさないぜ☆（＾～＾）
            urecord_opt = UsiPosition::parse_usi_line_moves(
                &line,
                &mut start,
                position.get_board_size(),
                &app,
            );
        }

        // USI -> RPM 変換を作れていないので、ポジションをもう１回初期局面に戻してから、プレイアウトします。
        // TODO できれば USI -> RPM 変換したい。
        if let Some(urecord) = urecord_opt {
            // 差し替え。
            deck.clear_of_tapes(Slot::Training, &app);
            UsiConverter::play_out_usi_tape(position, &urecord, deck, &app);
        }
    }

    // #####
    // # T #
    // #####

    pub fn test_2heads_vec(board_size: BoardSize, app: &Application) {
        let tvec = TwoHeadsVec::new();

        let mut caret = Caret::new_facing_right_caret();
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 最初:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app),
            ))
        }

        // １つ目
        let tvec = tvec.new_vector_with_inserted_note(
            &mut caret,
            ShogiNote::from_id_ope(None, ShogiNoteOpe::change_phase(0), false),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: １つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }

        // ２つ目
        let tvec = tvec.new_vector_with_inserted_note(
            &mut caret,
            ShogiNote::from_id_ope(
                Some(PieceIdentify::K00),
                ShogiNoteOpe::from_address(Address::from_cell(
                    Cell::from_file_rank(5, 1),
                    board_size,
                )),
                false,
            ),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 2つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }

        // ３つ目
        let tvec = tvec.new_vector_with_inserted_note(
            &mut caret,
            ShogiNote::from_id_ope(
                Some(PieceIdentify::K00),
                ShogiNoteOpe::from_address(Address::from_cell(
                    Cell::from_file_rank(6, 2),
                    board_size,
                )),
                false,
            ),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 3つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }

        // ４つ目
        let tvec = tvec.new_vector_with_inserted_note(
            &mut caret,
            ShogiNote::from_id_ope(None, ShogiNoteOpe::change_phase(0), false),
            board_size,
            &app,
        );
        caret.seek_a_note(&app);
        if app.is_debug() {
            app.comm.println(&format!(
                "[#Test: 4つ目:{}, {}]",
                caret.to_human_presentable(&app),
                tvec.to_human_presentable(board_size, &app)
            ))
        }
    }

    // #####
    // # U #
    // #####

    pub fn usi_new_game(deck: &mut CassetteDeck, app: &Application) {
        // 今対局分のラーニング・テープを１つ追加するぜ☆（＾～＾）

        // ラーニング・テープ作成。
        let mut tape = CassetteTape::new_facing_right(&app);
        tape.set_file_full_name_without_extension(
            &RpmTapeBox::create_file_full_name_without_extension(&app.kw29_conf, &app),
        );
        deck.add_tape_to_tape_box(Slot::Learning, tape, &app);
        deck.seek_of_next_tape(Slot::Learning, &app);
    }
}
