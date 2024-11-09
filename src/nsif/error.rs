use thiserror::Error;

#[derive(Debug, Error)]
pub enum NsifError {
    #[error("The given file is not an NSIF/NITF file")]
    FileMismatch,
}
