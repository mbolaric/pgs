use std::fmt::Display;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PgsSegmentType {
    PDS = 0x14, // Palette Definition Segment 
	ODS = 0x15, // Object Definition Segment 
	PCS = 0x16, // Presentation Composition Segment 
	WDS = 0x17, // Window Definition Segment 
	END = 0x80, // End of Display Set Segment 
	ERR = 0x00  // Error in Segment
}

impl Display for PgsSegmentType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
            PgsSegmentType::PDS => write!(f, "Palette Definition Segment"),
            PgsSegmentType::ODS => write!(f, "Object Definition Segment"),
            PgsSegmentType::PCS => write!(f, "Presentation Composition Segment"),
            PgsSegmentType::WDS => write!(f, "Window Definition Segment"),
            PgsSegmentType::END => write!(f, "End of Display Set Segment"),
            PgsSegmentType::ERR => write!(f, "Error in Segment"),
        }
	}
}

impl From<u8> for PgsSegmentType {
	fn from(value: u8) -> Self {
		match value {
			0x14 => PgsSegmentType::PDS,
			0x15 => PgsSegmentType::ODS,
			0x16 => PgsSegmentType::PCS,
			0x17 => PgsSegmentType::WDS,
			0x80 => PgsSegmentType::END,
			_ => PgsSegmentType::ERR
        }
	}
}