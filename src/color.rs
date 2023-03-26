pub struct R5G5B5A1 {}

impl R5G5B5A1 {
    #[inline]
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> u16 {
        let r = (r / 8) as u16;
        let g = (g / 8) as u16;
        let b = (b / 8) as u16;
        let a = (a / 255) as u16;

        return (r << 11) | (g << 6) | (b << 1) | a;   
    }

    #[inline]
    pub fn to_rgba(pixel: u16) -> Vec<u8> {
        let r = ((pixel & 0xF800) >> 11) as u8;
        let g = ((pixel & 0x07C0) >> 6) as u8;
        let b = ((pixel & 0x003E) >> 1) as u8;
        let a = (pixel & 0x01) as u8;

        return vec![r * 8, g * 8, b * 8, a * 255];
    }
}

pub struct Intensity {}

impl Intensity {
    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> u8 {
        // Seen on Stack Overflow: https://stackoverflow.com/questions/687261/converting-rgb-to-grayscale-intensity/689547#comment20506224_689547
        return (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8;
    }
}
