use derive_more::Display;
use isolang::Language;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{object::ObjectLocation, user::UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct VideoId {
    id: Uuid,
}

impl From<Uuid> for VideoId {
    fn from(value: Uuid) -> Self {
        Self { id: value }
    }
}

#[derive(Debug, Clone)]
pub struct Video {
    id: VideoId,
    created_at: OffsetDateTime,
    created_by: UserId,
    langauge: Language,
    object_location: ObjectLocation,
    is_published: bool,
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
