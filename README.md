# farbe
Library for working with retro image formats

This library currently supports the following:
- converting to and from n64 formats

## Installation
Add this to your Cargo.toml:
```toml
[dependencies]
farbe = "0.1.0"
```

## How to use
```rust
use farbe::image::n64::{ImageFormat, NativeImage, PNGImage};

// convert to png
let bytes: &[u8] = include_bytes!("image.rgba32")
let image = NativeImage::read(bytes, ImageFormat::RGBA32, 160, 160).unwrap();

let mut output_file = std::fs::File::create("image.png").unwrap();
image.as_png(&mut output_file).unwrap();

// convert to n64 format
let bytes: &[u8] = include_bytes!("image.png");
let image = PNGImage::read(bytes).unwrap();

let mut output_file = std::fs::File::create("image.output.rgba32").unwrap();
image.as_rgba32(&mut output_file).unwrap();
```