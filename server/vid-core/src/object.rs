use derive_more::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum ObjectLocation {
    S3(S3ObjectLocation),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{bucket}:{path}")]
pub struct S3ObjectLocation {
    bucket: String,
    path: String,
}
