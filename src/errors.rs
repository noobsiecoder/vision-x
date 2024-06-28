// MIT License

// Copyright (c) 2024 Abhishek Sriram

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Enum to indicate the errorcode during runtime
///
/// # Examples
/// ```
/// # use vision_x::errors;
/// use vision_x::{errors::ErrorCode, image};
///
/// # fn main() -> Result<(), errors::Error> {
/// let path = "images/png/xyz.png";
/// let image_data = image::load(&path);
///
/// assert_eq!(image_data.unwrap_err().code, ErrorCode::PathNotFound);
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
    PathNotFound,
    FileNotFound,
    InvalidExtension,
    InvalidColorExtension,
    IOError,
    InternalError,
}

/// Build error value if any error occurs during runtime
///
/// # Examples
/// ```
/// use vision_x::{errors::Error as ImageError, image};
///
/// fn get_image() -> Result<(), ImageError> {
///     let path = "images/png/xyz.png";
///     let image_data = image::load(&path)?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub code: ErrorCode,
}

impl Error {
    /// Returns the error object
    pub fn custom_error<T>(self) -> Result<T, Self> {
        Err(self)
    }
}
