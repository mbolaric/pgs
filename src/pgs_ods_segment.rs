//! # PGS Object Definition Segment (ODS)
//!
//! This module defines the `PgsOdsSegment` struct, which represents the Object Definition Segment (ODS) 
//! in the Presentation Graphic Stream (PGS) format. The ODS contains the object data for a subtitle, 
//! such as the image itself, along with metadata like width, height, and sequence information.

use std::rc::Rc;

use crate::{pgs_memory_buffer::{BigEndian, ReadBytes}, Error, PgsMemoryBuffer, PgsSegmentHeader, Result};

/// Enum representing the sequence flag in an ODS.
/// The sequence flag indicates whether this segment is part of a sequence, and if it is, 
/// whether it is the first, last, or both.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum PgsOdsSequenceFlag {
    Unknown,
    First,
    Last,
    Both,
}

impl From<u8> for PgsOdsSequenceFlag {
    /// Converts a raw `u8` value to the corresponding `PgsOdsSequenceFlag` enum variant.
    ///
    /// # Parameters
    /// - `value`: The raw `u8` value representing the sequence flag.
    ///
    /// # Returns
    /// The corresponding `PgsOdsSequenceFlag` variant.    
    fn from(value: u8) -> Self {
        match value {
            0x40 => PgsOdsSequenceFlag::Last,
            0x80 => PgsOdsSequenceFlag::First,
            0xC0 => PgsOdsSequenceFlag::Both,
            _ => PgsOdsSequenceFlag::Unknown
        }
    }
}

/// Struct representing an Object Definition Segment (ODS) in a PGS file.
/// The ODS contains the actual image data (subtitle graphics) along with metadata.
#[derive(Debug)]
pub struct PgsOdsSegment {
    pub header: PgsSegmentHeader,
    pub object_id: u16,
    pub object_version_number: u8,
    pub last_in_sequence_flag: PgsOdsSequenceFlag,
    pub object_data_length: u32,
    pub width: u16,
    pub height: u16,
    pub object_data: Vec<u8>
}

impl PgsOdsSegment {
    /// Creates a new, empty `PgsOdsSegment` with the given header.
    ///
    /// # Parameters
    /// - `header`: The segment header for the ODS.
    ///
    /// # Returns
    /// A new `PgsOdsSegment` instance with default values for the object data.
    fn new(header: PgsSegmentHeader) -> Self {
        PgsOdsSegment {
            header,
            object_id: 0,
            object_version_number: 0,
            last_in_sequence_flag: PgsOdsSequenceFlag::Unknown,
            object_data_length: 0,
            width: 0,
            height: 0,
            object_data: Vec::new()
        }
    }

    /// Constructs a `PgsOdsSegment` from the given header and raw data buffer.
    ///
    /// This method parses the ODS segment data, extracting the object ID, version number, sequence flag,
    /// object data length, and optionally the object width and height (if this is the first or both sequence flag).
    ///
    /// # Parameters
    /// - `header`: The segment header.
    /// - `data`: A slice of raw data representing the contents of the ODS segment.
    ///
    /// # Errors
    /// Returns `Error::InvalidSegmentDataLength` if the length of the provided data is less than the expected length.
    ///
    /// # Returns
    /// An `Rc<PgsOdsSegment>` containing the parsed segment.
    pub fn from_data(header: PgsSegmentHeader, data: &[u8]) -> Result<Rc<PgsOdsSegment>> {
        if data.len() < header.segment_length as usize {
            return Err(Error::InvalidSegmentDataLength);
        }

        let mut segment = PgsOdsSegment::new(header);

        let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(data);
        segment.object_id = buffer.read_u16::<BigEndian>()?;
        segment.object_version_number = buffer.read_u8()?;
        segment.last_in_sequence_flag = PgsOdsSequenceFlag::from(buffer.read_u8()?);

        // Length have different of 4 bytes because w/h
        segment.object_data_length = buffer.read_u24::<BigEndian>()? - 4;

        let (width, height) = if segment.last_in_sequence_flag == PgsOdsSequenceFlag::First || segment.last_in_sequence_flag == PgsOdsSequenceFlag::Both {
            let width = buffer.read_u16::<BigEndian>()?;
            let height = buffer.read_u16::<BigEndian>()?;
            (width, height)
        } else {
            (0, 0)
        };

        segment.width = width;
        segment.height = height;
        segment.object_data = buffer.read_into_vec(segment.object_data_length)?;

        Ok(Rc::new(segment))
    }
}