use core::fmt;
use std::error::Error;

/// Type alias for std::error::Error
pub type VisionXError = Box<dyn Error + 'static>;
/// Type alias for Result<T, E>
pub type VisionXResult<T> = Result<T, VisionXError>;

/// enum VisionXErrorKind is a custom errorkind
#[derive(Debug)]
pub enum VisionXErrorKind {
    /// Used to indicate when array index is overflowed, in cases where array is smaller than the required iterator size
    IndexOutofBound(String),
    /// Size error denotes error for image's frame size
    InvalidSize(String),
    /// Used while performing any operation when encountering an invalid color code for the operation
    InvalidColorType(String),
    /// Used when the color depth of an image is invalid for an operation
    InvalidImageDepthSize(String),
    // Used when buffer space is small to accomodate a strem of bytes
    InsufficientBufferSize(String),
}

/// Implements `fmt::Display` for our custom Errorkind
impl fmt::Display for VisionXErrorKind {
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
