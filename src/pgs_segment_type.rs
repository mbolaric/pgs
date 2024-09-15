use std::fmt::Display;

/// Represents the type of a segment in a Presentation Graphic Stream (PGS).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PgsSegmentType {
    /// Palette Definition Segment 
    PDS = 0x14,
    /// Object Definition Segment 
	ODS = 0x15,
    /// Presentation Composition Segment 
	PCS = 0x16,
    /// Window Definition Segment 
	WDS = 0x17,
    /// End of Display Set Segment 
	END = 0x80,
    /// Error in Segment
	ERR = 0x00  
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
    /// Converts a `u8` value to a `PgsSegmentType`.
    ///
    /// # Parameters
    /// - `value`: The `u8` value representing the segment type.
    ///
    /// # Returns
    /// The corresponding `PgsSegmentType`. Returns `PgsSegmentType::ERR` for unknown values.
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