use derive_more::Display;
use time::Duration;

#[derive(Debug, Clone, Display)]
#[display(fmt = "{start} - {end}")]
pub struct TimeRange {
    pub start: Duration,
    pub end: Duration,
}

impl TimeRange {
    pub fn new(start: Duration, end: Duration) -> Self {
        Self { start, end }
    }
}
