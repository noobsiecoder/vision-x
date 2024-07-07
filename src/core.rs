use crate::errors::{VisionXErrorKind, VisionXResult};

use ndarray::Array2;

/// Store pixel values of an image in HSV colorspace
pub type Hsv = ImageData<f32, 3>;
/// Store pixel values of a grayscale image with an 8bit color depth
pub type Grayscale = ImageData<u8, 1>;
/// Store pixel values of a grayscale + alpha channel image with an 8bit color depth
pub type GrayscaleAlpha = ImageData<u8, 2>;
/// Store pixel values of an image in RGB colorspace with an 8bit color depth
pub type Rgb = ImageData<u8, 3>;
/// Store pixel values of an image in RGBA colorspace with an 8bit color depth
pub type Rgba = ImageData<u8, 4>;
/// Store pixel values of a grayscale image with 16bit color depth
pub type Grayscale16 = ImageData<u16, 1>;
/// Store pixel values of a grayscale + alpha channel image with 16bit color depth
pub type GrayscaleAlpha16 = ImageData<u16, 2>;
/// Store pixel values of an image in RGB colorspace with 16bit color depth
pub type Rgb16 = ImageData<u16, 3>;
/// Store pixel values of an image in RGBA colorspace with 16bit color depth
pub type Rgba16 = ImageData<u16, 4>;

/// `Image` represents a set of colors available in the image processing library. The supported color spaces are: **Rgb, Rgba, Grayscale, GrayscaleAlpha, Hsv**
///
/// This enum is the fundemantal block to represent an image. It holds the image data required to perfom any operation on the image
///
/// # Example
///
/// ```
/// use vision_x::core::{Image, ImageData};
/// use ndarray::Array2;
///
/// # fn main() {
/// let width: usize = 128;
/// let height: usize = 128;
///
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
/// let img = Image::ImageGrayscale(ImageData::new(width as u32, height as u32, pixels));
/// # }
/// ```
#[derive(Debug, Clone)]
pub enum Image {
    /// Represents a grayscale image with an 8bit color depth
    ImageGrayscale(Grayscale),
    /// Represents a grayscale + alpha channel image with an 8bit color depth
    ImageGrayscaleAlpha(GrayscaleAlpha),
    /// Represents an image in RGB colorspace with an 8bit color depth
    ImageRgb(Rgb),
    /// Represents an image in RGBA colorspace with an 8bit color depth
    ImageRgba(Rgba),
    /// Represents a grayscale image with an 16bit color depth
    ImageGrayscale16(Grayscale16),
    /// Represents a grayscale + alpha channel image with an 8bit color depth
    ImageGrayscaleAlpha16(GrayscaleAlpha16),
    /// Represents an image in RGB colorspace with 16bit color depth
    ImageRgb16(Rgb16),
    /// Represents an image in RGBA colorspace with 16bit color depth
    ImageRgba16(Rgba16),
    /// Represents an image in the HSV colorspace
    ///
    /// Note: Cannot be used in `io::write()`
    ImageHsv(Hsv),
}

/// Core implementation for enum `Image`
impl Image {
    /// converts enum values to string
    ///
    /// # Example
    ///
    /// ```
    /// use vision_x::core::{Image, ImageData};
    /// use ndarray::Array2;
    ///
    /// # fn main() {
    /// let width: usize = 128;
    /// let height: usize = 128;
    ///
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
    /// let img = Image::ImageGrayscale(ImageData::new(width as u32, height as u32, pixels));
    /// assert_eq!(img.to_str(), "grayscale");
    /// # }
    /// ```
    pub fn to_str(&self) -> &'static str {
        match self {
            Image::ImageGrayscale(_) => "grayscale",
            Image::ImageGrayscaleAlpha(_) => "grayscale_alpha",
            Image::ImageGrayscale16(_) => "grayscale16",
            Image::ImageGrayscaleAlpha16(_) => "grayscale_alpha16",
            Image::ImageRgb(_) => "rgb",
            Image::ImageRgb16(_) => "rgb16",
            Image::ImageRgba(_) => "rgba",
            Image::ImageRgba16(_) => "rgba16",
            Image::ImageHsv(_) => "hsv",
        }
    }
}

// TODO: Create an Nd array specific to image pixel
/// Type alias for ndarray::Array2<T; N>
pub type PixelNdArray<T, const N: usize> = Array2<[T; N]>;

/// `ImageData` represents the height, width and pixel values of an image
///
/// This struct is wrapped inside the `Image` enum
/// ```
#[derive(Debug, Clone)]
pub struct ImageData<T, const N: usize> {
    height: u32,
    width: u32,
    pixels: PixelNdArray<T, N>,
}

/// Core implementation of struct `ImageData`
impl<T: Default + Copy, const N: usize> ImageData<T, N> {
    /// Creates a new `ImageData` object. Accepts width, height and pixels (type: `ndarray::Array2<_>`)
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
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
    /// let raw_img = ImageData::new(width as u32, height as u32, pixels);
    /// # }
    /// ```
    pub fn new(width: u32, height: u32, pixels: PixelNdArray<T, N>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Takes the pixel values from the object and flattens from Nd to 1d array
    ///
    /// Used in bufferstream to write into an image file
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
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
    /// let raw_img = ImageData::new(width as u32, height as u32, pixels);
    /// let buf_pixels = raw_img.flatten_pixels();
    /// # }
    /// ```
    pub fn flatten_pixels(&self) -> Vec<T> {
        let mut values = Vec::new();
        self.pixels.for_each(|px_vec| {
            for px in px_vec {
                values.push(*px);
            }
        });

        values
    }

    // Getters

    /// Returns the height of the image in `&u32`
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
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
    /// let raw_img = ImageData::new(width as u32, height as u32, pixels);
    /// assert_eq!(*raw_img.height(), height as u32);
    /// # }
    /// ```
    pub fn height(&self) -> &u32 {
        &self.height
    }

    /// Returns the width of the image in `&u32`
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
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
    /// let raw_img = ImageData::new(width as u32, height as u32, pixels);
    /// assert_eq!(*raw_img.width(), width as u32);
    /// # }
    /// ```
    pub fn width(&self) -> &u32 {
        &self.width
    }

    /// Returns all the pixel values of the image in `&PixelNdArray<T, N>`
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
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
    /// let raw_img = ImageData::new(width as u32, height as u32, pixels.clone());
    /// assert_eq!(*raw_img.pixels(), pixels);
    /// # }
    /// ```
    pub fn pixels(&self) -> &PixelNdArray<T, N> {
        &self.pixels
    }

    // Setters

    /// Set height of the image. Requires a mutable object of `ImageData` and accepts height in `u32`
    ///
    /// # Example
    ///
    /// ```
    /// # use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
    /// # let width: usize = 128;
    /// # let height: usize = 128;
    /// # let mut pixels = Array2::from_elem((height, width), [0u8; 1]);
    /// # for y in 0..height {
    /// #     for x in 0..width {
    /// #         if x < width / 2 {
    /// #             pixels[(x, y)] = [255; 1];
    /// #         } else {
    /// #             pixels[(x, y)] = [0; 1];
    /// #         }
    /// #     }
    /// # }
    /// #
    /// // ..
    /// let mut raw_img = ImageData::new(width as u32, height as u32, pixels);
    /// raw_img.set_height(120);
    /// # }
    /// ```
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    /// Set width of the image. Requires a mutable object of `ImageData` and accepts width in `u32`
    ///
    /// # Example
    ///
    /// ```
    /// # use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
    /// # let width: usize = 128;
    /// # let height: usize = 128;
    /// # let mut pixels = Array2::from_elem((height, width), [0u8; 1]);
    /// # for y in 0..height {
    /// #     for x in 0..width {
    /// #         if x < width / 2 {
    /// #             pixels[(x, y)] = [255; 1];
    /// #         } else {
    /// #             pixels[(x, y)] = [0; 1];
    /// #         }
    /// #     }
    /// # }
    /// #
    /// // ..
    /// let mut raw_img = ImageData::new(width as u32, height as u32, pixels);
    /// raw_img.set_width(120);
    /// # }
    /// ```
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /// Set pixel values for the image. Requires a mutable object of `ImageData` and accepts width in `PixelNdArray<T, N>`
    ///
    /// # Example
    ///
    /// ```
    /// # use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
    /// # let width: usize = 128;
    /// # let height: usize = 128;
    /// # let mut pixels = Array2::from_elem((height, width), [0u8; 1]);
    /// # for y in 0..height {
    /// #     for x in 0..width {
    /// #         if x < width / 2 {
    /// #             pixels[(x, y)] = [255; 1];
    /// #         } else {
    /// #             pixels[(x, y)] = [0; 1];
    /// #         }
    /// #     }
    /// # }
    /// #
    /// // ..
    /// let mut raw_img = ImageData::new(width as u32, height as u32, pixels.clone());
    /// let mut new_pixels = Array2::from_elem((height, width), [0u8; 1]);
    /// for y in 0..height {
    ///     for x in 0..width {
    ///         if x < height / 2 {
    ///             pixels[(x, y)] = [255; 1];
    ///         } else {
    ///             pixels[(x, y)] = [0; 1];
    ///         }
    ///     }
    /// }
    ///
    /// raw_img.set_pixels(new_pixels);
    /// # }
    /// ```
    pub fn set_pixels(&mut self, pixels: PixelNdArray<T, N>) {
        self.pixels = pixels;
    }

    // pixel manipulators

    /// Get pixel values at coordinate (x, y). Returns `None` if (x, y) is not within the image's dimension
    ///
    /// # Example
    ///
    /// ```
    /// # use ndarray::Array2;
    /// use vision_x::core::ImageData;
    ///
    /// # fn main() {
    /// # let width: usize = 128;
    /// # let height: usize = 128;
    /// # let mut pixels = Array2::from_elem((height, width), [0u8; 1]);
    /// # for y in 0..height {
    /// #     for x in 0..width {
    /// #         if x < width / 2 {
    /// #             pixels[(x, y)] = [255; 1];
    /// #         } else {
    /// #             pixels[(x, y)] = [0; 1];
    /// #         }
    /// #     }
    /// # }
    /// #
    /// // ..
    /// let mut raw_img = ImageData::new(width as u32, height as u32, pixels.clone());
    /// let pixel = raw_img.get_pixel_at(12, 12);
    /// assert_eq!(pixel, Some(&[255; 1]));
    /// # }
    /// ```
    pub fn get_pixel_at(&self, x: usize, y: usize) -> Option<&[T; N]> {
        if x < self.width as usize && y < self.height as usize {
            return Some(&self.pixels[(y, x)]);
        }

        None
    }

    /// Set pixel values at coordinate (x, y). Returns `Err` if (x, y) is not within the image's dimension
    ///
    /// # Example
    ///
    /// ```
    /// # use ndarray::Array2;
    /// use vision_x::core::ImageData;
    /// # use vision_x::errors::VisionXResult;
    ///
    /// # fn main() -> VisionXResult<()> {
    /// # let width: usize = 128;
    /// # let height: usize = 128;
    /// # let mut pixels = Array2::from_elem((height, width), [0u8; 1]);
    /// # for y in 0..height {
    /// #     for x in 0..width {
    /// #         if x < width / 2 {
    /// #             pixels[(x, y)] = [255; 1];
    /// #         } else {
    /// #             pixels[(x, y)] = [0; 1];
    /// #         }
    /// #     }
    /// # }
    /// #
    /// // ..
    /// let mut raw_img = ImageData::new(width as u32, height as u32, pixels.clone());
    /// raw_img.set_pixel_at(12, 12, [128; 1])?;
    /// # Ok(()) }
    /// ```
    pub fn set_pixel_at(&mut self, x: usize, y: usize, value: [T; N]) -> VisionXResult<()> {
        if x < self.width as usize && y < self.height as usize {
            self.pixels[(y, x)] = value;

            return Ok(());
        }

        //
        let err = format!(
            "({}, {}) for size ({}, {})",
            &x, &y, &self.width, &self.height
        );
        Err(Box::new(VisionXErrorKind::IndexOutofBound(err)))
    }
}

#[cfg(test)]
mod core_test {
    use crate::core::{Image, ImageData};
    use crate::io;
    use ndarray::{Array2, ArrayBase};

    // Focuses on pixel creation using ndarray::Array<_> and `io::write()`
    #[test]
    fn test_pixel_placement() {
        let width: usize = 128;
        let height: usize = 128;

        let mut pixels: ArrayBase<ndarray::OwnedRepr<[u8; 1]>, ndarray::Dim<[usize; 2]>> =
            Array2::from_elem((height, width), [0u8; 1]);
        for y in 0..height {
            for x in 0..width {
                if x < width / 2 {
                    pixels[(x, y)] = [255; 1];
                } else {
                    pixels[(x, y)] = [0; 1];
                }
            }
        }

        let img = Image::ImageGrayscale(ImageData::new(width as u32, height as u32, pixels));
        let path = "images/test/jade_grayscale-fun1.png";
        let res = io::write(path, &img);
        assert!(res.is_ok());
    }
}
