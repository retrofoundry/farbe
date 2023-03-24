use farbe::image::n64::{ImageFormat, NativeImage, PNGImage};

#[test]
fn rgba32_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.rgba32");
    let image = NativeImage::read(bytes, ImageFormat::RGBA32, 160, 160).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.rgba32.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn rgba16_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.rgba16");
    let image = NativeImage::read(bytes, ImageFormat::RGBA16, 32, 64).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.rgba16.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn i4_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.i4");
    let image = NativeImage::read(bytes, ImageFormat::I4, 16, 16).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.i4.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn i8_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.i8");
    let image = NativeImage::read(bytes, ImageFormat::I8, 64, 64).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.i8.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn ia4_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.ia4");
    let image = NativeImage::read(bytes, ImageFormat::IA4, 128, 16).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.ia4.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn ia8_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.ia8");
    let image = NativeImage::read(bytes, ImageFormat::IA8, 144, 24).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.ia8.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn ia16_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.ia16");
    let image = NativeImage::read(bytes, ImageFormat::IA16, 64, 16).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.ia16.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn ci4_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.ci4");
    let image = NativeImage::read(bytes, ImageFormat::CI4, 48, 85).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.ci4.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

#[test]
fn ci8_to_png() {
    let bytes: &[u8] = include_bytes!("n64/test.ci8");
    let image = NativeImage::read(bytes, ImageFormat::CI8, 32, 32).unwrap();

    // write to file
    let mut output_file = std::fs::File::create("tests_output/n64/test.ci8.png").unwrap();
    image.as_png(&mut output_file).unwrap();
}

// MARK: - Back to Native

#[test]
fn png_to_rgba32() {
    let bytes: &[u8] = include_bytes!("n64/test.rgba32.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.rgba32");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_rgba32(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_rgba16() {
    let bytes: &[u8] = include_bytes!("n64/test.rgba16.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.rgba16");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_rgba16(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_i4() {
    let bytes: &[u8] = include_bytes!("n64/test.i4.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.i4");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_i4(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_i8() {
    let bytes: &[u8] = include_bytes!("n64/test.i8.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.i8");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_i8(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_ia4() {
    let bytes: &[u8] = include_bytes!("n64/test.ia4.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.ia4");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_ia4(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_ia8() {
    let bytes: &[u8] = include_bytes!("n64/test.ia8.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.ia8");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_ia8(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_ia16() {
    let bytes: &[u8] = include_bytes!("n64/test.ia16.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.ia16");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_ia16(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_ci4() {
    let bytes: &[u8] = include_bytes!("n64/test.ci4.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.ci4");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_ci4(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}

#[test]
fn png_to_ci8() {
    let bytes: &[u8] = include_bytes!("n64/test.ci8.png");
    let image = PNGImage::read(bytes).unwrap();

    // compare to original
    let original_bytes: &[u8] = include_bytes!("n64/test.ci8");
    let mut ouput: Vec<u8> = Vec::new();
    image.as_ci8(&mut ouput).unwrap();

    assert_eq!(ouput, original_bytes);
}
