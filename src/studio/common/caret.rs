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

/// ゼロおよび正の数では、（キャレット番号＋１）と、（要素の個数）は等しい。
/// 負の数では、（キャレット番号の絶対値）と、（要素の個数）は等しい。
pub fn caret_number_to_length(caret_number: i16) -> usize {
    if -1 < caret_number {
        (caret_number + 1) as usize
    } else {
        -caret_number as usize
    }
}

// 意識。キャレットを go_to_next すると作成される。
#[derive(Debug)]
pub struct Awareness {
    // 予想する移動後のキャレットの位置。
    pub expected_caret_number: i16,
    // 配列のインデックス。
    pub index: usize,
    // 負の方の配列か。
    pub negative: bool,
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

        // 向きを変えるだけでは、回転テーブル・ターン☆（＾～＾）
        self.facing_left = !self.facing_left;

        // 振り返ってから、１歩前へ☆（＾～＾）
        //self.go_to_next(&app);
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
            index: aware_index,
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

            // 向きを変えるだけでは、回転テーブル・ターン☆（＾～＾）
            self.facing_left = true;

            // 振り返ってから、１歩前へ☆（＾～＾）
            //self.go_to_next(&app);
        }
    }

    /// その場で、向きだけ変えるぜ☆（＾～＾）
    pub fn turn_towards_positive_infinity(&mut self, app: &Application) {
        if self.is_facing_left() {
            if app.is_debug() {
                app.comm.println("[#Turn towards -->]");
            }

            // 向きを変えるだけでは、回転テーブル・ターン☆（＾～＾）
            self.facing_left = false;

            // 振り返ってから、１歩前へ☆（＾～＾）
            //self.go_to_next(&app);
        }
    }

    /// トランケート用に使う。
    ///
    /// 向いている方向に関わらず、正か負かを返します。
    /// インデックスを返します。負の配列では 数を 0 側に 1 つ寄せます。
    ///
    /// # Returns
    ///
    /// (is_positive, index)
    pub fn to_index_for_truncation(&self) -> (bool, usize) {
        // 正と負で、0 の扱い方が異なることに注意。
        if self.is_facing_left() {
            // 負の無限大の方を向いているとき。
            if self.unconscious_number <= 0 {
                // 0以下の左隣は負
                (false, get_index_from_caret_numbers(self.unconscious_number))
            } else {
                // 1以上の左隣は正。
                (true, get_index_from_caret_numbers(self.unconscious_number))
            }
        } else {
            // 正の無限大の方を向いているとき。
            if self.unconscious_number >= 0 {
                // 0以上の右隣は正。
                (true, get_index_from_caret_numbers(self.unconscious_number))
            } else {
                // 0未満の右隣は負。
                (false, get_index_from_caret_numbers(self.unconscious_number))
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
