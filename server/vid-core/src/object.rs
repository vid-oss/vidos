use std::fmt::Display;

use derive_more::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum ObjectLocation {
    S3(S3ObjectLocation),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S3ObjectLocation {
    bucket: String,
    path: String,
}

impl Display for S3ObjectLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", &self.bucket, &self.path))
    }
}
