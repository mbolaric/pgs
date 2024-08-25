use std::rc::Rc;

use crate::{pgs_decode_rle::decode_rle, Error, PgsOdsSegment, PgsPcsSegment, PgsPdsSegment, PgsWdsSegment, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PgsDisplaySetState {
    Incomplete,
    Complete,
    EmptyFrame
}

#[derive(Debug, Default, Clone)]
pub struct PgsDisplaySet {
    pub pcs: Option<Rc<PgsPcsSegment>>,
    pub wds: Option<Rc<PgsWdsSegment>>,
    pub pds: Option<Rc<PgsPdsSegment>>,
    pub ods: Option<Rc<PgsOdsSegment>>
}

impl PgsDisplaySet {
    pub fn new() -> Self {
        PgsDisplaySet {
            pcs: None,
            wds: None,
            pds: None,
            ods: None
        }
    }

    pub(crate) fn clean(&mut self) {
        self.pcs = None;
        self.wds = None;
        self.pds = None;
        self.ods = None;
    }

    pub fn state(&self) -> PgsDisplaySetState {
        if self.pcs.is_some() && self.wds.is_some() {
            if self.pds.is_some() && self.ods.is_some() {
                return PgsDisplaySetState::Complete;
            }
            return PgsDisplaySetState::EmptyFrame;
        }
        PgsDisplaySetState::Incomplete
    }

    pub fn get_rle_image(&self) -> Result<&Vec<u8>> {
        if self.state() != PgsDisplaySetState::Complete {
            return Err(Error::IncompleteDisplaySet);
        }
        Ok(&self.ods.as_ref().unwrap().object_data)
    }

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