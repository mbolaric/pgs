use crate::pgs_const::PG;
use crate::pgs_memory_buffer::{BigEndian, ReadBytes};
use crate::pgs_segment_type::PgsSegmentType;
use crate::pgs_error::{Result, Error};
use crate::PgsMemoryBuffer;

pub const PGS_SEGMENT_HEADER_LENGTH: usize = 13;

#[derive(Debug)]
pub struct PgsSegmentHeader {
    pub segment_type: PgsSegmentType,
    pub segment_length: u16,
    pub presentation_timestamp: u32,
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
    fn default() -> Self {
        Self { segment_type: PgsSegmentType::ERR, segment_length: 0, presentation_timestamp: 0, decoding_timestamp: 0 }
    }
}
