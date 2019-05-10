/// 閉区間。両端を含む。
/// 向きは 基本的に正の方向（昇順）。
/// キャレット番号を入れる。（インデックスではない）
pub struct ClosedInterval {
    minimum: i16,
    maximum: i16,
    facing_left: bool,
}
impl ClosedInterval {
    // ###############
    // # Constructor #
    // ###############

    pub fn new_facing_right() -> Self {
        ClosedInterval {
            minimum: std::i16::MAX,
            maximum: std::i16::MIN,
            facing_left: false,
        }
    }

    // #####
    // # F #
    // #####

    /// # Arguments
    ///
    /// * `value0` - min, max を調べるのがめんどくさいんで、どっちでもいい☆（＾～＾）
    pub fn from_all(value0: i16, value1: i16, facing_left_flag: bool) -> Self {
        let (min, max) = if value0 <= value1 {
            (value0, value1)
        } else {
            (value1, value0)
        };

        ClosedInterval {
            minimum: min,
            maximum: max,
            facing_left: facing_left_flag,
        }
    }

    // #####
    // # G #
    // #####

    pub fn get_minimum_caret_number(&self) -> i16 {
        self.minimum
    }
    pub fn get_maximum_caret_number(&self) -> i16 {
        self.maximum
    }

    pub fn get_start(&self) -> i16 {
        if self.is_facing_left() {
            self.maximum
        } else {
            self.minimum
        }
    }
    pub fn get_end(&self) -> i16 {
        if self.is_facing_left() {
            self.minimum
        } else {
            self.maximum
        }
    }

    // #####
    // # I #
    // #####

    pub fn is_facing_left(&self) -> bool {
        self.facing_left
    }

    pub fn intersect_caret_number(&mut self, caret_number: i16) {
        if caret_number < self.minimum {
            self.minimum = caret_number;
        }

        if self.maximum < caret_number {
            self.maximum = caret_number;
        }
    }

    pub fn intersect_closed_interval(&mut self, closed_interval: ClosedInterval) {
        if closed_interval.minimum < self.minimum {
            self.minimum = closed_interval.minimum;
        }

        if self.maximum < closed_interval.maximum {
            self.maximum = closed_interval.maximum;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.maximum < self.minimum
    }

    // #####
    // # L #
    // #####

    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            (self.maximum - self.minimum + 1) as usize
        }
    }

    // #####
    // # T #
    // #####

    pub fn to_human_presentable(&self) -> String {
        format!(
            "[CloseInterval({}<={}), Len: {}]",
            self.minimum,
            self.maximum,
            self.len()
        )
        .to_string()
    }
}
