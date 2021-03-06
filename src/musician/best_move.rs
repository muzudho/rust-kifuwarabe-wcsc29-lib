use instrument::piece_etc::*;
use sheet_music_format::kifu_usi::usi_move::*;
use studio::address::*;
use studio::application::Application;
use studio::board_size::*;

/// (Usi move, どの駒を動かした一手か, どこの駒を動かした一手か, あれば取った駒，取った駒の番地)
#[derive(Clone)]
pub struct BestMove {
    pub usi_move: UsiMove,
    pub subject_pid: PieceIdentify,
    pub subject_addr: Address,
    pub capture_pid: Option<PieceIdentify>,
    pub capture_addr: Option<Address>,
}
impl BestMove {
    /// Human presentable.
    pub fn to_human_presentable(&self, board_size: BoardSize, app: &Application) -> String {
        format!(
            "[Best move: Usi: '{}', SubjPid: {}, SubjAddr: '{}', CapPid: {}, CapAddr: '{}']",
            self.usi_move.to_sign(&app),
            self.subject_pid.to_human_presentable_4width(),
            self.subject_addr.to_human_presentable(board_size),
            if let Some(cap_id) = self.capture_pid {
                cap_id.to_human_presentable_4width()
            } else {
                "None".to_string()
            },
            if let Some(cap_addr) = self.capture_addr {
                cap_addr.to_human_presentable(board_size)
            } else {
                "None".to_string()
            },
        )
    }
}
