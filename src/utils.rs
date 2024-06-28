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

use crate::{
    errors::Error as ImageError, errors::ErrorCode as ImageErrorCode, extensions::ImageExtension,
};

use core::result;
use std::{io, path::Path};

// check if path exists using native rust API
//
// returns `Result<bool>`; `Err(...)` when system denies permission to read
// else returns Ok(true) or Ok(false)
pub fn check_path(path: &str) -> io::Result<bool> {
    Path::new(&path).try_exists()
}

// check if extension is a valid file format extension
//
// returns `result::Result<ImageExtension, ImageError>`
// `Err(...)` is returned due to InvalidExtension or FileNotFound
// else `Ok(ImageExtension)` is returned
pub fn check_extension(path: &str) -> result::Result<ImageExtension, ImageError> {
    // split path by '/' and match with iterator's last element
    match path.split('/').last() {
        // check the file name and then look for its extension
        // if file name is empty (could be due to file not found or given path could be empty) throw error
        Some(file_name) => {
            // obtain the file extension
            // check if it contains some value
            // or else throws an error
            let extension = file_name.split('.').last();
            if extension.is_some() {
                // match with its corresponding format
                // return with the enum `ImageExtension`
                match extension.unwrap().to_lowercase().as_str() {
                    "png" => Ok(ImageExtension::PNG),
                    "jpg" | "jpeg" => Ok(ImageExtension::JPEG),
                    value => ImageError::custom_error(ImageError {
                        code: ImageErrorCode::InvalidExtension,
                        message: format!("Error: '{}' is an unrecognized extension", &value),
                    }),
                }
            } else {
                ImageError::custom_error(ImageError {
                    code: ImageErrorCode::InvalidExtension,
                    message: format!("Error: extension not provided"),
                })
            }
        }
        None => ImageError::custom_error(ImageError {
            code: ImageErrorCode::FileNotFound,
            message: format!("Error: Unable to locate file at '{}'", &path),
        }),
    }
}

#[cfg(test)]
mod util_test {
    use super::check_path;

    // this tests' sole purpose is for checking the path
    #[test]
    fn path() {
        let path = "images/png/lenna.png"; // path not found - return Ok(false)
        let result = check_path(&path);
        assert!(result.is_ok());
        assert!(!result.unwrap());

        let path = "images/png/cat.png"; // path found - return Ok(true)
        let result = check_path(&path);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
