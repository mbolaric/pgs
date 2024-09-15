//! # PGS Display Set
//!
//! This module defines the `PgsDisplaySet` struct, which represents a collection of segments
//! involved in the rendering of a PGS (Presentation Graphic Stream) subtitle frame. The display set
//! includes PCS (Presentation Composition Segment), WDS (Window Definition Segment), PDS (Palette
//! Definition Segment), and ODS (Object Definition Segment). The state of the display set can be
//! used to determine if a frame is complete and ready for rendering.

use std::rc::Rc;

use crate::{pgs_decode_rle::decode_rle, Error, PgsOdsSegment, PgsPcsSegment, PgsPdsSegment, PgsWdsSegment, Result};

/// Enum representing the state of the `PgsDisplaySet`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PgsDisplaySetState {
    /// The display set is incomplete, not all required segments (PCS, WDS, PDS, ODS) are present.
    Incomplete,
    /// The display set is complete and ready to be rendered.
    Complete,
    /// The display set contains only empty frames, meaning it has PCS and WDS but no PDS or ODS.
    EmptyFrame
}

/// Struct representing a collection of PGS segments required for rendering a single subtitle frame.
/// The segments include:
/// - `pcs`: Presentation Composition Segment.
/// - `wds`: Window Definition Segment.
/// - `pds`: Palette Definition Segment.
/// - `ods`: Object Definition Segment.
#[derive(Debug, Default, Clone)]
pub struct PgsDisplaySet {
    pub pcs: Option<Rc<PgsPcsSegment>>,
    pub wds: Option<Rc<PgsWdsSegment>>,
    pub pds: Option<Rc<PgsPdsSegment>>,
    pub ods: Option<Rc<PgsOdsSegment>>
}

impl PgsDisplaySet {
    /// Creates a new, empty `PgsDisplaySet` with no segments.
    ///
    /// # Returns
    /// A new `PgsDisplaySet` instance with all segment options set to `None`.
    pub fn new() -> Self {
        PgsDisplaySet {
            pcs: None,
            wds: None,
            pds: None,
            ods: None
        }
    }

    /// Clears the display set by setting all segments (PCS, WDS, PDS, ODS) to `None`.
    ///
    /// This can be used to reset the display set for reuse.
    pub(crate) fn clean(&mut self) {
        self.pcs = None;
        self.wds = None;
        self.pds = None;
        self.ods = None;
    }

    /// Determines the current state of the display set.
    ///
    /// - If PCS and WDS are present, but PDS and ODS are not, the state is `EmptyFrame`.
    /// - If all segments (PCS, WDS, PDS, ODS) are present, the state is `Complete`.
    /// - Otherwise, the state is `Incomplete`.
    ///
    /// # Returns
    /// The current state of the `PgsDisplaySet`.
    pub fn state(&self) -> PgsDisplaySetState {
        if self.pcs.is_some() && self.wds.is_some() {
            if self.pds.is_some() && self.ods.is_some() {
                return PgsDisplaySetState::Complete;
            }
            return PgsDisplaySetState::EmptyFrame;
        }
        PgsDisplaySetState::Incomplete
    }

    /// Returns a reference to the RLE (Run-Length Encoded) image data contained in the ODS segment.
    ///
    /// # Errors
    /// Returns `Error::IncompleteDisplaySet` if the display set is not in the `Complete` state.
    ///
    /// # Returns
    /// A reference to the raw RLE image data.    
    pub fn get_rle_image(&self) -> Result<&Vec<u8>> {
        if self.state() != PgsDisplaySetState::Complete {
            return Err(Error::IncompleteDisplaySet);
        }
        Ok(&self.ods.as_ref().unwrap().object_data)
    }

    /// Decodes the RLE image data and returns the image as a 2D array of pixels.
    ///
    /// This function decodes the image contained in the ODS segment using the palette from the PDS
    /// segment. It can return the image in either grayscale or color depending on the `gray` parameter.
    ///
    /// # Parameters
    /// - `gray`: A boolean flag indicating whether to decode the image in grayscale (`true`) or color (`false`).
    ///
    /// # Errors
    /// Returns `Error::IncompleteDisplaySet` if the display set is not in the `Complete` state.
    ///
    /// # Returns
    /// A 2D vector containing the decoded pixels, where each pixel is represented as a 32-bit color value.
    pub fn get_decoded_image(&self, gray: bool) -> Result<Vec<Vec<u32>>> {
        if self.state() != PgsDisplaySetState::Complete {
            return Err(Error::IncompleteDisplaySet);
        }

        let ods: &Rc<PgsOdsSegment> = self.ods.as_ref().unwrap();
        let pds = self.pds.as_ref().unwrap();
        let pixels = decode_rle(pds.clone(), ods.clone(), gray)?;
        Ok(pixels)
    }
}