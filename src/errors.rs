use core::fmt;
use std::error::Error;

///
pub type VisionXError = Box<dyn Error + 'static>;
///
pub type VisionXResult<T> = Result<T, VisionXError>;

/// enum VisionXErrorKind is a custom errorkind
#[derive(Debug)]
pub enum VisionXErrorKind {
    //
    IndexOutofBound(String),
    //
    InvalidSize(String),
    //
    InvalidColorType(String),
    //
    InvalidImageDepthSize(String),
    //
    InsufficientBufferSize(String),
}

/// Implements `fmt::Display` for our custom Errorkind
impl fmt::Display for VisionXErrorKind {
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VisionXErrorKind::IndexOutofBound(err) => write!(f, "index error at {err}"),
            VisionXErrorKind::InvalidSize(err) => write!(f, "size error while {err}"),
            VisionXErrorKind::InvalidColorType(err) => {
                write!(f, "invalid color codec used while {err}")
            }
            VisionXErrorKind::InvalidImageDepthSize(err) => {
                write!(f, "invalid color depth size is used to {}", err)
            }
            VisionXErrorKind::InsufficientBufferSize(err) => {
                write!(f, "buffer size error while {err}")
            }
        }
    }
}

/// Implements `std::error::Error` for our custom Errorkind
impl std::error::Error for VisionXErrorKind {}
