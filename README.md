# VisionX: A Comprehensive Image Processing Library in Rust

**🚧 Under development**

VisionX is a robust and comprehensive image processing library written in Rust. It aims to provide a wide range of functionalities similar to OpenCV, allowing developers to perform various image processing tasks efficiently and effectively with **built-in batteries (image processing tools for different scenarios/use-cases)**

## Deliverables

### Basic Image Manipulation
- Reading and Writing Images: Support for various formats like JPEG, PNG, BMP, etc.
- Resizing and Cropping: Functions to resize and crop images.
- Color Space Conversion: Conversion between different color spaces (RGB, Grayscale, HSV, etc.).

### Image Enhancement

- Histogram Equalization: Enhance image contrast.
- Brightness and Contrast Adjustment: Modify image brightness and contrast.
- Smoothing (Blurring): Gaussian blur, median blur, bilateral filter.

### Image Segmentation

- Thresholding: Global, adaptive, and Otsu's thresholding.
- Edge Detection: Sobel, Canny, Laplacian edge detectors.

### Geometric Transformations

- Affine Transformations: Translation, rotation, scaling, and shearing.
- Perspective Transformations: Warp images using perspective transformations.

### Morphological Operations

- Erosion and Dilation: Basic morphological operations.
- Opening and Closing: Noise removal and gap closing.

### Image Filtering

- Convolution and Correlation: Custom kernel filtering.
- Non-Linear Filtering: Median filter, bilateral filter.

### Utilities

- Drawing Functions: Draw lines, circles, rectangles, text on images.
- Mouse and Keyboard Event Handling: Interaction with images via GUI.

## Installation

```toml
# Add this to your Cargo.toml:
[dependencies]
vision_x = "0.1.0"
```

```rust
// Then, add the following to your crate root:
use vision_x;
```

## Usage/Examples

Here's a basic example of reading an image, converting it to grayscale, and saving the result:

```rust
use vision_x::core::Image;
use vision_x::io;

fn main() -> Result<()> {
    // Load an image from a file
    let img = io::read("path/to/image.jpg")?;

    // Convert the image to grayscale
    let gray_img = img.grayscale();

    // Save the grayscale image
    io::write("path/to/gray_image.jpg", &gray_img)?;

    // End of operation
    Ok(())
}
```

## Contributing

Contributions are always welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](/LICENSE) file for details.

## Contact

For any questions or issues, please open an issue on GitHub or contact at [abhishek.\_.sriram@outlook.com](mailto:abhishek._.sriram@outlook.com).
