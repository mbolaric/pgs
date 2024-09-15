mod helpers;

use std::{fs::File, io::{Cursor, Write}};

use log::error;
use clap::Parser;
use tiff::{encoder::{colortype, TiffEncoder}, tags};

use pgs_parse::{PgsDisplaySet, PgsDisplaySetState, PgsParser, Result};

use crate::helpers::init_logging;

pub fn num_to_bytes(dest: &mut [u8], num: u64, length: usize) {
    for i in 0..length {
        dest[i] = ((num >> (i * 8)) & 0xFF) as u8;
    }
}

pub fn get_tiff_stream(ds: &PgsDisplaySet, gray: bool) -> Result<Cursor<Vec<u8>>> {
    let samples = 3;

    let ods = ds.ods.as_ref().unwrap();
    let width = (ods.width) as usize;
    let height = (ods.height) as usize;

    // Decode RLE into pixels array
    let pixels = ds.get_decoded_image(gray)?;

    let mut buffer: Vec<u8> = vec![0; width * samples as usize];
    let mut temp: Vec<u8> = vec![0; width * samples as usize];
    let mut image_buffer: Vec<u8> = vec![0; width * height * samples as usize];

    for i in 0..height {
        for j in 0..width {
            num_to_bytes(&mut temp[j * samples as usize..], pixels[i][j] as u64, samples as usize);
        }
        buffer.copy_from_slice(&temp);
        for (k, byte) in buffer.iter().enumerate() {
            image_buffer[i * width * samples as usize + k] = *byte;
        }
    }
    let mut tiff_stream = Cursor::new(Vec::new());
    let mut encoder = TiffEncoder::new(&mut tiff_stream).unwrap();

    let mut image = encoder.new_image::<colortype::RGB8>(
        width as u32,
        height as u32,
    ).unwrap();

    let _ = image.encoder().write_tag(tags::Tag::PlanarConfiguration, 1);
    let _ = image.encoder().write_tag(tags::Tag::PhotometricInterpretation, 2);

    let _ = image.encoder().write_tag(tags::Tag::XResolution, 300);
    let _ = image.encoder().write_tag(tags::Tag::YResolution, 300);
    let _ = image.encoder().write_tag(tags::Tag::RowsPerStrip, width as u32 * samples);

    image.write_data(&image_buffer).unwrap();

    Ok(tiff_stream)
}

#[derive(Parser, Default, Debug)]
#[clap(version, author = "Milan Bolaric", about = "Parse PGS - Test App", name = "parse_sup_file")]
pub struct Args {
    #[clap(short, long)]
    pub pgs_file_name: String,

    #[clap(short, long)]
    pub tiff_file_name: String,

    #[clap(short, long)]
    pub display_set: u32,
}

pub fn main() {
    init_logging();

    let args = Args::parse();

    match PgsParser::parse(&args.pgs_file_name) {
        Ok(parser) => {
            let ds = &parser.get_display_sets()[args.display_set as usize];
            if ds.state() == PgsDisplaySetState::Incomplete {
                error!("Incomplete Display Set");
            }

            let tiff_data = get_tiff_stream(ds, true);
            let mut file = File::create(args.tiff_file_name).unwrap();   
            let _ = file.write_all(tiff_data.unwrap().get_ref());
            let _ = file.flush();
        },
        Err(err) => {
            error!("{:?}", err);
        }
    }
}