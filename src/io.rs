use crate::{
    core::{Image, ImageData},
    errors::{VisionXErrorKind, VisionXResult},
};

use image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba};
use ndarray::Array2;
use std::{path::Path, usize};

/// Reads an image file. Returns a result with an `Image` enum containing raw data like pixels in `ImageData` struct wrapped inside the enum
///
/// Returns `Err` if path/file is not found or if any error occurs while read operation
///
/// # Example
///
/// ```
/// use vision_x::io;
/// # use vision_x::errors::VisionXResult;
///
/// # fn main() -> VisionXResult<()> {
/// let path = "images/jpg/lenna.jpg";
/// let img = io::read(path)?;
/// # Ok(()) }
/// ```
pub fn read<P: AsRef<Path>>(path: P) -> VisionXResult<Image> {
    let raw_image = image::open(path)?;
    match &raw_image {
        DynamicImage::ImageLuma8(grayscale) => {
            let (width, height) = grayscale.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u8; 1]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u8; 1]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&Luma<u8>> = grayscale.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img = ImageData::new(width, height, pixels);
            Ok(Image::ImageGrayscale(img))
        }
        DynamicImage::ImageLumaA8(grayscale_alpha) => {
            let (width, height) = grayscale_alpha.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u8; 2]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u8; 2]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&LumaA<u8>> = grayscale_alpha.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img: ImageData<u8, 2> = ImageData::new(width, height, pixels);
            Ok(Image::ImageGrayscaleAlpha(img))
        }
        DynamicImage::ImageLuma16(grayscale16) => {
            let (width, height) = grayscale16.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u16; 1]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u16; 1]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&Luma<u16>> = grayscale16.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img: ImageData<u16, 1> = ImageData::new(width, height, pixels);
            Ok(Image::ImageGrayscale16(img))
        }
        DynamicImage::ImageLumaA16(grayscale_alpha16) => {
            let (width, height) = grayscale_alpha16.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u16; 2]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u16; 2]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&LumaA<u16>> = grayscale_alpha16.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img: ImageData<u16, 2> = ImageData::new(width, height, pixels);
            Ok(Image::ImageGrayscaleAlpha16(img))
        }
        DynamicImage::ImageRgb8(rgb) => {
            let (width, height) = rgb.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u8; 3]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u8; 3]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&Rgb<u8>> = rgb.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img: ImageData<u8, 3> = ImageData::new(width, height, pixels);
            Ok(Image::ImageRgb(img))
        }
        DynamicImage::ImageRgba8(rgba) => {
            let (width, height) = rgba.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u8; 4]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u8; 4]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&Rgba<u8>> = rgba.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img: ImageData<u8, 4> = ImageData::new(width, height, pixels);
            Ok(Image::ImageRgba(img))
        }
        DynamicImage::ImageRgb16(rgb16) => {
            let (width, height) = rgb16.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u16; 3]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u16; 3]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&Rgb<u16>> = rgb16.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img: ImageData<u16, 3> = ImageData::new(width, height, pixels);
            Ok(Image::ImageRgb16(img))
        }
        DynamicImage::ImageRgba16(rgba16) => {
            let (width, height) = rgba16.dimensions();
            let mut pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[u16; 4]>,
                ndarray::Dim<[usize; 2]>,
            > = Array2::from_elem((height as usize, width as usize), [0u16; 4]);

            for y in 0..height {
                for x in 0..width {
                    let value: Option<&Rgba<u16>> = rgba16.get_pixel_checked(x, y);
                    if value.is_some() {
                        pixels[(y as usize, x as usize)] = value.unwrap().0;
                    }
                }
            }

            let img: ImageData<u16, 4> = ImageData::new(width, height, pixels);
            Ok(Image::ImageRgba16(img))
        }
        _ => Err(Box::new(VisionXErrorKind::InvalidImageDepthSize(
            "read image".to_string(),
        ))),
    }
}

/// Writes a bufferstream into a image file.
///
/// Returns `Ok` if write is successful, else, returns `Err` if path/file is not found or if any error occurs while write operation
///
/// # Example
///
/// ```
/// use ndarray::Array2;
/// use vision_x::core::{Image, ImageData};
/// use vision_x::io;
/// # use vision_x::errors::VisionXResult;
///
/// # fn main() -> VisionXResult<()> {
/// let width: usize = 128;
/// let height: usize = 128;
/// let mut pixels = Array2::from_elem((height, width), [0u8; 1]);
/// for y in 0..height {
///     for x in 0..width {
///         if x < width / 2 {
///             pixels[(x, y)] = [255; 1];
///         } else {
///             pixels[(x, y)] = [0; 1];
///         }
///     }
/// }
///
/// let new_path = "images/test/jade_write-doctest.jpg";
/// let img = Image::ImageGrayscale(ImageData::new(width as u32, height as u32, pixels));
/// io::write(new_path, &img)?;
/// # Ok(()) }
/// ```
pub fn write<P: AsRef<Path>>(path: P, img: &Image) -> VisionXResult<()> {
    match img {
        Image::ImageGrayscale(gray_img) => {
            let pixels_vec: Vec<u8> = gray_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<Luma<u8>, Vec<u8>>> =
                ImageBuffer::<Luma<u8>, Vec<u8>>::from_vec(
                    *gray_img.width(),
                    *gray_img.height(),
                    pixels_vec.to_vec(),
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<Luma<u8>, Vec<u8>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        Image::ImageGrayscaleAlpha(gray_alpha_img) => {
            let pixels_vec: Vec<u8> = gray_alpha_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<LumaA<u8>, Vec<u8>>> =
                ImageBuffer::<LumaA<u8>, Vec<u8>>::from_vec(
                    *gray_alpha_img.width(),
                    *gray_alpha_img.height(),
                    pixels_vec,
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<LumaA<u8>, Vec<u8>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        Image::ImageRgb(rgb_img) => {
            let pixels_vec: Vec<u8> = rgb_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<Rgb<u8>, Vec<u8>>> =
                ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(
                    *rgb_img.width(),
                    *rgb_img.height(),
                    pixels_vec,
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        Image::ImageRgba(rgba_img) => {
            let pixels_vec: Vec<u8> = rgba_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<Rgba<u8>, Vec<u8>>> =
                ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(
                    *rgba_img.width(),
                    *rgba_img.height(),
                    pixels_vec,
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        Image::ImageGrayscale16(gray16_img) => {
            let pixels_vec: Vec<u16> = gray16_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<Luma<u16>, Vec<u16>>> =
                ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(
                    *gray16_img.width(),
                    *gray16_img.height(),
                    pixels_vec,
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<Luma<u16>, Vec<u16>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        Image::ImageGrayscaleAlpha16(gray_alpha16_img) => {
            let pixels_vec: Vec<u16> = gray_alpha16_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<LumaA<u16>, Vec<u16>>> =
                ImageBuffer::<LumaA<u16>, Vec<u16>>::from_vec(
                    *gray_alpha16_img.width(),
                    *gray_alpha16_img.height(),
                    pixels_vec,
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<LumaA<u16>, Vec<u16>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        Image::ImageRgb16(rgb16_img) => {
            let pixels_vec: Vec<u16> = rgb16_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<Rgb<u16>, Vec<u16>>> =
                ImageBuffer::<Rgb<u16>, Vec<u16>>::from_vec(
                    *rgb16_img.width(),
                    *rgb16_img.height(),
                    pixels_vec,
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<Rgb<u16>, Vec<u16>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        Image::ImageRgba16(rgba16_img) => {
            let pixels_vec: Vec<u16> = rgba16_img.flatten_pixels();
            let buffer_option: Option<ImageBuffer<Rgba<u16>, Vec<u16>>> =
                ImageBuffer::<Rgba<u16>, Vec<u16>>::from_vec(
                    *rgba16_img.width(),
                    *rgba16_img.height(),
                    pixels_vec,
                );

            if buffer_option.is_some() {
                let buffer: ImageBuffer<Rgba<u16>, Vec<u16>> = buffer_option.unwrap();
                buffer.save(path)?;
            } else {
                return Err(Box::new(VisionXErrorKind::InsufficientBufferSize(
                    "writing image data to file".to_string(),
                )));
            }

            Ok(())
        }
        value => Err(Box::new(VisionXErrorKind::InvalidColorType(format!(
            "writing {} image to file",
            value.to_str()
        )))),
    }
}

#[cfg(test)]
mod read_image_test {

    // Read grayscale8bit image
    #[test]
    fn grayscale() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/png/basn0g01.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        if let Image::ImageGrayscale(grayscale) = raw_img {
            for x in 0..*grayscale.width() as usize {
                let pixel_opt: Option<&[u8; 1]> = grayscale.get_pixel_at(x, 0);
                if pixel_opt.is_some() {
                    if x == (*grayscale.width() - 1) as usize {
                        assert_eq!(pixel_opt.unwrap(), &[0u8]);
                    } else {
                        assert_eq!(pixel_opt.unwrap(), &[255u8]);
                    }
                }
            }

            for y in 0..*grayscale.height() as usize {
                let pixel_opt: Option<&[u8; 1]> = grayscale.get_pixel_at(15, y);
                if pixel_opt.is_some() {
                    if y >= (*grayscale.height() / 2) as usize {
                        assert_eq!(pixel_opt.unwrap(), &[0u8]);
                    } else {
                        assert_eq!(pixel_opt.unwrap(), &[255u8]);
                    }
                }
            }
        }
    }

    // Read grayscale + alpha channel 8bit image
    #[test]
    fn grayscale_alpha() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/png/basn4a08.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        if let Image::ImageGrayscaleAlpha(grayscale_alpha) = raw_img {
            let vec: [u8; 32] = [
                0, 8, 16, 24, 32, 41, 49, 57, 65, 74, 82, 90, 98, 106, 115, 123, 131, 139, 148,
                156, 164, 172, 180, 189, 197, 205, 213, 222, 230, 238, 246, 255,
            ];

            for x in 0..*grayscale_alpha.width() as usize {
                let pixel_opt: Option<&[u8; 2]> =
                    grayscale_alpha.get_pixel_at(x, (*grayscale_alpha.height() - 1) as usize);
                if pixel_opt.is_some() {
                    assert_eq!(pixel_opt.unwrap(), &[0u8, vec[x]]);
                }
            }
        }
    }

    // Read rgb8bit image
    #[test]
    fn rgb() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/png/scenery.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        if let Image::ImageRgb(rgb) = raw_img {
            let vec: [[u8; 3]; 10] = [
                [82, 38, 37], // 960, 1480
                [74, 34, 35], // 961, 1482
                [62, 33, 35], // 962, 1484
                [63, 42, 49], // 963, 1486
                [40, 43, 50], // 964, 1488
                [25, 28, 37], // 965, 1490
                [36, 40, 52], // 966, 1492
                [37, 43, 57], // 967, 1494
                [34, 41, 57], // 968, 1496
                [32, 39, 58], // 969, 1498
            ];
            for x in 960..970 {
                let i = x - 960;
                let pixel_opt: Option<&[u8; 3]> = rgb.get_pixel_at(x, 1480 + (i * 2));
                if pixel_opt.is_some() {
                    assert_eq!(pixel_opt.unwrap(), &vec[i]);
                }
            }
        }
    }

    // Read rgba8bit image
    #[test]
    fn rgba() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/png/cat.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        if let Image::ImageRgba(rgba) = raw_img {
            let vec: [[u8; 4]; 10] = [
                [130, 64, 20, 255], // 1364, 444
                [127, 62, 22, 255], // 1365, 446
                [130, 64, 22, 255], // 1366, 448
                [129, 62, 19, 255], // 1367, 450
                [127, 64, 26, 255], // 1368, 452
                [130, 66, 24, 255], // 1369, 454
                [133, 70, 34, 255], // 1370, 456
                [144, 80, 42, 255], // 1371, 458
                [148, 86, 41, 255], // 1372, 460
                [136, 84, 47, 255], // 1373, 462
            ];
            for x in 1364..1374 {
                let i = x - 1364;
                let pixel_opt: Option<&[u8; 4]> = rgba.get_pixel_at(x, 444 + (i * 2));
                if pixel_opt.is_some() {
                    assert_eq!(pixel_opt.unwrap(), &vec[i]);
                }
            }

            let pixel_opt: Option<&[u8; 4]> = rgba.get_pixel_at(21, 23);
            if pixel_opt.is_some() {
                assert_eq!(pixel_opt.unwrap(), &[255, 255, 255, 0]);
            }
        }
    }
}

//
#[cfg(test)]
mod write_image_test {

    // Write a grayscale8bit image
    // File format is changed while writing new image
    #[test]
    fn grayscale() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/png/basn0g01.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        let path: &str = "images/test/jade_basn0g01.jpg";
        let res = io::write(path, &raw_img);
        assert!(res.is_ok());
    }

    // Write a grayscale + alpha 8bit image
    #[test]
    fn grayscale_alpha() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/png/basn4a08.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        let path: &str = "images/test/jade_basn4a08.png";
        let res = io::write(path, &raw_img);
        assert!(res.is_ok());
    }

    // Write a rgb8bit image
    // File format is changed while writing new image
    // TODO: check why .tiff is slow and doesn't generate/write
    #[test]
    fn rgb() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/jpg/cat.jpg";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        let path: &str = "images/test/jade_catboy2.png";
        let res = io::write(path, &raw_img);
        assert!(res.is_ok());

        let path: &str = "images/test/jade_catboy2.bmp";
        let res = io::write(path, &raw_img);
        assert!(res.is_ok());
    }

    // Write a rgba8bit image
    #[test]
    fn rgba() {
        use crate::core::Image;
        use crate::errors::VisionXResult;
        use crate::io;

        let path: &str = "images/png/cat.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        let path: &str = "images/test/jade_catboy.png";
        let res = io::write(path, &raw_img);
        assert!(res.is_ok());
    }
}
