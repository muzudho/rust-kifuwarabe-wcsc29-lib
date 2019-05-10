use studio::application::Application;
use studio::common::closed_interval::ClosedInterval;

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
    pub fn new_facing_right_caret() -> Self {
        Caret::new_facing_right_caret_with_number(0)
    }

    pub fn new_facing_right_caret_with_number(init_num: i16) -> Self {
        Caret {
            facing_left: false,
            unconscious_number: init_num,
        }
    }

    pub fn clear_facing_right(&mut self) {
        self.facing_left = false;
        self.unconscious_number = 0;
    }

    /// 要素を返してから、向きの通りに移動します。境界チェックは行いません。
    /// 境界なんかないから、どんどん　進んでいくぜ☆（＾～＾）
    pub fn go_to_next(&mut self, _app: &Application) -> Awareness {
        let awareness;

        // ゼロおよび正の数では、（キャレット番号）と、（要素の個数＋１）と、（インデックス）は等しい。
        // 負の数では、（キャレット番号の絶対値）と、（要素の個数）と、（インデックス－１）は等しい。
        let old_negative;
        if self.facing_left {
            // 左向き。
            if -1 < self.unconscious_number {
                // 正の方の配列にいたのだった場合。
                old_negative = false;
            } else {
                // 負の方の配列にいたのだった場合。
                old_negative = true;
            }
        } else {
            // 右向き。
            if self.unconscious_number < 0 {
                // 負の方の配列にいたのだった場合。
                old_negative = true;
            } else {
                // 正の方の配列にいたのだった場合。
                old_negative = false;
            }
        }

        let old_number = self.unconscious_number;

        if self.facing_left {
            // 左向き。
            self.unconscious_number -= 1;
        } else {
            // 右向き。
            self.unconscious_number += 1;
        }

        awareness = if old_negative {
            // 負の方の配列にいたのだった場合。
            Awareness {
                expected_caret_number: self.unconscious_number,
                index: (-old_number - 1) as usize,
                negative: true,
            }
        } else {
            // 正の方の配列にいたのだった場合。
            Awareness {
                expected_caret_number: self.unconscious_number,
                index: old_number as usize,
                negative: true,
            }
        };

        /*
        // ログ出力☆（＾～＾）
        {
            if self.is_facing_left() {
                app.comm
                    .print(&format!("[Caret: {}<--{}]", self.number, old).to_string());
            } else {
                app.comm
                    .print(&format!("[Caret: {}-->{}]", old, self.number).to_string());
            }
        }
         */

        awareness
    }

    /// 足踏みする（step in）☆（＾～＾）
    /// キャレットの現在番地が欲しいケースがけっこうある☆（＾～＾）
    /// 思想上、なるべく go_to_next() を使えだぜ☆（＾～＾）
    pub fn step_in(&self) -> i16 {
        self.unconscious_number
    }

    /// TODO ちょっと戻りたいときに☆（＾～＾）できれば使わない方がいい☆（＾～＾）
    pub fn go_back(&mut self, _app: &Application) -> Awareness {
        // ゼロおよび正の数では、（キャレット番号）と、（要素の個数＋１）と、（インデックス）は等しい。
        // 負の数では、（キャレット番号の絶対値）と、（要素の個数）と、（インデックス－１）は等しい。
        let old_negative;
        if self.facing_left {
            // 左向き。
            if -1 < self.unconscious_number {
                // 正の方の配列にいたのだった場合。
                old_negative = false;
            } else {
                // 負の方の配列にいたのだった場合。
                old_negative = true;
            }
        } else {
            // 右向き。
            if self.unconscious_number < 0 {
                // 負の方の配列にいたのだった場合。
                old_negative = true;
            } else {
                // 正の方の配列にいたのだった場合。
                old_negative = false;
            }
        }

        let old_number = self.unconscious_number;

        // 逆方向に進む☆（＾～＾）
        if self.facing_left {
            // 左向き。
            self.unconscious_number += 1;
        } else {
            // 右向き。
            self.unconscious_number -= 1;
        }

        if old_negative {
            // 負の方の配列にいたのだった場合。
            Awareness {
                expected_caret_number: self.unconscious_number,
                index: (-old_number - 1) as usize,
                negative: true,
            }
        } else {
            // 正の方の配列にいたのだった場合。
            Awareness {
                expected_caret_number: self.unconscious_number,
                index: old_number as usize,
                negative: true,
            }
        }
    }

    pub fn is_internal_of(&self, closed_interval: ClosedInterval) -> bool {
        closed_interval.get_minimum_caret_number() <= self.unconscious_number
            && self.unconscious_number <= closed_interval.get_maximum_caret_number()
    }

    /// その場で、向きだけ変えるぜ☆（＾～＾）
    pub fn look_back_to_negative(&mut self, app: &Application) {
        // app.comm.print("[LookBack N]");
        if !self.is_facing_left() {
            // 向きを変えるだけでは、回転テーブル・ターン☆（＾～＾）
            self.facing_left = true;

            // 振り返ってから、１歩前へ☆（＾～＾）
            self.go_to_next(&app);
        }
    }

    /// その場で、向きだけ変えるぜ☆（＾～＾）
    pub fn look_back_to_positive(&mut self, app: &Application) {
        // app.comm.print("[LookBack P]");
        if self.is_facing_left() {
            // 向きを変えるだけでは、回転テーブル・ターン☆（＾～＾）
            self.facing_left = false;

            // 振り返ってから、１歩前へ☆（＾～＾）
            self.go_to_next(&app);
        }
    }

    /// その場で、向きだけ変えるぜ☆（＾～＾）
    pub fn look_back_to_opponent(&mut self, app: &Application) {
        // app.comm.print("[LookBack O]");

        // 向きを変えるだけでは、回転テーブル・ターン☆（＾～＾）
        self.facing_left = !self.facing_left;

        // 振り返ってから、１歩前へ☆（＾～＾）
        self.go_to_next(&app);
    }

    pub fn is_facing_left(&self) -> bool {
        self.facing_left
    }

    /// 等しい。
    pub fn equals(&self, target: i16) -> bool {
        self.unconscious_number == target
    }
    /// target 以上。
    pub fn is_greater_than_or_equal_to(&self, target: i16) -> bool {
        target <= self.unconscious_number
    }
    pub fn while_to(&self, target: &ClosedInterval, _app: &Application) -> bool {
        if self.is_facing_left() {
            /*
            app.comm.print(&format!(
                "[min:{}, num:{}]",
                target.get_minimum_caret_number(),
                self.number
            ));
            */
            target.get_minimum_caret_number() < self.unconscious_number
        } else {
            /*
            app.comm.print(&format!(
                "[num:{}, max:{}]",
                self.number,
                target.get_maximum_caret_number(),
            ));
            */
            self.unconscious_number < target.get_maximum_caret_number()
        }
    }

    /// マイナスゼロが無いので、負の配列ではインデックスを１小さくします。
    pub const NEGATIVE_ZERO_LEN: i16 = 1;

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
}
