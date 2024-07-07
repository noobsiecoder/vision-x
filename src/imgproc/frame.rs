use ndarray::Array2;

use crate::{
    core::ImageData,
    errors::{VisionXErrorKind, VisionXResult},
};

/// Implementation for frame/image size manipulation
impl<T: Default + Copy, const N: usize> ImageData<T, N> {
    /// Create a image resized to specified dimension. Accepts width and height as `u32` respectively
    ///
    /// Uses **nearest-neighbor interpolation** as it is fast and easier to compute
    ///
    /// ## Note
    /// This can produce blocky and pixelated image, especially when upscaling
    ///
    /// # Example
    ///
    /// ```
    /// use vision_x::core::Image;
    /// use vision_x::io;
    /// # use vision_x::errors::VisionXResult;
    ///
    /// # fn main() -> VisionXResult<()> {
    /// let path = "images/jpg/lenna.jpg";
    /// let img: Image = io::read(path)?;
    /// if let Image::ImageRgb(rgb) = img {
    ///     let new_width = 512;
    ///     let new_height = 512;
    ///     let resized_img = rgb.resize(new_width, new_height);
    /// }
    /// # Ok(()) }
    /// ```
    pub fn resize(&self, width: u32, height: u32) -> Self {
        let mut resized_pixels: ndarray::ArrayBase<
            ndarray::OwnedRepr<[T; N]>,
            ndarray::Dim<[usize; 2]>,
        > = Array2::from_elem((height as usize, width as usize), [T::default(); N]);

        for y in 0..height {
            for x in 0..width {
                let old_x = (x * self.width()) / width;
                let old_y = (y * self.height()) / height;

                let pixel = self.get_pixel_at(old_x as usize, old_y as usize);
                if pixel.is_some() {
                    resized_pixels[(y as usize, x as usize)] = *pixel.unwrap();
                }
            }
        }

        ImageData::new(width, height, resized_pixels)
    }

    /// Consider only a portion of the image (region of interest). Accepts two points which will be used in cropping the original image with type: `(u32, u32)` respectively
    ///
    /// Returns `Err` if points are out of boundary
    ///
    /// # Example
    ///
    /// ```
    /// use vision_x::core::Image;
    /// use vision_x::io;
    /// # use vision_x::errors::VisionXResult;
    ///
    /// # fn main() -> VisionXResult<()> {
    /// let path = "images/jpg/lenna.jpg";
    /// let img: Image = io::read(path)?;
    /// if let Image::ImageRgb(rgb) = img {
    ///     let point1 = (100, 100);
    ///     let point2 = (200, 200);
    ///     let cropped_img = rgb.crop(point1, point2)?;
    /// }
    /// # Ok(()) }
    /// ```
    pub fn crop(&self, point1: (u32, u32), point2: (u32, u32)) -> VisionXResult<Self> {
        let dim = (*self.width(), *self.height());
        if point1.0 < point2.0 && point1.1 < point2.1 && point2.0 <= dim.0 && point2.1 <= dim.1 {
            let cropped_pixels: ndarray::ArrayBase<
                ndarray::OwnedRepr<[T; N]>,
                ndarray::Dim<[usize; 2]>,
            > = self
                .pixels()
                .slice(ndarray::s![
                    point1.1 as usize..point2.1 as usize,
                    point1.0 as usize..point2.0 as usize
                ])
                .to_owned();

            let new_width = point2.0 - point1.0;
            let new_height = point2.1 - point1.1;

            Ok(ImageData::new(new_width, new_height, cropped_pixels))
        } else {
            let err = format!("cropping as given size fails condition for values: Point1({}, {}) and Point2({}, {}) for image size ({}, {})", point1.0, point1.1, point2.0, point2.1, dim.0, dim.1);
            Err(Box::new(VisionXErrorKind::IndexOutofBound(err)))
        }
    }
}

#[cfg(test)]
mod frame_test {
    use crate::core::{Image, ImageData};
    use crate::errors::VisionXResult;
    use crate::io;

    // Resize rgb8bit image
    // tested for upscaling and downscaling
    #[test]
    fn resize() {
        let path: &str = "images/jpg/cat.jpg";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let image: Image = res.unwrap();
        if let Image::ImageRgb(rgb) = image {
            let resized_img: ImageData<u8, 3> = rgb.resize(2000, 2000);

            assert_eq!(2000, *resized_img.height());

            let path: &str = "images/test/jade_cat-resize.png";
            let img: Image = Image::ImageRgb(resized_img);
            let res: VisionXResult<()> = io::write(path, &img);
            assert!(res.is_ok());
        };
    }

    // Crop a rgb8bit image
    // Edge conditions tested
    #[test]
    fn crop() {
        let path: &str = "images/png/scenery.png";
        let res: VisionXResult<Image> = io::read(path);
        assert!(res.is_ok());

        let image: Image = res.unwrap();
        if let Image::ImageRgb(rgb) = image {
            let cropped_img: VisionXResult<ImageData<u8, 3>> = rgb.crop((1400, 512), (1900, 1600));
            assert!(cropped_img.is_ok());

            if let Ok(raw_img) = cropped_img {
                assert_eq!(1600 - 512, *raw_img.height());

                let path: &str = "images/test/jade_scenery-crop.jpg";
                let img: Image = Image::ImageRgb(raw_img);
                let res: VisionXResult<()> = io::write(path, &img);
                assert!(res.is_ok());
            }
        };
    }
}
