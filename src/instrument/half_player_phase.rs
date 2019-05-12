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
    state: HalfPlayerPhaseValue,
}
impl HalfPlayerPhaseObject {
    // ###############
    // # Constructor #
    // ###############

    pub fn new_empty(app: &Application) -> Self {
        if app.is_debug() {
            app.comm.println("[#phase.new]");
        }

        Self {
            state: HalfPlayerPhaseValue::ZeroPointFive,
        }
    }

    pub fn from_value(init_value: HalfPlayerPhaseValue) -> Self {
        Self { state: init_value }
    }

    // #####
    // # G #
    // #####

    pub fn get_state(self) -> HalfPlayerPhaseValue {
        self.state
    }

    // #####
    // # I #
    // #####

    pub fn is_half(self) -> bool {
        self.state == HalfPlayerPhaseValue::ZeroPointFive
            || self.state == HalfPlayerPhaseValue::OnePointFive
    }

    // #####
    // # L #
    // #####

    pub fn look_back_for_position(&mut self, deck: &mut CassetteDeck, app: &Application) {
        deck.slots[Slot::Learning as usize].look_back_caret(&app);
    }

    // #####
    // # R #
    // #####

    pub fn repeat_phase(&mut self, app: &Application) {
        if app.is_debug() {
            app.comm.println("[#phase.repeat_phase]");
        }
        self.state = HalfPlayerPhaseValue::ZeroPointFive;
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

    // #####
    // # S #
    // #####

    /// 隣へ☆（＾ｑ＾）！ position用☆（＾～＾）ラーニング・テープと関連付くぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `is_facing_left` - ラーニング・テープのキャレットの向き。
    pub fn seek_a_player_for_position(&mut self, is_facing_left: bool, app: &Application) {
        let old_state = self.get_state();

        use instrument::half_player_phase::HalfPlayerPhaseValue::*;
        self.state = if is_facing_left {
            // 左向きのとき。
            match self.state {
                ZeroPointFive => Second,
                First => ZeroPointFive,
                OnePointFive => First,
                Second => OnePointFive,
            }
        } else {
            // 右向きのとき。
            match self.state {
                ZeroPointFive => First,
                First => OnePointFive,
                OnePointFive => Second,
                Second => ZeroPointFive,
            }
        };

        if app.is_debug() {
            if is_facing_left {
                app.comm.println(&format!(
                    "[#HalfPlayerPhase: フェーズチェンジ {:?}<--{:?}]",
                    self.get_state(),
                    old_state,
                ));
            } else {
                app.comm.println(&format!(
                    "[#HalfPlayerPhase: フェーズチェンジ {:?}-->{:?}]",
                    old_state,
                    self.get_state(),
                ));
            }
        }
    }
}
