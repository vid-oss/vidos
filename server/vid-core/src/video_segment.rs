use derive_more::Display;

use crate::{object::ObjectLocation, time_range::TimeRange, video::VideoId};

#[derive(Debug, Clone)]
pub struct VideoSegment {
    video_id: VideoId,
    time_range: TimeRange,
    object_location: ObjectLocation,
}

#[derive(Debug, Clone, Copy, Display)]
#[display(fmt = "{width}x{height} px")]
pub struct Resolution {
    width: u32,
    height: u32,
}
