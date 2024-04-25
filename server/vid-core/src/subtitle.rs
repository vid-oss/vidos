use derive_more::Display;

use crate::{time_range::TimeRange, video::VideoId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct SubtitleId {
    id: i64,
}

impl From<i64> for SubtitleId {
    fn from(value: i64) -> Self {
        Self { id: value }
    }
}

impl SubtitleId {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone)]
pub struct Subtitle {
    pub id: SubtitleId,
    pub video_id: VideoId,
    pub text: String,
    pub time_range: TimeRange,
}
