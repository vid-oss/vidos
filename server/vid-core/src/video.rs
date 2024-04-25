use derive_more::{Display, From, Into};
use isolang::Language;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{object::ObjectLocation, user::UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From, Into)]
pub struct VideoId {
    id: Uuid,
}

#[derive(Debug, Clone)]
pub struct Video {
    id: VideoId,
    created_at: OffsetDateTime,
    created_by: UserId,
    langauge: Language,
    source_location: SourceLocation,
    is_published: bool,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    video: ObjectLocation,
    subtitles: ObjectLocation,
}

#[derive(Debug, Clone)]
pub enum VideoDescriptor {
    Movie(MovieDescriptor),
    Episode(EpisodeDescriptor),
}

#[derive(Debug, Clone)]
pub struct MovieDescriptor {
    name: String,
}

#[derive(Debug, Clone)]
pub struct EpisodeDescriptor {
    name: String,
    number: String,
}
