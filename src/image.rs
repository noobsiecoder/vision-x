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
    errors::{Error, ErrorCode},
    extensions::{self, Extension, ImageExtension},
    utils::{check_extension, check_path},
};
use std::{collections::HashMap, fs::File, io::BufReader};

/// Store each pixel value from the image
///
/// In this hashmap, the key contains each image coordinates corresponding to its color (value)
///
/// # Example
///
/// ```
/// # use vision_x::errors;
/// use vision_x::image::{self, Pixels};
/// use std::collections::HashMap;
///
/// # fn main() -> Result<(), errors::Error> {
/// let path = "images/png/icon.png";
/// let image_data = image::load(&path)?;
/// assert_eq!(image_data.pixels[&(20, 14)], (Some(31), Some(135), Some(148), None));
///
/// let mut pixel: Pixels = HashMap::new();
/// pixel.insert((0, 0), (Some(255), Some(0), Some(0), None));
/// assert_eq!(pixel[&(0, 0)], (Some(255), Some(0), Some(0), None));
/// # Ok(()) }
/// ```
pub type Pixels = HashMap<(u32, u32), (Option<u8>, Option<u8>, Option<u8>, Option<u8>)>;

/// Image color types corresponding to its file format
///
/// # Examples
///
/// ```
/// # use vision_x::errors;
/// use vision_x::image::{self, ColorType};
///
/// # fn main() -> Result<(), errors::Error> {
/// let path = "images/png/icon.png";
/// let image_data = image::load(&path)?;
/// assert_eq!(ColorType::PNGIndexed, image_data.color_type);
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ColorType {
    PNGRgb,
    PNGRgba,
    PNGGrayscale,
    PNGGrayscaleAlpha,
    PNGIndexed,
    JPEGRgb,
}

/// Image object to store its metadata and pixel values
///
/// # Examples
///
/// ```
/// # use vision_x::errors;
/// use vision_x::{extensions::ImageExtension, image::ColorType, image::Image};
/// use std::collections::HashMap;
///
/// # fn main() -> Result<(), errors::Error> {
/// let image_data = Image {
///     height: 32,
///     width: 32,
///     extension: ImageExtension::PNG,
///     color_type: ColorType::PNGRgb,
///     pixels: HashMap::new(),
/// };
/// # Ok(()) }
/// ```
#[derive(Debug, Clone)]
pub struct Image {
    pub height: u32,
    pub width: u32,
    pub extension: ImageExtension,
    pub color_type: ColorType,
    pub pixels: Pixels,
}

/// Read image from system and load to an `vision_x::Image` object.
///
/// This function is required to load the image file (.png | .jpg | .jpeg are only valid) and then any modification and operation can be done on the `Image` object.
///
/// # Examples
///
/// ```
/// # use vision_x::errors;
/// use vision_x::image;
///
/// # fn main() -> Result<(), errors::Error> {
/// let path = "images/png/cat.png";
/// let image_data = image::load(&path)?;
/// # Ok(()) }
/// ```
pub fn load(path: &'static str) -> Result<Image, Error> {
    // check path exists in system
    //
    // returns `Ok(false)` when path doesn not exist/invalid
    // or else returns `Err(...)` when system denies permission to read
    // else returns `Ok(true)` when path exists
    if let Ok(false) = check_path(&path) {
        return Error::custom_error(Error {
            message: format!("Error: Unable to locate path '{}'", &path),
            code: ErrorCode::PathNotFound,
        });
    } else if let Err(err) = check_path(&path) {
        return Error::custom_error(Error {
            message: format!("{}", err.to_string()),
            code: ErrorCode::IOError,
        });
    }

    // `check_path()` has returned `Ok(true)`; path is valid and exists
    // check file extension for image format
    // if the image format is not *.png | .jpg | .jpeg, throws an error
    // else, the image file is opened from the system
    let extension = check_extension(&path)?;
    let file = File::open(&path);
    // checks if the file open was successful
    //
    // if error, returns an `ErrorCode::IOError`; error while opening file
    if file.is_err() {
        return Err(Error {
            code: ErrorCode::IOError,
            message: format!("Error: {}", file.unwrap_err().to_string()),
        });
    }

    // if no error occurs in the file open operation, the `Result` object is unwrapped
    // buffer is prepared to read the imagestream data
    let file = file.unwrap();
    let mut reader = BufReader::new(file);

    // match file format extension and decode imagestream data
    //
    // throw error if any trouble extracting the data
    match extension {
        ImageExtension::PNG => {
            let image = extensions::PNGImage::extract_data(&mut reader)?;

            Ok(image)
        }
        ImageExtension::JPEG => Ok(Image {
            height: 0,
            width: 0,
            extension: ImageExtension::JPEG,
            color_type: ColorType::JPEGRgb,
            pixels: Pixels::new(),
        }),
    }
}

/// Save `Image` object as an image file
///
/// Saves the image file in the working directory
fn _save(_path: &'static str, _data: &Image) -> Result<(), Error> {
    Ok(())
}

// test module solely for image reading
#[cfg(test)]
mod image_load_tests {
    use crate::image::{self, ColorType};

    // test image extract from png file and check with the returned data
    // `/images` directory not yet commited in the repository
    // hence, test is bound to fail
    //
    // test for `vision_x::image::ColorType::PNGIndexed`
    #[test]
    fn read_png_indexed() {
        let path = "images/png/icon.png";
        let image = image::load(&path);
        println!();
        assert!(image.is_ok());
        let image_data = image.unwrap();
        assert_eq!(image_data.height, 48);
        assert_eq!(
            image_data.pixels[&(20, 14)],
            (Some(31), Some(135), Some(148), None) // teal color for the image: icon
        );
    }

    // test for `vision_x::image::ColorType::PNGGrayscale`
    #[test]
    fn read_png_grayscale() {
        let path = "images/png/basn0g01.png";
        let image = image::load(&path);
        assert!(image.is_ok());
        let image_data = image.unwrap();
        assert_eq!(image_data.height, 32);
        assert_eq!(image_data.color_type, ColorType::PNGGrayscale);
        assert_eq!(image_data.pixels.len(), 1024);
        assert_eq!(
            image_data.pixels[&(0, 0)],
            (Some(255), None, None, None) // #fff color for the image: basn0g01
        );
        assert_eq!(
            image_data.pixels[&(30, 30)],
            (Some(0), None, None, None) // #000 color for the image: basn0g01
        );
    }

    // test for `vision_x::image::ColorType::PNGRgb`
    #[test]
    fn read_png_rgb() {
        let path = "images/png/scenery.png";
        let image = image::load(&path);
        assert!(image.is_ok());
        let image_data = image.unwrap();
        assert_eq!(image_data.height, 2961);
        assert_eq!(image_data.color_type, ColorType::PNGRgb);
        assert_eq!(image_data.pixels.len(), 1974 * 2961);
        assert_eq!(
            image_data.pixels[&(12, 34)],
            (Some(171), Some(181), Some(193), None) // #ABB5C1 color for the image: scenery
        );
        assert_eq!(
            image_data.pixels[&(1655, 2000)],
            (Some(75), Some(69), Some(83), None) // #4B4553 color for the image: scenery
        );
    }

    // test for `vision_x::image::ColorType::PNGGrayscaleAlpha`
    #[test]
    fn read_png_grayscale_alpha() {
        let path = "images/png/basn4a08.png";
        let image = image::load(&path);
        assert!(image.is_ok());
        let image_data = image.unwrap();
        assert_eq!(image_data.height, 32);
        assert_eq!(image_data.color_type, ColorType::PNGGrayscaleAlpha);
        assert_eq!(image_data.pixels.len(), 32 * 32);
        assert_eq!(
            image_data.pixels[&(19, 22)],
            (Some(74), Some(156), None, None)
        );
    }
}
