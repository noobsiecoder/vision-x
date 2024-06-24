use std::{
    borrow::Borrow,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use super::Image;

#[derive(Debug, PartialEq)]
enum ImageExt {
    // AVIF,
    // BMP,
    PNG,
    JPG,
    // TIFF,
}

fn extension(path: &'static str) -> Option<ImageExt> {
    let filename = path
        .split('/')
        .collect::<Vec<&str>>()
        .last() // get last value
        .unwrap()
        .to_owned();
    let extension = match filename
        .split('.') // from the last string, split the filename and extension
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .trim()
        .to_ascii_lowercase()
        .as_ref()
    {
        // ".avif" | ".avifs" => Some(ImageExt::AVIF),
        // "bmp" | "dib" => Some(ImageExt::BMP),
        "png" => Some(ImageExt::PNG),
        "jif" | "jpg" | "jpe" | "jpeg" | "jfif" | "jfi" => Some(ImageExt::JPG),
        // "tif" | "tiff" => Some(ImageExt::TIFF),
        _ => None,
    };

    extension
}

fn dimension(size: &usize, data: &Vec<u8>, extension: &ImageExt) -> Option<(u16, u16)> {
    match extension {
        // ImageExt::BMP => None,
        ImageExt::PNG => {
            let file_header: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10]; // signature data
            if data.len() < 8 || &data[0..8] != file_header {
                return None; // TODO: Handle Error
            }

            let mut pos = 8;
            while pos < *size {
                let chunk_length =
                    u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
                        as usize;
                pos += 4;

                let chunk_type = data[pos..pos + 4].borrow();
                pos += 4;

                if chunk_type == b"IHDR" {
                    let width = u32::from_be_bytes([
                        data[pos],
                        data[pos + 1],
                        data[pos + 2],
                        data[pos + 3],
                    ]);
                    let height = u32::from_be_bytes([
                        data[pos + 4],
                        data[pos + 5],
                        data[pos + 6],
                        data[pos + 7],
                    ]);
                    return Some((width as u16, height as u16));
                }

                pos += chunk_length + 4;
            }

            None
        }
        ImageExt::JPG => {
            let mut pos = 2;
            while pos < *size {
                if data[pos] != 0xFF {
                    return None; // TODO: Handle error
                }

                let marker = data[pos + 1];
                let segment_length = u16::from_be_bytes([data[pos + 2], data[pos + 3]]) as usize;
                if marker >= 0xC0 && marker <= 0xC2 {
                    let height = u16::from_be_bytes([data[pos + 5], data[pos + 6]]);
                    let width = u16::from_be_bytes([data[pos + 7], data[pos + 8]]);
                    return Some((width, height));
                }

                pos += segment_length + 2;
            }

            None
        } // ImageExt::TIFF => None,
    }
}

pub fn read(path: &'static str) -> Result<Option<Image>, Box<dyn Error>> {
    // TODO: Check file exists
    match Path::new(path).try_exists() {
        Ok(true) => {
            let file = File::open(&path)?;
            let mut buf_reader = BufReader::new(file);
            let mut image: Vec<u8> = Vec::new();
            buf_reader.read_to_end(&mut image)?;

            let size = image.len();
            let extension = extension(path);

            if extension == None {
                // TODO: Prepare custom Image error
                Ok(None)
            } else {
                // TODO: Handle `None` from dimension
                let dimension = dimension(&size, &image, &extension.unwrap()).unwrap();
                let image_data = Image {
                    data: image.clone(),
                    size,
                    height: dimension.0,
                    width: dimension.1,
                    dim: dimension,
                };

                Ok(Some(image_data))
            }
        }
        Ok(false) => Ok(None), // TODO: Custom error
        Err(err) => Err(Box::new(err)),
    }
}

#[cfg(test)]
mod read_image {
    use super::read;

    #[test]
    fn jpg() {
        let mut path = "./images/jpg/lenna.jpg";
        let mut _image = read(path).unwrap();
        let mut metadata = _image.unwrap();

        assert_eq!(metadata.dim, (225, 225));
        assert_eq!(metadata.size, 8017);

        path = "images/jpg/cat.jpg";
        _image = read(path).unwrap();
        metadata = _image.unwrap();

        assert_eq!(metadata.dim, (1600, 1598));
        assert_eq!(metadata.size, 279603);

        path = "images/cat.jpg";
        _image = read(path).unwrap();

        assert_eq!(_image, None);
    }

    #[test]
    fn png() {
        let mut path = "./images/png/icon.png";
        let mut _image = read(path).unwrap();
        let mut metadata = _image.unwrap();

        assert_eq!(metadata.dim, (48, 48));
        assert_eq!(metadata.size, 1543);

        path = "images/png/cat.png";
        _image = read(path).unwrap();
        metadata = _image.unwrap();

        assert_eq!(metadata.dim, (1520, 934));
        assert_eq!(metadata.size, 1909787);

        path = "images/cat.png";
        _image = read(path).unwrap();

        assert_eq!(_image, None);
    }
}
