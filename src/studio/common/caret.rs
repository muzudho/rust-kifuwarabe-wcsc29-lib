use studio::application::Application;
use studio::common::closed_interval::ClosedInterval;

/// 指し手をシークした結果は３つだぜ☆（＾～＾）
#[derive(Debug)]
pub enum SoughtMoveResult {
    // シークできた。
    Aware,
    // 端っこで、これ以上シークできない、
    Forever,
    // 非合法手だったりして、シークしなかった。
    Dream,
}

/// 負の方のキャレット番地を符号を反転して１引いて配列のインデックスを作る補正に使う☆（＾～＾）
pub const MINUS_ZERO_LEN: usize = 1;
/// Obsolute.
pub fn get_index_from_caret_numbers(caret_number: i16) -> usize {
    if -1 < caret_number {
        caret_number as usize
    } else {
        -caret_number as usize + MINUS_ZERO_LEN
    }
}

// 意識。キャレットを go_to_next すると作成される。
#[derive(Debug)]
pub struct Awareness {
    // 予想する移動後のキャレットの位置。
    pub expected_caret_number: i16,
    // 配列のインデックス。オーバーしているときに要素が無い方へシークすると None になる。
    pub index: Option<usize>,
    // 負の方の配列か。
    pub negative: bool,
}
impl Awareness {
    pub fn new() -> Self {
        Self {
            expected_caret_number: 0,
            index: None,
            negative: false,
        }
    }
}

/// 余談。
/// キャレットは、クリアーするとかリセットすることはできない☆（＾～＾）
/// 常に現在位置を示す☆（＾～＾）
/// 初期位置は持たない☆（＾～＾）
/// できるか、できないかではない、これは　そうであるべき　という　思想　だぜ☆（*＾～＾*）
pub struct Caret {
    facing_left: bool,
    // キャレットの位置。
    unconscious_number: i16,
}
impl Caret {
    /// マイナスゼロが無いので、負の配列ではインデックスを１小さくします。
    pub const NEGATIVE_ZERO_LEN: i16 = 1;

    // ###############
    // # Constructor #
    // ###############

    pub fn new_facing_right_caret() -> Self {
        Caret::new_facing_right_caret_with_number(0)
    }

    pub fn new_facing_right_caret_with_number(init_num: i16) -> Self {
        Caret {
            facing_left: false,
            unconscious_number: init_num,
        }
    }

    // #####
    // # B #
    // #####

    /// 向きを変えずにうしろに下がるぜ☆（＾～＾）
    pub fn back_walk(&mut self, app: &Application) -> Awareness {
        self.look_back(&app);
        let awareness = self.seek_a_note(&app);
        self.look_back(&app);

        awareness
    }

    // #####
    // # C #
    // #####

    pub fn clear_facing_right(&mut self) {
        self.facing_left = false;
        self.unconscious_number = 0;
    }

    // #####
    // # E #
    // #####

    /// 等しい。
    pub fn equals(&self, target: i16) -> bool {
        self.unconscious_number == target
    }

    // #####
    // # I #
    // #####

    pub fn is_internal_of(&self, closed_interval: ClosedInterval) -> bool {
        closed_interval.get_minimum_caret_number() <= self.unconscious_number
            && self.unconscious_number <= closed_interval.get_maximum_caret_number()
    }

    pub fn is_facing_left(&self) -> bool {
        self.facing_left
    }

    /// target 以上。
    pub fn is_greater_than_or_equal_to(&self, target: i16) -> bool {
        target <= self.unconscious_number
    }

    // #####
    // # L #
    // #####

    /// その場で、向きだけ変えるぜ☆（＾～＾）
    pub fn look_back(&mut self, app: &Application) {
        if app.is_debug() {
            app.comm.println("[#LookBack]");
        }

        // 向きを変えるだけ。
        self.facing_left = !self.facing_left;
    }

    // #####
    // # S #
    // #####

    /// 要素を返してから、向きの通りに移動します。境界チェックは行いません。
    /// 境界なんかないから、どんどん　進んでいくぜ☆（＾～＾）
    pub fn seek_a_note(&mut self, app: &Application) -> Awareness {
        // ゼロおよび正の数では、（キャレット番号）と、（要素の個数＋１）と、（インデックス）は等しい。
        // 負の数では、（キャレット番号の絶対値）と、（要素の個数）と、（インデックス－１）は等しい。
        let aware_index;
        let aware_negative;
        if self.facing_left {
            // 左向き。
            if 0 < self.unconscious_number {
                // 正のところを通る。（逆さ移動）
                //
                // 0 [0] 1 [1] 2
                //
                // で、1から0へ移動した場合 [0] ということに注意。移動後のキャレットがインデックスになる。
                self.unconscious_number -= 1;
                aware_index = self.unconscious_number as usize;
                aware_negative = false;
                if app.is_debug() {
                    app.comm.println("[#Go to next: <-- 正のところを通った]")
                }
            } else {
                // 負のところを通る。
                aware_index = (-self.unconscious_number - 1) as usize;
                self.unconscious_number -= 1;
                aware_negative = true;
                if app.is_debug() {
                    app.comm.println("[#Go to next: <-- 負のところを通った]")
                }
            }
        } else {
            // 右向き。
            if self.unconscious_number < 0 {
                // 負のところを通る。（逆さ移動）
                //
                // -2 [1] -1 [0] 0
                //
                // で、-1から0へ移動した場合 [0] ということに注意。移動後のキャレットの絶対値がインデックスになる。
                self.unconscious_number += 1;
                aware_index = (-self.unconscious_number) as usize;
                aware_negative = true;
                if app.is_debug() {
                    app.comm.println("[#Go to next: --> 負のところを通った]")
                }
            } else {
                // 正のところを通る。
                aware_index = self.unconscious_number as usize;
                self.unconscious_number += 1;
                aware_negative = false;
                if app.is_debug() {
                    app.comm.println("[#Go to next: --> 正のところを通った]")
                }
            }
        }

        Awareness {
            expected_caret_number: self.unconscious_number,
            index: Some(aware_index),
            negative: aware_negative,
        }
    }

    /// 足踏みする（step in）☆（＾～＾）
    /// キャレットの現在番地が欲しいケースがけっこうある☆（＾～＾）
    /// 思想上、なるべく go_to_next() を使えだぜ☆（＾～＾）
    pub fn step_in(&self) -> i16 {
        self.unconscious_number
    }

    // #####
    // # T #
    // #####

    /// その場で、向きだけ変えるぜ☆（＾～＾）
    pub fn turn_towards_negative_infinity(&mut self, app: &Application) {
        if !self.is_facing_left() {
            if app.is_debug() {
                app.comm.println("[#Turn towards <--]");
            }

            // 向きを変えるだけ。
            self.facing_left = true;
        }
    }

    /// その場で、向きだけ変えるぜ☆（＾～＾）
    pub fn turn_towards_positive_infinity(&mut self, app: &Application) {
        if self.is_facing_left() {
            if app.is_debug() {
                app.comm.println("[#Turn towards -->]");
            }

            // 向きを変えるだけ。
            self.facing_left = false;
        }
    }

    /// キャレットのあるところから見て、０に近い方の隣の要素のインデックスを返します。
    /// キャレットが　０　にあるときは　インデックスは　None 　になりますが、
    /// 正、負　は　キャレットの向きに合わさります。
    ///
    /// # Returns
    ///
    /// (awareness)
    pub fn get_peak(&self) -> Awareness {
        if self.unconscious_number == 0 {
            // キャレットが 0番地 にあるとき。

            Awareness {
                expected_caret_number: self.unconscious_number,
                index: None,
                negative: self.is_facing_left(),
            }
        } else if self.unconscious_number < 0 {
            // キャレットが 負の方にあるとき。
            Awareness {
                expected_caret_number: self.unconscious_number,
                index: Some((-self.unconscious_number - 1) as usize),
                negative: true,
            }
        } else {
            // キャレットが 正の方にあるとき。
            Awareness {
                expected_caret_number: self.unconscious_number,
                index: Some((self.unconscious_number - 1) as usize),
                negative: false,
            }
        }
    }

    /// デバッグ表示用。
    pub fn to_human_presentable(&self, _app: &Application) -> String {
        if self.is_facing_left() {
            format!("[Caret: <--{}]", self.unconscious_number).to_string()
        } else {
            format!("[Caret: {}-->]", self.unconscious_number).to_string()
        }
    }

    // #####
    // # W #
    // #####

    pub fn while_to(&self, target: &ClosedInterval, _app: &Application) -> bool {
        if self.is_facing_left() {
            /*
            app.comm.print(&format!(
                "[min:{}, num:{}]",
                target.get_minimum_caret_number(),
                self.number
            ));
            */
            target.get_minimum_caret_number() <= self.unconscious_number
        } else {
            /*
            app.comm.print(&format!(
                "[num:{}, max:{}]",
                self.number,
                target.get_maximum_caret_number(),
            ));
            */
            self.unconscious_number <= target.get_maximum_caret_number()
        }
    }
}
