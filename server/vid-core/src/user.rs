use derive_more::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct UserId {
    id: String,
}
