//! # PGS Segment Header
//!
//! This module defines the `PgsSegmentHeader` struct, which represents the header of a PGS segment.

use crate::pgs_const::PG;
use crate::pgs_memory_buffer::{BigEndian, ReadBytes};
use crate::pgs_segment_type::PgsSegmentType;
use crate::pgs_error::{Result, Error};
use crate::PgsMemoryBuffer;

/// Constant defining the length of a PGS segment header.
pub const PGS_SEGMENT_HEADER_LENGTH: usize = 13;

/// Struct representing the header of a PGS segment.
#[derive(Debug)]
pub struct PgsSegmentHeader {
    // The type of the segment, as defined by the PGS specification.
    pub segment_type: PgsSegmentType,
    /// The length of the segment (excluding the header).
    pub segment_length: u16,
    /// The presentation timestamp (PTS) for the segment.
    pub presentation_timestamp: u32,
    /// The decoding timestamp (DTS) for the segment.
    pub decoding_timestamp: u32
}

impl PgsSegmentHeader {
    fn new(segment_type: PgsSegmentType, presentation_timestamp: u32, decoding_timestamp: u32, segment_length: u16) -> Self {
        PgsSegmentHeader {
            segment_type,
            segment_length,
            presentation_timestamp,
            decoding_timestamp
        }
    }

    /// Parses a `PgsSegmentHeader` from the provided raw data.
    ///
    /// This function reads the necessary fields from the data buffer and constructs a `PgsSegmentHeader`.
    /// It checks the validity of the header, ensuring that the segment begins with the expected `PG` marker.
    ///
    /// # Parameters
    /// - `data`: A slice of raw data representing the contents of the PGS segment header.
    ///
    /// # Errors
    /// Returns an error if the length of the data is less than the required header length or if the header is invalid.
    ///
    /// # Returns
    /// A `PgsSegmentHeader` constructed from the provided data.
    pub fn from_data(data: &[u8]) -> Result<PgsSegmentHeader> {
        if data.len() < PGS_SEGMENT_HEADER_LENGTH {
            return Err(Error::InvalidSegmentDataLength);
        }

        let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(data);

        let pg = buffer.read_u16::<BigEndian>()?;
        if pg != PG {
            return Err(Error::ReadInvalidSegment);
        }
        
        let pts = buffer.read_u32::<BigEndian>()?;
        let dts = buffer.read_u32::<BigEndian>()?;
        let s_type = PgsSegmentType::from(buffer.read_u8()?);
        let s_size = buffer.read_u16::<BigEndian>()?;

        Ok(PgsSegmentHeader::new(s_type, pts, dts, s_size))
    }
}

impl Default for PgsSegmentHeader {
    /// Provides a default implementation for the `PgsSegmentHeader`.
    /// This default header has a segment type of `ERR` and zero for all other fields.
    fn default() -> Self {
        Self { segment_type: PgsSegmentType::ERR, segment_length: 0, presentation_timestamp: 0, decoding_timestamp: 0 }
    }
}
