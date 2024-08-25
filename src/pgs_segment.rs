use std::rc::Rc;

use crate::{PgsOdsSegment, PgsPcsSegment, PgsPdsSegment, PgsWdsSegment};

#[derive(Debug)]
pub enum PgsSegment {
    Pcs(Rc<PgsPcsSegment>),
    Wds(Rc<PgsWdsSegment>),
    Pds(Rc<PgsPdsSegment>),
    Ods(Rc<PgsOdsSegment>),
    End,
}