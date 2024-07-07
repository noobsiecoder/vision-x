use crate::{
    core::{Image, ImageData},
    errors::{VisionXErrorKind, VisionXResult},
};

/// Implementation to convert an image into RGB, Grayscale, and HSV colorspace
impl Image {
    /// Convert pixel's color depth (16bit) to 8bit
    ///
    /// ## Note
    /// Function is used internally (private scope)
    fn downcast_8bit(pixel: u16) -> u8 {
        (pixel as f32 / u16::MAX as f32) as u8
    }

    /// Cast pixel's value from rgb to grayscale colorspace
    ///
    /// ## Note
    /// Function is used internally (private scope)
    fn rgb_to_gray(rgb: &[u8; 3]) -> [u8; 1] {
        // weighted sum formula
        // G = (0.299 * R) + (0.587 * G) + (0.114 * B)
        [(0.299 * rgb[0] as f32 + 0.587 * rgb[1] as f32 + 0.114 * rgb[2] as f32).round() as u8]
    }

    /// Cast pixel's value from rgb to hsv colorspace
    ///
    /// ## Note
    /// Function is used internally (private scope)
    fn rgb_to_hsv(rgb: &[u8; 3]) -> [f32; 3] {
        // link to formula and more on conversion:
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        //
        // r' = R / 255
        // g' = G / 255
        // b' = B / 255
        //
        // c_max = max(r', g' , b')
        // c_min = min(r', g' , b')
        //
        // delta = c_max - c_min
        //
        // h = if c_max == R {
        //  ((g' - b') mod(6)) / delta
        // } else if c_max == G {
        //  ((b' - r') + 2 / delta
        // } else if c_max == B {
        //  ((r' - g') + 4 / delta
        // } else {
        //   0.0deg
        // }
        //
        // s = if c_max == 0.0 { 0.0 } else { delta / c_max }
        // v = c_max
        let r_prime: f32 = rgb[0] as f32 / 255.0;
        let g_prime: f32 = rgb[1] as f32 / 255.0;
        let b_prime: f32 = rgb[1] as f32 / 255.0;

        let c_max: f32 = f32::max(r_prime, f32::max(g_prime, b_prime));
        let c_min: f32 = f32::min(r_prime, f32::min(g_prime, b_prime));

        let delta: f32 = c_max - c_min;
        let h: f32 = if c_max == r_prime {
            ((g_prime - b_prime) / delta) % 6.0
        } else if c_max == g_prime {
            ((b_prime - r_prime) / delta) + 2.0
        } else if c_max == b_prime {
            ((r_prime - g_prime) / delta) + 4.0
        } else {
            0.0
        };
        let s: f32 = if c_max == 0.0 { 0.0 } else { delta / c_max };
        let v: f32 = c_max;

        [h, s, v]
    }

    /// Cast pixel's value from hsv to rgb colorspace
    ///
    /// ## Note
    /// Function is used internally (private scope)
    fn hsv_to_rgb(hsv: &[f32; 3]) -> [u8; 3] {
        // link to formula and more on conversion:
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
        //
        // chroma = s * v
        // h' = h / 60deg
        //
        // x = chroma * (1 - abs(h' mod(2) - 1))
        //
        // (r', g', b') = if h >= 0.0 && h < 60.0 {
        //     (chroma, x, 0.0)
        // } else if h >= 60.0 && h < 120.0 {
        //     (x, chroma, 0.0)
        // } else if h >= 120.0 && h < 180.0 {
        //     (0.0, chroma, x)
        // } else if h >= 240.0 && h < 300.0 {
        //     (0.0, x, chroma)
        // } else if h >= 300.0 && h < 360.0 {
        //     (x, 0.0, chroma)
        // } else {
        //     (chroma, 0.0, x)
        // }
        //
        // r = r' + m * 255
        // g = g' + m * 255
        // b = b' + m * 255
        let chroma: f32 = hsv[1] * hsv[2];
        let h_prime: f32 = hsv[0] / 60.0;

        let x: f32 = chroma * (1.0 - f32::abs(h_prime % 2.0 - 1.0));
        let m = hsv[1] - chroma;

        let (r_prime, g_prime, b_prime) = if hsv[0] >= 0.0 && hsv[0] < 60.0 {
            (chroma, x, 0.0)
        } else if hsv[0] >= 60.0 && hsv[0] < 120.0 {
            (x, chroma, 0.0)
        } else if hsv[0] >= 120.0 && hsv[0] < 180.0 {
            (0.0, chroma, x)
        } else if hsv[0] >= 180.0 && hsv[0] < 240.0 {
            (0.0, x, chroma)
        } else if hsv[0] >= 240.0 && hsv[0] < 300.0 {
            (x, 0.0, chroma)
        } else if hsv[0] >= 300.0 && hsv[0] < 360.0 {
            (chroma, 0.0, x)
        } else {
            (0.0, 0.0, 0.0)
        };

        [
            (r_prime + m * 255.0).round() as u8,
            (g_prime + m * 255.0).round() as u8,
            (b_prime + m * 255.0).round() as u8,
        ]
    }

    /// Convert an image to Grayscale colorspace. Supports all type of colorspace from `Image` enum
    ///
    /// # Example
    ///
    /// ```
    /// use vision_x::io;
    /// use vision_x::core::Image;
    /// # use vision_x::errors::VisionXResult;
    ///
    /// # fn main() -> VisionXResult<()> {
    /// let path: &str = "images/png/scenery.png";
    /// let rgb_img: Image = io::read(path)?;
    /// let grayscale_img: Image = rgb_img.grayscale();
    /// # Ok(()) }
    /// ```
    pub fn grayscale(&self) -> Self {
        let grayscale_image: ImageData<u8, 1> = match self {
            Image::ImageGrayscale(grayscale) => grayscale.clone(), // expensive operation, please avoid at any cost
            Image::ImageGrayscaleAlpha(grayscale_alpha) => {
                let width: &u32 = grayscale_alpha.width();
                let height: &u32 = grayscale_alpha.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 2]>,
                    ndarray::Dim<[usize; 2]>,
                > = grayscale_alpha.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u8; 2]| [px_vec[0]]);

                ImageData::new(*width, *height, gray_pixels)
            }
            Image::ImageGrayscale16(grayscale16) => {
                // TODO: Requires dithering algorithm to account for the loss in visual quality
                let width: &u32 = grayscale16.width();
                let height: &u32 = grayscale16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = grayscale16.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels
                    .map(|px_vec: &[u16; 1]| [(px_vec[0] as f32 / 65535.0 * 255.0).round() as u8]);

                ImageData::new(*width, *height, gray_pixels)
            }
            Image::ImageGrayscaleAlpha16(grayscale_alpha16) => {
                // TODO: Requires dithering algorithm to account for the loss in visual quality
                let width: &u32 = grayscale_alpha16.width();
                let height: &u32 = grayscale_alpha16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 2]>,
                    ndarray::Dim<[usize; 2]>,
                > = grayscale_alpha16.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels
                    .map(|px_vec: &[u16; 2]| [(px_vec[0] as f32 / 65535.0 * 255.0).round() as u8]);

                ImageData::new(*width, *height, gray_pixels)
            }
            Image::ImageRgb(rgb) => {
                let width: &u32 = rgb.width();
                let height: &u32 = rgb.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgb.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u8; 3]| Self::rgb_to_gray(px_vec));

                ImageData::new(*width, *height, gray_pixels)
            }
            Image::ImageRgba(rgba) => {
                let width: &u32 = rgba.width();
                let height: &u32 = rgba.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 4]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgba.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels
                    .map(|px_vec: &[u8; 4]| Self::rgb_to_gray(&[px_vec[0], px_vec[1], px_vec[2]]));

                ImageData::new(*width, *height, gray_pixels)
            }
            Image::ImageRgb16(rgb16) => {
                let width: &u32 = rgb16.width();
                let height: &u32 = rgb16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgb16.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u16; 3]| {
                    Self::rgb_to_gray(&[
                        Self::downcast_8bit(px_vec[0]),
                        Self::downcast_8bit(px_vec[1]),
                        Self::downcast_8bit(px_vec[2]),
                    ])
                });

                ImageData::new(*width, *height, gray_pixels)
            }
            Image::ImageRgba16(rgba16) => {
                let width: &u32 = rgba16.width();
                let height: &u32 = rgba16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 4]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgba16.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u16; 4]| {
                    Self::rgb_to_gray(&[
                        Self::downcast_8bit(px_vec[0]),
                        Self::downcast_8bit(px_vec[1]),
                        Self::downcast_8bit(px_vec[2]),
                    ])
                });

                ImageData::new(*width, *height, gray_pixels)
            }
            Image::ImageHsv(hsv) => {
                let width: &u32 = hsv.width();
                let height: &u32 = hsv.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[f32; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = hsv.pixels();

                let gray_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 1]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[f32; 3]| {
                    [(0.299 * (px_vec[0] as f32 / 65535.0 * 255.0)
                        + 0.587 * (px_vec[1] as f32 / 65535.0 * 255.0)
                        + 0.114 * (px_vec[2] as f32 / 65535.0 * 255.0))
                        .round() as u8]
                });

                ImageData::new(*width, *height, gray_pixels)
            }
        };

        // return grayscale image
        Image::ImageGrayscale(grayscale_image)
    }

    /// Convert an image to RGB colorspace. Supports only RGBA, RGB16, RGBA16, and HSV colorspace
    ///
    /// # Example
    ///
    /// ```
    /// use vision_x::io;
    /// use vision_x::core::Image;
    /// # use vision_x::errors::VisionXResult;
    ///
    /// # fn main() -> VisionXResult<()> {
    /// let path: &str = "images/png/cat.png";
    /// let rgba_img: Image = io::read(path)?;
    /// let rgb_img: Image = rgba_img.rgb()?;
    /// # Ok(()) }
    /// ```
    pub fn rgb(&self) -> VisionXResult<Self> {
        match self {
            Image::ImageRgb(rgb) => {
                let width: &u32 = rgb.width();
                let height: &u32 = rgb.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgb.pixels();

                Ok(Image::ImageRgb(ImageData::new(
                    *width,
                    *height,
                    pixels.clone(),
                )))
            } // expensive operation, please avoid at any cost
            Image::ImageRgba(rgba) => {
                let width: &u32 = rgba.width();
                let height: &u32 = rgba.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 4]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgba.pixels();

                let rgb_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u8; 4]| [px_vec[0], px_vec[1], px_vec[2]]);

                Ok(Image::ImageRgb(ImageData::new(*width, *height, rgb_pixels)))
            }
            Image::ImageRgb16(rgb16) => {
                let width: &u32 = rgb16.width();
                let height: &u32 = rgb16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgb16.pixels();

                let rgb_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u16; 3]| {
                    [
                        Self::downcast_8bit(px_vec[0]),
                        Self::downcast_8bit(px_vec[1]),
                        Self::downcast_8bit(px_vec[2]),
                    ]
                });

                Ok(Image::ImageRgb(ImageData::new(*width, *height, rgb_pixels)))
            }
            Image::ImageRgba16(rgba16) => {
                let width: &u32 = rgba16.width();
                let height: &u32 = rgba16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 4]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgba16.pixels();

                let rgb_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u16; 4]| {
                    [
                        Self::downcast_8bit(px_vec[0]),
                        Self::downcast_8bit(px_vec[1]),
                        Self::downcast_8bit(px_vec[2]),
                    ]
                });

                Ok(Image::ImageRgb(ImageData::new(*width, *height, rgb_pixels)))
            }
            Image::ImageHsv(hsv) => {
                let width: &u32 = hsv.width();
                let height: &u32 = hsv.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[f32; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = hsv.pixels();

                let rgb_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[f32; 3]| Self::hsv_to_rgb(px_vec));

                Ok(Image::ImageRgb(ImageData::new(*width, *height, rgb_pixels)))
            }
            value => Err(Box::new(VisionXErrorKind::InvalidColorType(format!(
                "converting pixel value from {} to RGB colorspace",
                value.to_str()
            )))),
        }
    }

    /// Convert an image to HSV colorspace. Supports only RGB, RGBA, RGB16, and RGBA16 colorspace
    ///
    /// # Example
    ///
    /// ```
    /// use vision_x::io;
    /// use vision_x::core::Image;
    /// # use vision_x::errors::VisionXResult;
    ///
    /// # fn main() -> VisionXResult<()> {
    /// let path: &str = "images/jpg/lenna.jpg";
    /// let rgb_img: Image = io::read(path)?;
    /// let hsv_img: Image = rgb_img.hsv()?;
    /// # Ok(()) }
    /// ```
    pub fn hsv(&self) -> VisionXResult<Self> {
        match self {
            Image::ImageRgb(rgb) => {
                let width: &u32 = rgb.width();
                let height: &u32 = rgb.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgb.pixels();

                let hsv_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[f32; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u8; 3]| Self::rgb_to_hsv(px_vec));

                Ok(Image::ImageHsv(ImageData::new(*width, *height, hsv_pixels)))
            }
            Image::ImageRgba(rgba) => {
                let width: &u32 = rgba.width();
                let height: &u32 = rgba.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u8; 4]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgba.pixels();

                let hsv_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[f32; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u8; 4]| {
                    let rgb = [px_vec[0], px_vec[1], px_vec[2]];
                    Self::rgb_to_hsv(&rgb)
                });

                Ok(Image::ImageHsv(ImageData::new(*width, *height, hsv_pixels)))
            }
            Image::ImageRgb16(rgb16) => {
                let width: &u32 = rgb16.width();
                let height: &u32 = rgb16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgb16.pixels();

                let hsv_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[f32; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u16; 3]| {
                    let rgb: [u8; 3] = [
                        Self::downcast_8bit(px_vec[0]),
                        Self::downcast_8bit(px_vec[1]),
                        Self::downcast_8bit(px_vec[2]),
                    ];
                    Self::rgb_to_hsv(&rgb)
                });

                Ok(Image::ImageHsv(ImageData::new(*width, *height, hsv_pixels)))
            }
            Image::ImageRgba16(rgba16) => {
                let width: &u32 = rgba16.width();
                let height: &u32 = rgba16.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[u16; 4]>,
                    ndarray::Dim<[usize; 2]>,
                > = rgba16.pixels();

                let hsv_pixels: ndarray::ArrayBase<
                    ndarray::OwnedRepr<[f32; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = pixels.map(|px_vec: &[u16; 4]| {
                    let rgb: [u8; 3] = [
                        Self::downcast_8bit(px_vec[0]),
                        Self::downcast_8bit(px_vec[1]),
                        Self::downcast_8bit(px_vec[2]),
                    ];
                    Self::rgb_to_hsv(&rgb)
                });

                Ok(Image::ImageHsv(ImageData::new(*width, *height, hsv_pixels)))
            }
            Image::ImageHsv(hsv) => {
                let width: &u32 = hsv.width();
                let height: &u32 = hsv.height();
                let pixels: &ndarray::ArrayBase<
                    ndarray::OwnedRepr<[f32; 3]>,
                    ndarray::Dim<[usize; 2]>,
                > = hsv.pixels();

                Ok(Image::ImageHsv(ImageData::new(
                    *width,
                    *height,
                    pixels.clone(),
                )))
            } // expensive operation, please avoid at any cost
            value => Err(Box::new(VisionXErrorKind::InvalidColorType(format!(
                "converting pixel value from {} to HSV colorspace",
                value.to_str()
            )))),
        }
    }
}

#[cfg(test)]
mod color_test {
    use crate::core::Image;
    use crate::errors::VisionXResult;
    use crate::io;

    // Test all types of grayscale conversion (8/16bit)
    // write image using `io::write()`
    #[test]
    fn grayscale_conversion() {
        let path: &str = "images/png/scenery.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let rgb_img: Image = res.unwrap();
        let grayscale_img: Image = rgb_img.grayscale();

        let new_path = "images/test/jade_scenery-grayscale-conv.png";
        let res = io::write(new_path, &grayscale_img);
        assert!(res.is_ok());

        let new_path = "images/test/jade_scenery-grayscale-conv.jpg";
        let res = io::write(new_path, &grayscale_img);
        assert!(res.is_ok());

        let new_path = "images/test/jade_scenery-grayscale-conv.jpeg";
        let res = io::write(new_path, &grayscale_img);
        assert!(res.is_ok());

        let path: &str = "images/jpg/lenna.jpg";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let rgb_img: Image = res.unwrap();
        let grayscale_img: Image = rgb_img.grayscale();

        let new_path = "images/test/jade_lenna-grayscale-conv.png";
        let res = io::write(new_path, &grayscale_img);
        assert!(res.is_ok());
    }

    // Test rgb 16bit color depth to grayscale 8bit conversion
    // write image using `io::write()`
    #[test]
    fn rgb_16_to_grayscale_8() {
        let path: &str = "images/png/basn2c16.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        let grayscale_8bit: Image = raw_img.grayscale();
        let path: &str = "images/test/jade_basn2c16_8.png";
        let res = io::write(path, &grayscale_8bit);
        assert!(res.is_ok());
    }

    // Test grayscale 16bit color depth to grayscale 8bit conversion
    // write image using `io::write()`
    #[test]
    fn grayscale_16_to_grayscale_8() {
        let path: &str = "images/png/basn0g16.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let raw_img: Image = res.unwrap();
        let grayscale_8bit: Image = raw_img.grayscale();
        let path: &str = "images/test/jade_basn0g16_8.png";
        let res = io::write(path, &grayscale_8bit);
        assert!(res.is_ok());
    }
}
