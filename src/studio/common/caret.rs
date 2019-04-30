use studio::common::closed_interval::ClosedInterval;
use studio::communication::*;

/// 負の方のキャレット番地を符号を反転して１引いて配列のインデックスを作る補正に使う☆（＾～＾）
pub const MINUS_ZERO_LEN: usize = 1;
pub fn get_index_from_caret_numbers(caret_number: i16) -> usize {
    if -1 < caret_number {
        caret_number as usize
    } else {
        -caret_number as usize + MINUS_ZERO_LEN
    }
}

pub struct Caret {
    facing_left: bool,
    number: i16,
}
impl Caret {
    pub fn new_facing_right_caret() -> Self {
        Caret::new_facing_right_caret_with_number(0)
    }

    pub fn new_facing_right_caret_with_number(init_num: i16) -> Self {
        let mut brandnew = Caret {
            facing_left: false,
            number: init_num,
        };

        // Human care.
        brandnew.clear_facing_right();

        brandnew
    }

    pub fn clear_facing_right(&mut self) {
        self.facing_left = false;
        self.number = 0;
    }

    /// TODO リセットは 0 に戻るでいいのか☆（＾～＾）？
    pub fn reset(&mut self) {
        self.number = 0;
    }

    /// 要素を返してから、向きの通りに移動します。境界チェックは行いません。
    /// 境界なんかないから、どんどん　進んでいくぜ☆（＾～＾）
    pub fn go_to_next(&mut self, _comm: &Communication) -> i16 {
        let old = self.number;

        if self.facing_left {
            self.number -= 1;
        } else {
            self.number += 1;
        }

        old
    }

    /// 足踏みする☆（＾～＾）
    /// ミュータブルにしたくない場合だけ使う☆（＾～＾）なるべく go_to_next を使えだぜ☆（＾～＾）
    pub fn step_in(&self, _comm: &Communication) -> i16 {
        self.number
    }

    /// ちょっと戻りたいときに☆（＾～＾）
    pub fn go_back(&mut self, _comm: &Communication) -> i16 {
        let old = self.number;

        if self.facing_left {
            self.number += 1;
        } else {
            self.number -= 1;
        }

        old
    }

    pub fn is_internal_of(&self, closed_interval: ClosedInterval) -> bool {
        closed_interval.get_minimum_caret_number() <= self.number
            && self.number <= closed_interval.get_maximum_caret_number()
    }

    pub fn turn_to_negative(&mut self) {
        if !self.is_facing_left() {
            self.facing_left = true;
        }
    }

    pub fn turn_to_positive(&mut self) {
        if self.is_facing_left() {
            self.facing_left = false;
        }
    }

    pub fn turn_to_opponent(&mut self) {
        self.facing_left = !self.facing_left;
    }

    pub fn is_facing_left(&self) -> bool {
        self.facing_left
    }

    /// 等しい。
    pub fn equals(&self, target: i16) -> bool {
        self.number == target
    }
    /// target 以上。
    pub fn is_greater_than_or_equal_to(&self, target: i16) -> bool {
        target <= self.number
    }
    pub fn while_to(&self, target: &ClosedInterval) -> bool {
        if self.is_facing_left() {
            target.get_minimum_caret_number() < self.number
        } else {
            self.number < target.get_maximum_caret_number()
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
            if self.number <= 0 {
                // 0以下の左隣は負
                (false, get_index_from_caret_numbers(self.number))
            } else {
                // 1以上の左隣は正。
                (true, get_index_from_caret_numbers(self.number))
            }
        } else {
            // 正の無限大の方を向いているとき。
            if self.number >= 0 {
                // 0以上の右隣は正。
                (true, get_index_from_caret_numbers(self.number))
            } else {
                // 0未満の右隣は負。
                (false, get_index_from_caret_numbers(self.number))
            }
        }
    }

    /// デバッグ表示用。
    pub fn to_human_presentable(&self) -> String {
        if self.is_facing_left() {
            format!("[<--{}]", self.number).to_string()
        } else {
            format!("[{}-->]", self.number).to_string()
        }
    }
}
