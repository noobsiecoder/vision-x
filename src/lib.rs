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

//! VisionX is an Image processing library inspired from the OpenCV library
//! The project's sole focus is to develop tools and softwares specific to image processing with rust's memory safety
//! VisionX aims to provide basic image processing operations and include batteries (built-in image processing for different scenarios) to ease development and reduce time

//! To use the library:

//! ```bash
//! cargo install visionx
//!
//! // or paste inside your projects' Cargo.toml
//! // inside your Cargo.toml
//! // ..
//! [dependencies]
//! visionx = "*"
//! ```

//! Currently uses image and ndarray crate to handle image manipulation and pixel storage respectively

//! The project is still under development and any contribution at this moment is really appreciated

/// Contains all the core implementation for image storage
pub mod core;

/// Responsible for read, and write operation for an image
pub mod io;

/// Contains Result, and ErrorKind type. Responsible to throw error during runtime
pub mod errors;

/// Contains implementation of image processing tools/operations
mod imgproc;
