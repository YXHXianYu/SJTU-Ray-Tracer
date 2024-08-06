pub struct Interval {
    pub min: f64,
    pub max: f64
}

#[allow(dead_code)]
impl Interval {
    pub fn new() -> Interval {
        Interval::empty()
    }
    pub fn empty() -> Interval {
        Interval { min: f64::INFINITY, max: -f64::INFINITY }
    }
    pub fn universe() -> Interval {
        Interval { min: -f64::INFINITY, max: f64::INFINITY }
    }
    pub fn from(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { self.min }
        else if x > self.max { self.max }
        else { x }
    }
}
