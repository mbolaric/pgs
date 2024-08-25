use std::rc::Rc;

use crate::{pgs_memory_buffer::{BigEndian, ReadBytes}, Error, PgsMemoryBuffer, PgsSegmentHeader, Result};

#[derive(Debug)]
pub struct PgsWdsSegmentWindowDefinition {
    pub window_id: u8,
    pub window_horizontal_position: u16,
    pub window_vertical_position: u16,
    pub window_width: u16,
    pub window_height: u16
}

impl PgsWdsSegmentWindowDefinition {
    fn new(window_id: u8,
        window_horizontal_position: u16,
        window_vertical_position: u16,
        window_width: u16,
        window_height: u16) -> Self {
        PgsWdsSegmentWindowDefinition {
            window_id,
            window_horizontal_position,
            window_vertical_position,
            window_width,
            window_height
        }
    }
}

#[derive(Debug)]
pub struct PgsWdsSegment {
    pub header: PgsSegmentHeader,
    pub number_of_windows: u8,
    pub windows: Vec<PgsWdsSegmentWindowDefinition>
}

impl PgsWdsSegment {
    fn new(header: PgsSegmentHeader, number_of_windows: u8, windows: Vec<PgsWdsSegmentWindowDefinition>) -> Self {
        PgsWdsSegment {
            header,
            number_of_windows,
            windows
        }
    }

    pub fn from_data(header: PgsSegmentHeader, data: &[u8]) -> Result<Rc<PgsWdsSegment>> {
        if data.len() < header.segment_length as usize {
            return Err(Error::InvalidSegmentDataLength);
        }

        let mut buffer: PgsMemoryBuffer = PgsMemoryBuffer::from(data);
        let number_of_windows = buffer.read_u8()?;
        let mut windows: Vec<PgsWdsSegmentWindowDefinition> = Vec::new();
        for _ in 0..number_of_windows {
            let window_id = buffer.read_u8()?;
            let window_horizontal_position = buffer.read_u16::<BigEndian>()?;
            let window_vertical_position = buffer.read_u16::<BigEndian>()?;
            let window_width = buffer.read_u16::<BigEndian>()?;
            let window_height = buffer.read_u16::<BigEndian>()?;
            windows.push(PgsWdsSegmentWindowDefinition::new(window_id, window_horizontal_position, window_vertical_position, window_width, window_height))
        }

        Ok(Rc::new(PgsWdsSegment::new(header, number_of_windows, windows)))
    }
}