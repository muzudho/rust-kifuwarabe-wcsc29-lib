use audio_compo::cassette_deck::CassetteDeck;
use audio_compo::cassette_deck::Slot;
use studio::application::Application;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HalfPlayerPhaseValue {
    /// 0.5.
    ZeroPointFive,
    /// Starting first.
    First,
    /// 1.5.
    OnePointFive,
    /// Starting second.
    Second,
}
impl HalfPlayerPhaseValue {
    /// Human presentalbe.
    pub fn to_log(self) -> String {
        use instrument::half_player_phase::HalfPlayerPhaseValue::*;
        match self {
            ZeroPointFive => "＜",
            First => "▼",
            OnePointFive => "＞",
            Second => "△",
        }
        .to_string()
    }

    pub fn to_sign(self) -> String {
        use instrument::half_player_phase::HalfPlayerPhaseValue::*;
        match self {
            ZeroPointFive => "z",
            First => "b",
            OnePointFive => "o",
            Second => "w",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct HalfPlayerPhaseObject {
    // デバッグ用のでたらめな数字。
    hint_owner: i64,
    state: HalfPlayerPhaseValue,
}
impl HalfPlayerPhaseObject {
    pub fn new_empty(app: &Application, hint_owner_num: i64) -> Self {
        if app.is_debug() {
            app.comm.println("[#phase.new]");
        }

        HalfPlayerPhaseObject {
            hint_owner: hint_owner_num,
            state: HalfPlayerPhaseValue::ZeroPointFive,
        }
    }

    pub fn from_value(
        init_value: HalfPlayerPhaseValue,
        app: &Application,
        hint_owner_num: i64,
    ) -> Self {
        if app.is_debug() {
            app.comm.println("[#phase.from_value]");
        }

        HalfPlayerPhaseObject {
            hint_owner: hint_owner_num,
            state: init_value,
        }
    }

    pub fn repeat_phase(&mut self, app: &Application) {
        if app.is_debug() {
            app.comm.println("[#phase.repeat_phase]");
        }
        self.state = HalfPlayerPhaseValue::ZeroPointFive;
    }

    pub fn get_state(&self) -> HalfPlayerPhaseValue {
        self.state
    }

    pub fn is_half(&self) -> bool {
        self.state == HalfPlayerPhaseValue::ZeroPointFive
            || self.state == HalfPlayerPhaseValue::OnePointFive
    }

    /// 隣へ☆（＾ｑ＾）！ position用☆（＾～＾）
    pub fn go_next_phase_for_position(&mut self, deck: &CassetteDeck, slot: Slot) {
        if let Some(ref tape_box) = &deck.slots[slot as usize].tape_box {
            use instrument::half_player_phase::HalfPlayerPhaseValue::*;
            if tape_box.is_facing_left_of_current_tape() {
                self.state = match self.state {
                    ZeroPointFive => Second,
                    First => ZeroPointFive,
                    OnePointFive => First,
                    Second => OnePointFive,
                };
            } else {
                self.state = match self.state {
                    ZeroPointFive => First,
                    First => OnePointFive,
                    OnePointFive => Second,
                    Second => ZeroPointFive,
                };
            }
        } else {
            panic!("Please choice tape box.");
        }
    }

    /// 点対称に回転☆（＾ｑ＾）！
    pub fn rotate_point_symmetrically(&mut self, app: &Application) {
        if app.is_debug() {
            app.comm.println("[#phase.rotate_point_symmetrically]");
        }

        use instrument::half_player_phase::HalfPlayerPhaseValue::*;
        self.state = match self.state {
            ZeroPointFive => OnePointFive,
            First => Second,
            OnePointFive => ZeroPointFive,
            Second => First,
        };
    }
}
