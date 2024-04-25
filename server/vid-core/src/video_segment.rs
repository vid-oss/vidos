use crate::{
    object::ObjectLocation, resolution::Resolution, time_range::TimeRange, video::VideoId,
};

#[derive(Debug, Clone)]
pub struct VideoSegment {
    video_id: VideoId,
    time_range: TimeRange,
    object_location: ObjectLocation,
    resoulution: Resolution,
}
