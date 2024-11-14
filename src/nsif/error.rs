use thiserror::Error;

#[derive(Debug, Error)]
pub enum NsifError {
    #[error("The given file is not an NSIF/NITF file")]
    FileMismatch,
    #[error("The given image mode is not supported")]
    ImodeNotSupported,
    #[error("The given image compression is not supported")]
    IcNotSupported,
    #[error("The given image segment sub header is malformed")]
    ImageSegmentSubHeaderMalformed,
    #[error("The given image segment has invalid dimensions")]
    InvalidDimensions,
}
