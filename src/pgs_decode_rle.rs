use std::rc::Rc;

use crate::{pgs_error::Result, pgs_memory_buffer::ReadBytes, PgsMemoryBuffer, PgsOdsSegment, PgsPdsSegment, PgsSeek};

fn byte_to_int(byte: u8) -> u32 {
    byte as u32
}

fn calc_red(y: u8, cr: u8) -> u8 {
    let r = y as f32 + 1.40200 * (cr as f32 - 0x80 as f32);
    r.clamp(0.0, 255.0) as u8
}

fn calc_green(y: u8, cb: u8, cr: u8) -> u8 {
    let g = y as f32 - 0.34414 * (cb as f32 - 0x80 as f32) - 0.71414 * (cr as f32 - 0x80 as f32);
    g.clamp(0.0, 255.0) as u8
}

fn calc_blue(y: u8, cb: u8) -> u8 {
    let b = y as f32 + 1.77200 * (cb as f32 - 0x80 as f32);
    b.clamp(0.0, 255.0) as u8
}

fn get_argb(y: u8, cb: u8, cr: u8, transparency: u8) -> u32 {
    (calc_blue(y, cb) as u32) | ((calc_green(y, cb, cr) as u32) << 8) | ((calc_red(y, cr) as u32) << 16) | ((transparency as u32) << 24)
}

fn calc_gray(transparency: u8, luminance: u8) -> u32 {
	let tmp: u32 = 255 - transparency as u32 * luminance as u32 / 255;
	tmp | tmp << 8 | tmp << 16
}

fn get_gray_color(color: usize, pds: &Rc<PgsPdsSegment>) -> u32 {
    if color >= pds.palette_entries.len() { 
        0xFFFFFF
    } else {
        calc_gray(pds.palette_entries[color].transparency, pds.palette_entries[color].luminance)
    }
}

fn get_argb_color(color: usize, pds: &Rc<PgsPdsSegment>) -> u32 {
    if color >= pds.palette_entries.len() { 
        0xFFFFFF
    } else {
        let palette = &pds.palette_entries[color];
        get_argb(palette.luminance, palette.color_difference_blue, palette.color_difference_red,  palette.transparency)
    }
}

fn get_pixel_color(color: usize, pds: &Rc<PgsPdsSegment>, gray: bool) -> u32 {
    if gray { 
        get_gray_color(color, pds) 
    } else { 
        get_argb_color(color, pds)
    }
}

pub fn decode_rle(pds: Rc<PgsPdsSegment>, ods: Rc<PgsOdsSegment>, gray: bool) -> Result<Vec<Vec<u32>>> {
    let mut pixels: Vec<Vec<u32>> = vec![vec![0_u32; ods.width as usize]; ods.height as usize];

    let mut col: usize = 0;
    let mut row: usize = 0;

    let data = ods.object_data.as_slice();
    let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(data);
    let buffer_len = buffer.len()?;
    while buffer.pos()? < buffer_len {
        match buffer.read_u8()? {
            0x00 => {                
                match buffer.read_u8()? {
                    0x00 => {      
                        row += 1;
                        col = 0;
                        if buffer.pos()? >= buffer_len { 
                            break; 
                        }
                        continue;
                    },
                    data => {
                        match (data & 0xC0) >> 6 {
                            0 => {
                                for _ in 0..byte_to_int(data) {
                                    pixels[row][col] = get_pixel_color(0, &pds, gray);
                                    col += 1;
                                }
                            },
                            1 => {
                                let count = byte_to_int(buffer.read_u8()?) | (byte_to_int(data & 0x3F) << 8);
                                for _ in 0..count {
                                    pixels[row][col] = get_pixel_color(0, &pds, gray);
                                    col += 1;
                                }
                            },
                            2 => {
                                let color = byte_to_int(buffer.read_u8()?) as usize;
                                for _ in 0..byte_to_int(data & 0x3F) {
                                    pixels[row][col] = get_pixel_color(color, &pds, gray);
                                    col += 1;
                                }
                            },
                            3 => {
                                let count = byte_to_int(buffer.read_u8()?) | (byte_to_int(data & 0x3F) << 8);
                                let color = byte_to_int(buffer.read_u8()?) as usize;
                                for _ in 0..count {
                                    pixels[row][col] = get_pixel_color(color, &pds, gray);
                                    col += 1;
                                }
                            },
                            _ => {}
                        }
                    }
                }
            },
            data => {
                let color = byte_to_int(data) as usize;
                pixels[row][col] = get_pixel_color(color, &pds, gray);
                col += 1;
            }
        }
    }
    Ok(pixels)
}
