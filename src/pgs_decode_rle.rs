use std::rc::Rc;

use crate::{pgs_error::Result, pgs_memory_buffer::ReadBytes, PgsMemoryBuffer, PgsOdsSegment, PgsPdsSegment, PgsSeek};

/// Converts a single byte to an unsigned 32-bit integer.
pub fn byte_to_int(byte: u8) -> u32 {
    byte as u32
}

/// Calculates the red channel from the YCrCb color model.
/// Takes the luminance (Y) and chrominance red (Cr) values as input.
pub fn calc_red(y: u8, cr: u8) -> u8 {
    let r = y as f32 + 1.40200 * (cr as f32 - 0x80 as f32);
    r.clamp(0.0, 255.0) as u8
}

/// Calculates the green channel from the YCbCr color model.
/// Takes the luminance (Y), chrominance blue (Cb), and chrominance red (Cr) values as input.
pub fn calc_green(y: u8, cb: u8, cr: u8) -> u8 {
    let g = y as f32 - 0.34414 * (cb as f32 - 0x80 as f32) - 0.71414 * (cr as f32 - 0x80 as f32);
    g.clamp(0.0, 255.0) as u8
}

/// Calculates the blue channel from the YCbCr color model.
/// Takes the luminance (Y) and chrominance blue (Cb) values as input.
pub fn calc_blue(y: u8, cb: u8) -> u8 {
    let b = y as f32 + 1.77200 * (cb as f32 - 0x80 as f32);
    b.clamp(0.0, 255.0) as u8
}

/// Combines Y, Cb, Cr, and transparency into an ARGB 32-bit color.
/// Returns a 32-bit ARGB color value using the YCbCr to RGB color space conversion.
pub fn get_argb(y: u8, cb: u8, cr: u8, transparency: u8) -> u32 {
    (calc_blue(y, cb) as u32) | ((calc_green(y, cb, cr) as u32) << 8) | ((calc_red(y, cr) as u32) << 16) | ((transparency as u32) << 24)
}

/// Calculates a grayscale color using transparency and luminance values.
/// Returns a 32-bit grayscale value (gray intensity replicated across RGB channels).
pub fn calc_gray(transparency: u8, luminance: u8) -> u32 {
	let tmp: u32 = 255 - transparency as u32 * luminance as u32 / 255;
	tmp | tmp << 8 | tmp << 16
}

/// Retrieves the grayscale color from a PDS segment palette entry, or white if out of bounds.
pub fn get_gray_color(color: usize, pds: &Rc<PgsPdsSegment>) -> u32 {
    if color >= pds.palette_entries.len() { 
        0xFFFFFF
    } else {
        calc_gray(pds.palette_entries[color].transparency, pds.palette_entries[color].luminance)
    }
}

/// Retrieves the ARGB color from a PDS segment palette entry, or white if out of bounds.
pub fn get_argb_color(color: usize, pds: &Rc<PgsPdsSegment>) -> u32 {
    if color >= pds.palette_entries.len() { 
        0xFFFFFF
    } else {
        let palette = &pds.palette_entries[color];
        get_argb(palette.luminance, palette.color_difference_blue, palette.color_difference_red,  palette.transparency)
    }
}

/// Retrieves either grayscale or ARGB color, depending on the `gray` flag.
pub fn get_pixel_color(color: usize, pds: &Rc<PgsPdsSegment>, gray: bool) -> u32 {
    if gray { 
        get_gray_color(color, pds) 
    } else { 
        get_argb_color(color, pds)
    }
}

/// Decodes a Run-Length Encoded (RLE) bitmap using a PDS and ODS segment, returning a 2D array of pixel colors.
/// 
/// Arguments:
/// - `pds`: Reference-counted pointer to a `PgsPdsSegment` for palette data.
/// - `ods`: Reference-counted pointer to a `PgsOdsSegment` for object data (RLE).
/// - `gray`: Boolean flag indicating if grayscale color conversion should be used.
///
/// Returns:
/// - A 2D vector representing pixel colors decoded from the RLE data.
pub fn decode_rle(pds: Rc<PgsPdsSegment>, ods: Rc<PgsOdsSegment>, gray: bool) -> Result<Vec<Vec<u32>>> {
    // Create a 2D vector of pixels initialized to 0, with dimensions (width x height) based on the ODS.
    let mut pixels: Vec<Vec<u32>> = vec![vec![0_u32; ods.width as usize]; ods.height as usize];

    let mut col: usize = 0;
    let mut row: usize = 0;

    let data = ods.object_data.as_slice();
    let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(data);
    let buffer_len = buffer.len()?;
    while buffer.pos()? < buffer_len {
        match buffer.read_u8()? {
            0x00 => { // Special case: handle new row or extended RLE data.
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
            data => { // Standard case: a single color pixel.
                let color = byte_to_int(data) as usize;
                pixels[row][col] = get_pixel_color(color, &pds, gray);
                col += 1;
            }
        }
    }
    Ok(pixels)
}

#[cfg(test)]
mod tests {
    use crate::{PgsOdsSequenceFlag, PgsPdsSegmentPaletteEntry, PgsSegmentHeader, PgsSegmentType};

    use super::*;

    #[test]
    fn test_calc_red() {
        assert_eq!(calc_red(128, 128), 128);  // Middle YCbCr value for red
        assert_eq!(calc_red(255, 128), 255);  // Max luminance (Y) value, results in max red
        assert_eq!(calc_red(0, 128), 0);      // Min luminance (Y) value, results in min red
    }

    #[test]
    fn test_calc_green() {
        assert_eq!(calc_green(128, 128, 128), 128);  // Middle YCbCr value for green
        assert_eq!(calc_green(255, 128, 128), 255);  // Max luminance (Y) value, results in max green
        assert_eq!(calc_green(0, 128, 128), 0);      // Min luminance (Y) value, results in min green
    }
 
    #[test]
    fn test_calc_blue() {
        assert_eq!(calc_blue(128, 128), 128);  // Middle YCbCr value for blue
        assert_eq!(calc_blue(255, 128), 255);  // Max luminance (Y) value, results in max blue
        assert_eq!(calc_blue(0, 128), 0);      // Min luminance (Y) value, results in min blue
    }

    #[test]
    fn test_calc_gray() {
        // 1: No transparency (255), full luminance (255)
        let result = calc_gray(0, 255);
        assert_eq!(result, 0xFFFFFF, "Expected white color for no transparency and full luminance");

        // 2: Full transparency (255), full luminance (255)
        let result = calc_gray(255, 255);
        assert_eq!(result, 0x000000, "Expected black color for full transparency and full luminance");

        // 3: Partial transparency (128), full luminance (255)
        let result = calc_gray(128, 255);
        assert_eq!(result, 0x7F7F7F, "Expected gray color for half transparency and full luminance");

        // 4: No transparency (0), no luminance (0)
        let result = calc_gray(0, 0);
        assert_eq!(result, 0xFFFFFF, "Expected white color for no transparency and no luminance");

        // 5: Partial transparency (128), partial luminance (128)
        let result = calc_gray(128, 128);
        assert_eq!(result, 0xBFBFBF, "Expected a light gray color for partial transparency and partial luminance");
    }

    #[test]
    fn test_get_argb() {
        let argb = get_argb(128, 128, 128, 255);  // Middle values for YCbCr with full transparency
        assert_eq!(argb, 0xFF808080);  // Should return the ARGB value (255 alpha, mid gray color)
    }

    #[test]
    fn test_get_gray_color() {
        let pds_segment = Rc::new(PgsPdsSegment {
            header: PgsSegmentHeader {
                segment_type: PgsSegmentType::PDS,
                segment_length: 13,
                presentation_timestamp: 0,
                decoding_timestamp: 0
            },
            palette_id: 0,
            palette_version_number: 0,            
            palette_entries: vec![
                PgsPdsSegmentPaletteEntry { palette_entry_id: 0, transparency: 0, luminance: 255, color_difference_blue: 0, color_difference_red: 0 }, // White color
                PgsPdsSegmentPaletteEntry { palette_entry_id: 0, transparency: 255, luminance: 255, color_difference_blue: 0, color_difference_red: 0 }, // Black color
                PgsPdsSegmentPaletteEntry { palette_entry_id: 0, transparency: 128, luminance: 128, color_difference_blue: 0, color_difference_red: 0 } // Gray color
            ]
        });
 
        // 1: Valid index, expecting white color
        let result = get_gray_color(0, &pds_segment);
        assert_eq!(result, 0xFFFFFF, "Expected white color for palette index 0");

        // 2: Valid index, expecting black color
        let result = get_gray_color(1, &pds_segment);
        assert_eq!(result, 0x000000, "Expected black color for palette index 1");

        // 3: Valid index, expecting gray color
        let result = get_gray_color(2, &pds_segment);
        assert_eq!(result, 0xBFBFBF, "Expected gray color for palette index 2");

        // 4: Invalid index, should return white (0xFFFFFF)
        let result = get_gray_color(3, &pds_segment);
        assert_eq!(result, 0xFFFFFF, "Expected default white color for out of bounds index");

    }

    #[test]
    fn test_rle_decoding() {
        let pds_segment = PgsPdsSegment {
            header: PgsSegmentHeader {
                segment_type: PgsSegmentType::PDS,
                segment_length: 13,
                presentation_timestamp: 0,
                decoding_timestamp: 0
            },
            palette_id: 0,
            palette_version_number: 0,
            palette_entries: vec![
                PgsPdsSegmentPaletteEntry { palette_entry_id: 0, transparency: 0, luminance: 50, color_difference_blue: 100, color_difference_red: 100 },
                PgsPdsSegmentPaletteEntry { palette_entry_id: 0, transparency: 0, luminance: 150, color_difference_blue: 200, color_difference_red: 200 }
            ],
        };
    
        let ods_segment = PgsOdsSegment {
            header: PgsSegmentHeader {
                segment_type: PgsSegmentType::ODS,
                segment_length: 13,
                presentation_timestamp: 0,
                decoding_timestamp: 0
            },
            object_id: 0,
            object_version_number: 0,
            last_in_sequence_flag: PgsOdsSequenceFlag::Unknown,
            width: 5,
            height: 2,
            object_data_length: 0,
            object_data: vec![
                0x00, 0x00, 0x01, 0x02, 0x01, 0x03, 0x02
            ],
        };

        let result = decode_rle(Rc::new(pds_segment), Rc::new(ods_segment), false).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 5);

        let expected = vec![
            vec![0x0, 0x0, 0x0, 0x0, 0x0], // Row 1
            vec![0xFA49FF, 0xFFFFFF, 0xFA49FF, 0xFFFFFF, 0xFFFFFF], // Row 2
        ];

        assert_eq!(result, expected, "Decoded RLE data does not match the expected output");
    }    
}