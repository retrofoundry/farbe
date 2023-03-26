use std::io::{Read, Seek};
use clap::{Parser, arg, command};
use farbe::image::n64::{ImageFormat, NativeImage, PNGImage};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    format: ImageFormat,

    #[arg(long)]
    width: Option<u32>,

    #[arg(long)]
    height: Option<u32>,
}

fn main() {
    let args = Args::parse();

    // create file out of input path
    let input = std::fs::File::open(&args.input).unwrap();
    let mut reader = std::io::BufReader::new(input);

    // detect if input is png by reading the first 8 bytes
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic).unwrap();
    let is_png = magic == [137, 80, 78, 71, 13, 10, 26, 10];

    // reset reader to start
    reader.seek(std::io::SeekFrom::Start(0)).unwrap();

    // do validations
    if !is_png {
        if args.width.is_none() || args.height.is_none() {
            println!("Error: exporting to native format requires --width and --height to be set");
            return;
        }
    }
    
    // set output in case one was not given, default: append .png or .format to the input name
    let output = args.output.unwrap_or_else(|| {
        let mut output = args.input.clone();
        if is_png {
            output.push_str(".png");
        } else {
            output.push_str(&format!(".{:?}", args.format));
        }
        output
    });

    if is_png {
        let image = PNGImage::read(&mut reader).unwrap();
        let output = std::fs::File::create(output).unwrap();
        let mut writer = std::io::BufWriter::new(output);
        image.as_native(&mut writer, args.format).unwrap();
    } else {
        let image = NativeImage::read(&mut reader, args.format, args.width.unwrap(), args.height.unwrap()).unwrap();
        let output = std::fs::File::create(output).unwrap();
        let mut writer = std::io::BufWriter::new(output);
        image.as_png(&mut writer).unwrap();
    }
}
