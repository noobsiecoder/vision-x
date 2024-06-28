# v0.0.1-alpha (2024-06-29)

- Completely revamped code structure after issues with the return type `Image`
- Replaced `...::read(&'static str)` with `...::load(&'static str)` to fix issue and implemented for only PNG file format
- Added `vision_x::errors`, `vision_x::extenmions` and `vision_x::utils`

# v0.0.1-alpha (2024-06-25)

- Added `vision_x::image::read(path: &'static str)` utility
- Replaced `main.rs` with `lib.rs`

# v0.0.1-alpha (2024-06-22)

- Created repository
- Created README.md
- Created Cargo project using `cargo init`
- Modified .gitignore file
- Created TODO.md