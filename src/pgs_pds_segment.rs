//! # PGS Palette Definition Segment (PDS)
//!
//! This module defines the `PgsPdsSegment` struct, which represents the Palette Definition Segment (PDS)
//! in the Presentation Graphic Stream (PGS) format. The PDS defines color palettes used by the subtitles
//! or other graphical elements in a PGS file.

use std::{io::Read, rc::Rc};

use crate::{pgs_memory_buffer::ReadBytes, Error, PgsMemoryBuffer, PgsSegmentHeader, Result};

/// Struct representing an individual palette entry in a PDS.
/// Each palette entry consists of the palette ID and its corresponding color values (Y, Cr, Cb).
#[derive(Debug)]
pub struct PgsPdsSegmentPaletteEntry {
    pub palette_entry_id: u8,
    pub luminance: u8, // (Y)
    pub color_difference_red: u8, // (Cr)
    pub color_difference_blue: u8, // (Cb)
    pub transparency: u8
}

impl PgsPdsSegmentPaletteEntry {
    fn new(palette_entry_id: u8,
        luminance: u8,
        color_difference_red: u8,
        color_difference_blue: u8,
        transparency: u8) -> Self {
        PgsPdsSegmentPaletteEntry{
            palette_entry_id,
            luminance,
            color_difference_red,
            color_difference_blue,
            transparency
        }
    }
}

/// Struct representing a Palette Definition Segment (PDS) in a PGS file.
/// The PDS defines a color palette that can be used by various objects in the PGS file.
#[derive(Debug)]
pub struct PgsPdsSegment {
    pub header: PgsSegmentHeader,
    pub palette_id: u8,
    pub palette_version_number: u8,
    pub palette_entries: Vec<PgsPdsSegmentPaletteEntry>
}

impl PgsPdsSegment {
    fn new(header: PgsSegmentHeader,
        palette_id: u8,
        palette_version_number: u8,
        palette_entries: Vec<PgsPdsSegmentPaletteEntry>) -> Self {
        PgsPdsSegment {
            header,
            palette_id,
            palette_version_number,
            palette_entries
        }
    }

    /// Parses a `PgsPdsSegment` from the provided header and raw data buffer.
    ///
    /// This method reads the palette ID, version number, and individual palette entries from the data buffer.
    /// Each palette entry consists of luminance, color difference (Cr, Cb), and transparency values.
    ///
    /// # Parameters
    /// - `header`: The segment header.
    /// - `data`: A slice of raw data representing the contents of the PDS segment.
    ///
    /// # Errors
    /// Returns `Error::InvalidSegmentDataLength` if the length of the provided data is less than expected.
    ///
    /// # Returns
    /// An `Rc<PgsPdsSegment>` containing the parsed segment.
    pub fn from_data(header: PgsSegmentHeader, data: &[u8]) -> Result<Rc<PgsPdsSegment>> {
        if data.len() < header.segment_length as usize {
            return Err(Error::InvalidSegmentDataLength);
        }

        let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(data);
        let palette_id = buffer.read_u8()?;
        let palette_version_number = buffer.read_u8()?;

        let mut palette_buf: Vec<u8> = Vec::new();
        buffer.read_to_end(&mut palette_buf)?;

        // TODO: Return error if palette_buf.len() % 5 is not 0
        let palette_count = (palette_buf.len() as u32  - 2) / 5;

        let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(palette_buf);
        let mut palette_entries: Vec<PgsPdsSegmentPaletteEntry> = Vec::new();
        for _ in 0..palette_count {
            let palette_entry_id = buffer.read_u8()?;
            let luminance = buffer.read_u8()?;
            let color_difference_red = buffer.read_u8()?;
            let color_difference_blue = buffer.read_u8()?;
            let transparency = buffer.read_u8()?;
            palette_entries.push(PgsPdsSegmentPaletteEntry::new(palette_entry_id, luminance, color_difference_red, color_difference_blue, transparency))
        }


        Ok(Rc::new(PgsPdsSegment::new(header, palette_id, palette_version_number, palette_entries)))
    }
}