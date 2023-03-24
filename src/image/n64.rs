use crate::color::{intensity, r5g5b5a1};
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use png::{BitDepth, ColorType};
use std::io::{Read, Write};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    I4,
    I8,
    IA4,
    IA8,
    IA16,
    CI4,
    CI8,
    RGBA16,
    RGBA32,
}

pub struct NativeImage {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub struct PNGImage {
    data: Vec<u8>,
    color_type: ColorType,
    bit_depth: BitDepth,
}

impl NativeImage {
    pub fn read<R: Read>(
        mut reader: R,
        format: ImageFormat,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Self {
            format,
            width,
            height,
            data,
        })
    }

    pub fn as_png<W: Write>(&self, writer: W) -> Result<()> {
        let mut encoder = png::Encoder::new(writer, self.width, self.height);

        match self.format {
            ImageFormat::RGBA32 => {
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&self.data)?;
            }
            ImageFormat::RGBA16 => {
                let mut data = Vec::new();
                let mut cursor = std::io::Cursor::new(&self.data);

                while let Ok(pixel) = cursor.read_u16::<BigEndian>() {
                    data.append(&mut r5g5b5a1::to_rgba(pixel));
                }

                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
            ImageFormat::I4 => {
                let mut data = Vec::new();

                for y in 0..self.height {
                    for x in (0..self.width).step_by(2) {
                        let index = (y * self.width + x) / 2;
                        let byte = self.data[index as usize];

                        data.push(byte & 0xF0);
                        data.push((byte & 0x0F) << 4);
                    }
                }

                encoder.set_color(png::ColorType::Grayscale);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
            ImageFormat::I8 => {
                let mut data = Vec::new();

                for y in 0..self.height {
                    for x in 0..self.width {
                        let index = (y * self.width + x) as usize;
                        data.push(self.data[index]);
                    }
                }

                encoder.set_color(png::ColorType::Grayscale);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
            ImageFormat::IA4 => {
                let mut data = Vec::new();

                for y in 0..self.height {
                    for x in (0..self.width).step_by(2) {
                        let index = (y * self.width + x) / 2;
                        let byte = self.data[index as usize];

                        let source = (byte & 0xF0) >> 4;
                        let grayscale = ((source & 0x0E) >> 1) * 32;
                        let alpha = (source & 0x01) * 255;
                        data.append(&mut vec![grayscale, alpha]);

                        let source = byte & 0x0F;
                        let grayscale = ((source & 0x0E) >> 1) * 32;
                        let alpha = (source & 0x01) * 255;
                        data.append(&mut vec![grayscale, alpha]);
                    }
                }

                encoder.set_color(png::ColorType::GrayscaleAlpha);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
            ImageFormat::IA8 => {
                let mut data = Vec::new();

                for y in 0..self.height {
                    for x in 0..self.width {
                        let index = (y * self.width + x) as usize;
                        let byte = self.data[index as usize];

                        let grayscale = byte & 0xF0;
                        let alpha = (byte & 0x0F) << 4;

                        data.append(&mut vec![grayscale, alpha])
                    }
                }

                encoder.set_color(png::ColorType::GrayscaleAlpha);
                encoder.set_depth(png::BitDepth::Eight);
                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
            ImageFormat::IA16 => {
                let mut data = Vec::new();

                let mut cursor = std::io::Cursor::new(&self.data);
                while let Ok(grayscale) = cursor.read_u8() {
                    let alpha = cursor.read_u8()?;
                    data.append(&mut vec![grayscale, alpha])
                }

                encoder.set_color(png::ColorType::GrayscaleAlpha);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
            ImageFormat::CI4 => {
                let mut data = Vec::new();
                let palette: Vec<u8> = vec![2 ^ 8; 16 * 16];

                for y in 0..self.height {
                    for x in (0..self.width).step_by(2) {
                        let index = (y * self.width + x) / 2;
                        let byte = self.data[index as usize];

                        let target_index = (byte & 0xF0) >> 4;
                        data.push(target_index);

                        let target_index = byte & 0x0F;
                        data.push(target_index);
                    }
                }

                encoder.set_palette(palette);
                encoder.set_color(png::ColorType::Indexed);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
            ImageFormat::CI8 => {
                let mut data = Vec::new();
                let palette: Vec<u8> = vec![2 ^ 8; 16 * 16];

                for y in 0..self.height {
                    for x in 0..self.width {
                        let index = (y * self.width + x) as usize;
                        data.push(self.data[index]);
                    }
                }

                encoder.set_palette(palette);
                encoder.set_color(png::ColorType::Indexed);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;
            }
        }

        Ok(())
    }
}

impl PNGImage {
    pub fn read<R: Read>(reader: R) -> Result<Self> {
        let decoder = png::Decoder::new(reader);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let input_bytes = &buf[..info.buffer_size()];

        Ok(Self {
            data: input_bytes.to_vec(),
            color_type: info.color_type,
            bit_depth: info.bit_depth,
        })
    }

    pub fn as_rgba32<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.color_type != ColorType::Rgba {
            return Err(anyhow::anyhow!("Invalid color type"));
        }

        if self.bit_depth != BitDepth::Eight {
            return Err(anyhow::anyhow!("Invalid bit depth"));
        }

        writer.write_all(&self.data)?;
        Ok(())
    }

    pub fn as_rgba16<W: Write>(&self, mut writer: W) -> Result<()> {
        if self.color_type != ColorType::Rgba {
            return Err(anyhow::anyhow!("Invalid color type"));
        }

        if self.bit_depth != BitDepth::Eight {
            return Err(anyhow::anyhow!("Invalid bit depth"));
        }

        self.data.chunks_exact(4).for_each(|chunk| {
            let pixel = r5g5b5a1::from_rgba(chunk[0], chunk[1], chunk[2], chunk[3]);
            writer.write_u16::<BigEndian>(pixel).unwrap();
        });

        Ok(())
    }

    pub fn as_i4<W: Write>(&self, mut writer: W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Grayscale, BitDepth::Four) => {
                writer.write_all(&self.data)?;
            }
            (ColorType::Grayscale, BitDepth::Eight) => {
                self.data.chunks_exact(2).for_each(|chunk| {
                    let pixel = chunk[0] | chunk[1] >> 4;
                    writer.write_u8(pixel).unwrap();
                });
            }
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
                self.data.chunks_exact(4).for_each(|chunk| {
                    let pixel = chunk[0] | chunk[2] >> 4;
                    writer.write_u8(pixel).unwrap();
                });
            }
            (ColorType::Rgba, BitDepth::Eight) => {
                self.data.chunks_exact(8).for_each(|chunk| {
                    let i1 = intensity::from_rgb(chunk[0], chunk[1], chunk[2]);
                    let i2 = intensity::from_rgb(chunk[0], chunk[1], chunk[2]);
                    let pixel = i1 | i2 >> 4;
                    writer.write_u8(pixel).unwrap();
                });
            }
            (ColorType::Rgb, BitDepth::Eight) => {
                self.data.chunks_exact(6).for_each(|chunk| {
                    let i1 = intensity::from_rgb(chunk[0], chunk[1], chunk[2]);
                    let i2 = intensity::from_rgb(chunk[3], chunk[4], chunk[5]);
                    let pixel = i1 | i2 >> 4;
                    writer.write_u8(pixel).unwrap();
                });
            }
            case => {
                return Err(anyhow::anyhow!(
                    "Invalid color type or bit depth: {:?}",
                    case
                ));
            }
        }

        Ok(())
    }

    pub fn as_i8<W: Write>(&self, mut writer: W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Grayscale, BitDepth::Eight) => {
                writer.write_all(&self.data)?;
            }
            (ColorType::Grayscale, BitDepth::Four) => {
                self.data.chunks_exact(2).for_each(|chunk| {
                    let pixel = chunk[0] << 4 | chunk[1];
                    writer.write_u8(pixel).unwrap();
                });
            }
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
                self.data.chunks_exact(2).for_each(|chunk| {
                    writer.write_u8(chunk[0]).unwrap();
                });
            }
            (ColorType::Rgba, BitDepth::Eight) => {
                self.data.chunks_exact(4).for_each(|chunk| {
                    let intensity = intensity::from_rgb(chunk[0], chunk[1], chunk[2]);
                    writer.write_u8(intensity).unwrap();
                });
            }
            (ColorType::Rgb, BitDepth::Eight) => {
                self.data.chunks_exact(3).for_each(|chunk| {
                    let intensity = intensity::from_rgb(chunk[0], chunk[1], chunk[2]);
                    writer.write_u8(intensity).unwrap();
                });
            }
            case => {
                return Err(anyhow::anyhow!(
                    "Invalid color type or bit depth: {:?}",
                    case
                ));
            }
        }

        Ok(())
    }

    // ia4 consists of 4 bits: 3 bits are the intensity and one bit for the alpha
    // in our case we'll fit two ia4 pixels into one byte
    pub fn as_ia4<W: Write>(&self, mut writer: W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
                self.data.chunks_exact(4).for_each(|chunk| {
                    let high = (chunk[0] >> 5) << 1 | (chunk[1] > 127) as u8;
                    let low = (chunk[2] >> 5) << 1 | (chunk[3] > 127) as u8;

                    writer.write_u8(high << 4 | low).unwrap();
                });
            }
            case => {
                return Err(anyhow::anyhow!(
                    "Invalid color type or bit depth: {:?}",
                    case
                ));
            }
        }

        Ok(())
    }

    // ia8 consists of 8 bits: 4 bits are the intensity and the other 4 bits for the alpha
    pub fn as_ia8<W: Write>(&self, mut writer: W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
                self.data.chunks_exact(2).for_each(|chunk| {
                    writer.write_u8(chunk[0] | chunk[1] >> 4).unwrap();
                });
            }
            case => {
                return Err(anyhow::anyhow!(
                    "Invalid color type or bit depth: {:?}",
                    case
                ));
            }
        }

        Ok(())
    }

    pub fn as_ia16<W: Write>(&self, mut writer: W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
                writer.write_all(&self.data)?;
            }
            case => {
                return Err(anyhow::anyhow!(
                    "Invalid color type or bit depth: {:?}",
                    case
                ));
            }
        }

        Ok(())
    }

    pub fn as_ci4<W: Write>(&self, mut writer: W) -> Result<()> {
        if self.color_type != ColorType::Indexed {
            return Err(anyhow::anyhow!("Invalid color type: {:?}", self.color_type));
        }

        match self.bit_depth {
            BitDepth::Four => {
                writer.write_all(&self.data)?;
            }
            BitDepth::Eight => {
                self.data.chunks_exact(2).for_each(|chunk| {
                    let pixel = chunk[0] << 4 | chunk[1];
                    writer.write_u8(pixel).unwrap();
                });
            }
            case => {
                return Err(anyhow::anyhow!("Invalid bit depth: {:?}", case));
            }
        }

        Ok(())
    }

    pub fn as_ci8<W: Write>(&self, mut writer: W) -> Result<()> {
        if self.color_type != ColorType::Indexed {
            return Err(anyhow::anyhow!("Invalid color type: {:?}", self.color_type));
        }

        match self.bit_depth {
            BitDepth::Eight => {
                writer.write_all(&self.data)?;
            }
            BitDepth::Four => {
                self.data.chunks_exact(2).for_each(|chunk| {
                    let pixel = chunk[0] << 4 | chunk[1];
                    writer.write_u8(pixel).unwrap();
                });
            }
            case => {
                return Err(anyhow::anyhow!("Invalid bit depth: {:?}", case));
            }
        }

        Ok(())
    }
}
