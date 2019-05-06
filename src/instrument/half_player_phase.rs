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
    value: HalfPlayerPhaseValue,
}
impl HalfPlayerPhaseObject {
    pub fn new() -> Self {
        HalfPlayerPhaseObject {
            value: HalfPlayerPhaseValue::ZeroPointFive,
        }
    }

    pub fn from_value(init_value: HalfPlayerPhaseValue) -> Self {
        HalfPlayerPhaseObject { value: init_value }
    }

    pub fn get_value(&self) -> HalfPlayerPhaseValue {
        self.value
    }

    pub fn is_half(&self) -> bool {
        self.value == HalfPlayerPhaseValue::ZeroPointFive
            || self.value == HalfPlayerPhaseValue::OnePointFive
    }

    /// 隣へ☆（＾ｑ＾）！
    pub fn go_next(&mut self, deck: &CassetteDeck, slot: Slot, app: &Application) {
        if let Some(ref tape_box) = &deck.slots[slot as usize].tape_box {
            if app.is_debug() {
                app.comm.println(&format!(
                    "[#phase.go_next Start: {:?}, Slot: {:?}, FacingLeft: {}]",
                    self.value,
                    slot,
                    tape_box.is_facing_left_of_current_tape()
                ));
            }

            use instrument::half_player_phase::HalfPlayerPhaseValue::*;
            if tape_box.is_facing_left_of_current_tape() {
                self.value = match self.value {
                    ZeroPointFive => Second,
                    First => ZeroPointFive,
                    OnePointFive => First,
                    Second => OnePointFive,
                };
            } else {
                self.value = match self.value {
                    ZeroPointFive => First,
                    First => OnePointFive,
                    OnePointFive => Second,
                    Second => ZeroPointFive,
                };
            }

            if app.is_debug() {
                app.comm.println(&format!(
                    "[#phase.go_next End: {:?}, Slot: {:?}, FacingLeft: {}]",
                    self.value,
                    slot,
                    tape_box.is_facing_left_of_current_tape()
                ));
            }
        } else {
            panic!("Please choice tape box.");
        }
    }

    /// 点対称に回転☆（＾ｑ＾）！
    pub fn rotate_point_symmetrically(&mut self) {
        use instrument::half_player_phase::HalfPlayerPhaseValue::*;
        self.value = match self.value {
            ZeroPointFive => OnePointFive,
            First => Second,
            OnePointFive => ZeroPointFive,
            Second => First,
        };
    }
}
