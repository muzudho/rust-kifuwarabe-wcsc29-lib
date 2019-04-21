pub struct Caret {
    back: bool,
    number: i16,
}
impl Caret {
    pub fn new() -> Self {
        Caret {
            back: false,
            number: 0,
        }
    }

    pub fn is_back(self) -> bool {
        self.back
    }

    /// target 以上。
    pub fn is_greater_than_or_equal_to(&self, target: i16) -> bool {
        target <= self.number
    }

    /// 向きの通りに移動します。
    pub fn get_and_move(&mut self) -> i16 {
        let old = self.number;

        if self.back {
            self.number -= 1;
        } else {
            self.number += 1;
        }

        old
    }

    /// 逆向きに移動します。
    pub fn cancel_and_get(&mut self) -> i16 {
        if self.back {
            self.number += 1;
        } else {
            self.number -= 1;
        }

        self.number
    }
}
