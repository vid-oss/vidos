use isolang::Language;

use crate::{subtitle::SubtitleId, video::VideoId};

#[derive(Debug, Clone)]
pub struct SubtitleTranslation {
    video_id: VideoId,
    subtitle_id: SubtitleId,
    langauge: Language,
    text: String,
}
