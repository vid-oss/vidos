use derive_more::{Display, From, Into};

use crate::{time_range::TimeRange, video::VideoId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From, Into)]
pub struct SubtitleId {
    id: i64,
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
