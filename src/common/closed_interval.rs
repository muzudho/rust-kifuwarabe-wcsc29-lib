/// 閉区間。両端を含む。方向は持たない。
pub struct ClosedInterval {
    minimum: i16,
    maximum: i16,
}
impl ClosedInterval {
    pub fn new() -> Self {
        ClosedInterval {
            minimum: std::i16::MAX,
            maximum: std::i16::MIN,
        }
    }

    pub fn get_minimum(&self) -> i16 {
        self.minimum
    }
    pub fn get_maximum(&self) -> i16 {
        self.maximum
    }

    pub fn intersect(&mut self, value: i16) {
        if value < self.minimum {
            self.minimum = value;
        }

        if self.maximum < value {
            self.maximum = value;
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

    pub fn to_human_presentable(&self) -> String {
        format!("{}:{}", self.minimum, self.maximum).to_string()
    }
}
