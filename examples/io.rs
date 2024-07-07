//! Run from project directory using
//! cargo run --example io
extern crate vision_x;

use vision_x::core::Image;
use vision_x::errors::VisionXResult;
use vision_x::io;

/// Illustrates a read and write operation
/// Before writing the image to a new path, the image is converted to grayscale colorspace
fn main() -> VisionXResult<()> {
    let path: &str = "images/jpg/cat.jpg";
    let raw_img: Image = io::read(path)?;
    let grayscale_img: Image = raw_img.grayscale();

    let new_path: &str = "images/test/jaded_cat-grayscale.jpg";
    io::write(new_path, &grayscale_img)?;

    Ok(())
}
