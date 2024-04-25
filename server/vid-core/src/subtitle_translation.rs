use isolang::Language;

use crate::subtitle::SubtitleId;

#[derive(Debug, Clone)]
pub struct SubtitleTranslation {
    subtitle_id: SubtitleId,
    langauge: Language,
    text: String,
}
