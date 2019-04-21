use communication::*;

pub struct Caret {
    back: bool,
    number: i16,
}
impl Caret {
    pub fn new_next_caret() -> Self {
        Caret {
            back: false,
            number: 0,
        }
    }

    pub fn new_back_caret(last_number: i16) -> Self {
        Caret {
            back: true,
            number: last_number,
        }
    }

    pub fn reset(&mut self) {
        self.number = 0;
    }

    pub fn turn_to_negative(&mut self) {
        if !self.is_back() {
            self.back = true;
        }
    }

    pub fn turn_to_positive(&mut self) {
        if self.is_back() {
            self.back = false;
        }
    }

    pub fn turn_to_opponent(&mut self) {
        self.back = !self.back;
    }

    pub fn to_human_presentable(&self) -> String {
        if self.is_back() {
            format!("[<--{}]", self.number).to_string()
        } else {
            format!("[{}-->]", self.number).to_string()
        }
    }

    pub fn is_back(&self) -> bool {
        self.back
    }

    /// 等しい。
    pub fn equals(&self, target: i16) -> bool {
        self.number == target
    }
    /// target 以上。
    pub fn is_greater_than_or_equal_to(&self, target: i16) -> bool {
        target <= self.number
    }

    /// 要素を返してから、向きの通りに移動します。
    pub fn get_and_go(&mut self, comm: &Communication, hint: &str) -> i16 {
        let old = self.number;

        if self.back {
            self.number -= 1;
        } else {
            self.number += 1;
        }
        comm.println(&format!(
            "<Old{},New{}:{}>",
            old,
            &self.to_human_presentable(),
            hint
        ));

        old
    }

    /// 配列のインデックスに変換します。
    /// 負の方向は 数を 0 側に 1 つ寄せます。
    ///
    /// # Returns
    ///
    /// (is_positive, index)
    pub fn to_index(&self) -> (bool, usize) {
        if self.number < 0 {
            (false, (-self.number - 1) as usize)
        } else {
            (true, self.number as usize)
        }
    }
}
