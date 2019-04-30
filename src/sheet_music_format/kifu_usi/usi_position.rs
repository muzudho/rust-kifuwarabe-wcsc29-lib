use sheet_music_format::kifu_usi::usi_tape::*;
use std::*;
use studio::board_size::*;
use studio::communication::*;
use studio::parser::*;

pub struct UsiPosition {}
impl UsiPosition {
    /// startpos か、 sfen か、それ以外かに分かれる。
    pub fn parse_startpos_test(
        line: &str,
        start: &mut usize,
        _comm: &Communication,
    ) -> Option<bool> {
        if line.starts_with("position startpos") {
            //comm.println("#Fen: position startpos");
            *start = "position startpos".len();
            Some(true)
        } else if line.starts_with("position sfen ") {
            //comm.println("#position sfen ");
            *start = "position sfen ".len();
            Some(false)
        } else {
            None
        }
    }

    /// USI の moves の文字列を、オブジェクトに直訳するぜ☆（＾～＾）局面は動かさない☆（＾～＾）
    pub fn parse_usi_line_moves(
        comm: &Communication,
        line: &str,
        start: &mut usize,
        board_size: BoardSize,
    ) -> Option<UsiTape> {
        if Parser::match_keyword(&comm, &line, "moves", start)
            || Parser::match_keyword(&comm, &line, " moves", start)
        {
        } else {
            // comm.println(&format!("#Moves not matched. line: '{}', start: {}.", line, start));
            return None;
        }

        UsiTape::parse_usi_all_moves(&comm, line, start, board_size)
    }
}
