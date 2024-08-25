use std::rc::Rc;

use crate::{pgs_memory_buffer::{BigEndian, ReadBytes}, pgs_segment_header::PgsSegmentHeader, Error, PgsMemoryBuffer, Result};

#[derive(Debug, PartialEq)]
pub enum PgsPcsObjectCroppedFlag {
    ForceCroppedImage = 0x40,
    Off = 0x00
}

impl From<u8> for PgsPcsObjectCroppedFlag {
    fn from(value: u8) -> Self {
        match value {
            0x40 => PgsPcsObjectCroppedFlag::ForceCroppedImage,
            _ => PgsPcsObjectCroppedFlag::Off
        }
    }
}

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