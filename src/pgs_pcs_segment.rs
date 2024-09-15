//! # PGS Presentation Composition Segment (PCS)
//!
//! This module defines the `PgsPcsSegment` struct, which represents the Presentation Composition Segment (PCS)
//! in the Presentation Graphic Stream (PGS) format. The PCS provides the details of how the subtitles (or other
//! graphic elements) are arranged and displayed on the screen.

use std::rc::Rc;

use crate::{pgs_memory_buffer::{BigEndian, ReadBytes}, pgs_segment_header::PgsSegmentHeader, Error, PgsMemoryBuffer, Result};

/// Enum representing the object cropping flag in a PCS.
/// This flag indicates whether the object (subtitle image) is cropped and whether a forced cropped image should be used.
#[derive(Debug, PartialEq)]
pub enum PgsPcsObjectCroppedFlag {
    ForceCroppedImage = 0x40,
    Off = 0x00
}

impl From<u8> for PgsPcsObjectCroppedFlag {
    /// Converts a raw `u8` value to the corresponding `PgsPcsObjectCroppedFlag` enum variant.
    ///
    /// # Parameters
    /// - `value`: The raw `u8` value representing the cropped flag.
    ///
    /// # Returns
    /// The corresponding `PgsPcsObjectCroppedFlag` variant.    
    fn from(value: u8) -> Self {
        match value {
            0x40 => PgsPcsObjectCroppedFlag::ForceCroppedImage,
            _ => PgsPcsObjectCroppedFlag::Off
        }
    }
}

/// Struct representing a composition object in a PCS.
/// Composition objects describe the individual graphic elements that make up the subtitle image and its placement on the screen.
#[derive(Debug)]
pub struct PgsPcsSegmentCompositionObjects {
    pub object_id: u16,
    pub window_id: u8,
    pub object_cropped_flag: PgsPcsObjectCroppedFlag,
    pub object_horizontal_position: u16,
    pub object_vertical_position: u16,
    pub object_cropping_horizontal_position: u16,
    pub object_cropping_vertical_position: u16,
    pub object_cropping_width: u16,
    pub object_cropping_height_position: u16
}

impl PgsPcsSegmentCompositionObjects {
    fn new() -> Self {
        PgsPcsSegmentCompositionObjects {
            object_id: 0,
            window_id: 0,
            object_cropped_flag: PgsPcsObjectCroppedFlag::Off,
            object_horizontal_position: 0,
            object_vertical_position: 0,
            object_cropping_horizontal_position: 0,
            object_cropping_vertical_position: 0,
            object_cropping_width: 0,
            object_cropping_height_position: 0
        }
    }
}

/// Enum representing the composition state of a PCS.
/// The composition state describes whether the segment starts a new display, refreshes an existing display, or updates a display.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PgsPcsCompositionState {
    // Defines a new display.
    EpochStart,
    // Defines a display refresh.
    AcquisitionPoint,
    // Defines a display update.
    Normal,
}

impl From<u8> for PgsPcsCompositionState {
    /// Converts a raw `u8` value to the corresponding `PgsPcsCompositionState` enum variant.
    ///
    /// # Parameters
    /// - `value`: The raw `u8` value representing the composition state.
    ///
    /// # Returns
    /// The corresponding `PgsPcsCompositionState` variant.    
    fn from(value: u8) -> Self {
        match value {
            0x80 => PgsPcsCompositionState::EpochStart,
            0x40 => PgsPcsCompositionState::AcquisitionPoint,
            _ => PgsPcsCompositionState::Normal
        }
    }
}

#[derive(Debug)]
pub struct PgsPcsSegment {
    pub header: PgsSegmentHeader,
    pub width: u16,
    pub height: u16,
    pub frame_rate: u8,
    pub composition_number: u16,
    pub composition_state: PgsPcsCompositionState,
    pub palette_update_flag: u8,
    pub palette_id: u8,
    pub number_of_composition_objects: u8,
    pub composition_objects: Vec<PgsPcsSegmentCompositionObjects>
}

/// Struct representing a Presentation Composition Segment (PCS) in a PGS file.
/// The PCS defines how individual graphic objects (subtitles, etc.) are displayed on the screen, their position, and composition state.
impl PgsPcsSegment {
    fn new(header: PgsSegmentHeader) -> Self {
        PgsPcsSegment {
            header,
            width: 0,
            height: 0,
            frame_rate: 0x10,
            composition_number: 0,
            composition_state: PgsPcsCompositionState::Normal,
            palette_update_flag: 0,
            palette_id: 0,
            number_of_composition_objects: 0,
            composition_objects: Vec::new()
        }
    }

    /// Creates a new, empty `PgsPcsSegment`.
    ///
    /// # Parameters
    /// - `header`: The segment header for the PCS.
    ///
    /// # Returns
    /// A new `PgsPcsSegment` instance with default values for the composition objects.
    pub fn from_data(header: PgsSegmentHeader, data: &[u8]) -> Result<Rc<PgsPcsSegment>> {
        if data.len() < header.segment_length as usize {
            return Err(Error::InvalidSegmentDataLength);
        }

        let mut segment = PgsPcsSegment::new(header);

        let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(data);
        segment.width = buffer.read_u16::<BigEndian>()?;
        segment.height = buffer.read_u16::<BigEndian>()?;
        let _ = buffer.read_u8()?;
        segment.composition_number = buffer.read_u16::<BigEndian>()?;
        segment.composition_state = PgsPcsCompositionState::from(buffer.read_u8()?);
        segment.palette_update_flag = buffer.read_u8()?;
        segment.palette_id = buffer.read_u8()?;
        segment.number_of_composition_objects = buffer.read_u8()?;

        for _ in 0..segment.number_of_composition_objects {
            let mut com_obj = PgsPcsSegmentCompositionObjects::new();
            com_obj.object_id = buffer.read_u16::<BigEndian>()?;
            com_obj.window_id = buffer.read_u8()?;
            com_obj.object_cropped_flag = PgsPcsObjectCroppedFlag::from(buffer.read_u8()?);
            com_obj.object_horizontal_position = buffer.read_u16::<BigEndian>()?;
            com_obj.object_vertical_position = buffer.read_u16::<BigEndian>()?;
            if com_obj.object_cropped_flag == PgsPcsObjectCroppedFlag::ForceCroppedImage {
                com_obj.object_cropping_horizontal_position = buffer.read_u16::<BigEndian>()?;
                com_obj.object_cropping_vertical_position = buffer.read_u16::<BigEndian>()?;
                com_obj.object_cropping_width = buffer.read_u16::<BigEndian>()?;
                com_obj.object_cropping_height_position = buffer.read_u16::<BigEndian>()?;
            }

            segment.composition_objects.push(com_obj);
        }

        Ok(Rc::new(segment))
    }
}

impl Default for PgsPcsSegment {
    fn default() -> Self {
        Self::new(Default::default())
    }
}