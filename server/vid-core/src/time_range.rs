use derive_more::Display;
use thiserror::Error;
use time::Duration;

#[derive(Debug, Clone, Display)]
#[display(fmt = "{start} - {end}")]
pub struct TimeRange {
    pub start: Duration,
    pub end: Duration,
}

#[derive(Error, Debug)]
pub enum TimeRangeError {
    #[error("start cannot be negative")]
    NegativeStart(Duration),

    #[error("end cannot be negative")]
    NegativeEnd(Duration),

    #[error("end cannot be less than start")]
    EndLessThanStart { start: Duration, end: Duration },
}

impl TimeRange {
    pub fn new(start: Duration, end: Duration) -> Result<Self, TimeRangeError> {
        match (start, end) {
            (s, _) if s.is_negative() => Err(TimeRangeError::NegativeStart(s)),
            (_, e) if e.is_negative() => Err(TimeRangeError::NegativeEnd(e)),
            (s, e) if e < s => Err(TimeRangeError::EndLessThanStart { start: s, end: e }),
            (s, e) => Ok(Self { start: s, end: e }),
        }
    }
}
