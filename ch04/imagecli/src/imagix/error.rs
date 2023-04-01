use image::error;
use std::convert::From;
use std::{fmt, io};

#[derive(Debug)]
pub enum ImagixError {
    FileIOError(String),
    UserInputError(String),
    ImageResizeError(String),
    FromatError(String),
}

impl From<io::Error> for ImagixError {
    fn from(_error: io::Error) -> Self {
        ImagixError::FileIOError("Error in File I/O".to_string())
    }
}

impl From<error::ImageError> for ImagixError {
    fn from(_error: error::ImageError) -> Self {
        ImagixError::ImageResizeError("Error in image processing".to_string())
    }
}

impl From<io::ErrorKind> for ImagixError {
    fn from(_error: io::ErrorKind) -> Self {
        ImagixError::UserInputError("Error in use input".to_string())
    }
}

impl fmt::Display for ImagixError {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            out,
            "{}",
            ImagixError::FromatError("Error ocurred".to_string())
        )
    }
}
