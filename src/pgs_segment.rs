use std::rc::Rc;

use crate::{PgsOdsSegment, PgsPcsSegment, PgsPdsSegment, PgsWdsSegment};

/// Enum representing different types of PGS (Presentation Graphic Stream) segments.
/// These segments are used in Blu-ray subtitles to define various aspects of the subtitle data.
#[derive(Debug)]
pub enum PgsSegment {
    Pcs(Rc<PgsPcsSegment>),
    Wds(Rc<PgsWdsSegment>),
    Pds(Rc<PgsPdsSegment>),
    Ods(Rc<PgsOdsSegment>),
    End,
}