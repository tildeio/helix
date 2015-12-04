use Duration;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Unit {
    Years,
    Months,
    Days,
    Minutes,
    Seconds
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Part {
    pub amount: i64,
    pub unit: Unit
}

pub trait CalculateTime {
    fn advance(&self, part: Part) -> Self;
}
