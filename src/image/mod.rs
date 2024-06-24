use std::fmt::Debug;

mod read;

#[derive(Debug, PartialEq)]
pub struct Image {
    data: Vec<u8>,
    size: usize,
    height: u16,
    width: u16,
    dim: (u16, u16),
}
