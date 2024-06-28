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
    errors::{Error as ImageError, ErrorCode as ImageErrorCode},
    image::{ColorType as ImageColorType, Image, Pixels},
};
use std::{collections::HashMap, fmt::Debug, fs::File, io::BufReader};

/// Represents File Format
///
/// # Examples
///
/// ```
/// # use vision_x::errors;
/// use vision_x::extensions::ImageExtension;
///
/// # fn main() -> Result<(), errors::Error> {
/// let ext = ImageExtension::PNG;
/// assert_ne!(ext, ImageExtension::JPEG);
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ImageExtension {
    PNG,
    JPEG,
}

/// Represents PNG file format
#[derive(Debug, Clone)]
pub struct PNGImage;

/// Represents JPEG file format
#[derive(Debug, Clone)]
pub struct JPEGImage;

pub trait Extension<P> {
    /// Extract some metadata and all pixel values from the image
    ///
    /// Used for `PNGImage` and `JPEGImage`
    fn extract_data(data: P) -> Result<Image, ImageError>;
}

impl Extension<&mut BufReader<File>> for PNGImage {
    fn extract_data(buf_reader: &mut BufReader<File>) -> Result<Image, ImageError> {
        // decode png image file
        // and then read info into `reader`
        let decoder = png::Decoder::new(buf_reader);
        let reader = decoder.read_info();

        // check if read info returned error
        // else consume the `Ok(...)` value
        if reader.is_err() {
            let error = reader.err();
            if error.is_some() {
                return ImageError::custom_error(ImageError {
                    message: format!("Error: {}", error.unwrap().to_string()),
                    code: crate::errors::ErrorCode::InvalidExtension,
                });
            } else {
                // should not reach here
                return ImageError::custom_error(ImageError {
                    message: format!("Error: Error while decoding PNG file using 'png' crate"),
                    code: crate::errors::ErrorCode::InternalError,
                });
            }
        }

        // reader value is unwrapped and also image info is stored as a separate variable
        // initialize buffer value using the color info
        //
        // grayscale and grayscale-alpha output a buffer size of `sqrt(t)` for a total size of `t`
        // should return `t`; hence, square of output buffer size
        let mut reader = reader.unwrap();
        let image_info = reader.info().clone();
        let mut buf = match image_info.color_type {
            png::ColorType::Grayscale | png::ColorType::GrayscaleAlpha => {
                vec![0; reader.output_buffer_size() * reader.output_buffer_size()]
            }
            png::ColorType::Rgb | png::ColorType::Rgba | png::ColorType::Indexed => {
                vec![0; reader.output_buffer_size()]
            }
        };

        // decode the next frame into the buffer
        // if this step is skipped, pixel values would be taken incorrectly for all the colors
        // check if it returns an error
        if let Err(err) = reader.next_frame(&mut buf) {
            return Err(ImageError {
                code: ImageErrorCode::InternalError,
                message: format!("Error: {}", err.to_string()),
            });
        }

        // get bytes / pixel by matching with the color type
        let bytes_per_pixel = match image_info.color_type {
            png::ColorType::Rgb => 3,
            png::ColorType::Rgba => 4,
            png::ColorType::Grayscale => 1,
            png::ColorType::GrayscaleAlpha => 2,
            _ => 1, // For Indexed, we process differently
        };

        // using the color info, obtain each pixel values of the image
        // store pixels' coordinate and pixel values in the hashmap type: `Pixel`
        // return with `ok( Image {...} )`
        //
        // Indexed color format follows a different procedure to obtain the above values 
        match image_info.color_type {
            png::ColorType::Rgb => {
                let mut pixels: Pixels = HashMap::new();
                let mut idx: usize = 0;
                for y in 0..image_info.height {
                    for x in 0..image_info.width {
                        let chunk = &buf[idx..idx + bytes_per_pixel];
                        let (r, g, b) = (chunk[0], chunk[1], chunk[2]);
                        pixels.insert((x, y), (Some(r), Some(g), Some(b), None));

                        idx += bytes_per_pixel;
                    }
                }
                Ok(Image {
                    height: image_info.height,
                    width: image_info.width,
                    extension: ImageExtension::PNG,
                    color_type: ImageColorType::PNGRgb,
                    pixels,
                })
            }
            png::ColorType::Rgba => {
                let mut pixels: Pixels = HashMap::new();
                let mut idx: usize = 0;
                for y in 0..image_info.height {
                    for x in 0..image_info.width {
                        let chunk = &buf[idx..idx + bytes_per_pixel];
                        let (r, g, b, a) = (chunk[0], chunk[1], chunk[2], chunk[3]);
                        pixels.insert((x, y), (Some(r), Some(g), Some(b), Some(a)));

                        idx += bytes_per_pixel;
                    }
                }
                Ok(Image {
                    height: image_info.height,
                    width: image_info.width,
                    extension: ImageExtension::PNG,
                    color_type: ImageColorType::PNGRgba,
                    pixels,
                })
            }
            png::ColorType::Grayscale => {
                let mut pixels: Pixels = HashMap::new();
                let mut idx: usize = 0;
                for y in 0..image_info.height {
                    for x in 0..image_info.width {
                        // println!("GRAYSCALE: {}", idx + bytes_per_pixel);
                        let chunk = &buf[idx..idx + bytes_per_pixel];
                        let g = chunk[0];
                        pixels.insert((x, y), (Some(g), None, None, None));

                        idx += bytes_per_pixel;
                    }
                }
                Ok(Image {
                    height: image_info.height,
                    width: image_info.width,
                    extension: ImageExtension::PNG,
                    color_type: ImageColorType::PNGGrayscale,
                    pixels,
                })
            }
            png::ColorType::GrayscaleAlpha => {
                let mut pixels: Pixels = HashMap::new();
                let mut idx: usize = 0;
                for y in 0..image_info.height {
                    for x in 0..image_info.width {
                        let chunk = &buf[idx..idx + bytes_per_pixel];
                        let (g, a) = (chunk[0], chunk[1]);
                        pixels.insert((x, y), (Some(g), Some(a), None, None));

                        idx += bytes_per_pixel;
                    }
                }
                Ok(Image {
                    height: image_info.height,
                    width: image_info.width,
                    extension: ImageExtension::PNG,
                    color_type: ImageColorType::PNGGrayscaleAlpha,
                    pixels: pixels.clone(),
                })
            }
            png::ColorType::Indexed => {
                if let Some(palette) = image_info.palette.as_ref() {
                    let mut pixels: Pixels = HashMap::new();
                    let mut idx: usize = 0;
                    for y in 0..image_info.height {
                        for x in 0..image_info.width {
                            let index = buf[idx] as usize;
                            let palette_index = index as usize * 3;
                            let r = palette[palette_index];
                            let g = palette[palette_index + 1];
                            let b = palette[palette_index + 2];
                            pixels.insert((x, y), (Some(r), Some(g), Some(b), None));

                            idx += bytes_per_pixel;
                        }
                    }
                    Ok(Image {
                        height: image_info.height,
                        width: image_info.width,
                        extension: ImageExtension::PNG,
                        color_type: ImageColorType::PNGIndexed,
                        pixels: pixels.clone(),
                    })
                } else {
                    return Err(ImageError {
                        message: format!("Error: No palette found for indexed color type"),
                        code: ImageErrorCode::InvalidColorExtension,
                    });
                }
            }
        }
    }
}
