use log::{debug, error};

use pgs::{Error, PgsFile, PgsOdsSegment, PgsPcsSegment, PgsPdsSegment, PgsReader, PgsSegment, PgsSegmentHeader, PgsSegmentType, PgsWdsSegment, Result};

mod helpers;
use crate::helpers::init_logging;

fn read_segment(file: &mut PgsFile) -> Result<PgsSegment> {
    let buffer = file.read_n_bytes::<13>()?;
    let header = PgsSegmentHeader::from_data(&buffer)?;
    
    let pg = header.segment_type;
    if pg == PgsSegmentType::ERR {
        return Err(Error::ReadInvalidSegment);
    }

    let mut buffer = Vec::with_capacity(header.segment_length as usize);
    buffer.resize(header.segment_length as usize, 0);
    file.read_bytes(&mut buffer.as_mut_slice())?;

    let segment = match header.segment_type {
        PgsSegmentType::PCS => {            
            PgsSegment::Pcs(PgsPcsSegment::from_data(header, &buffer)?)
        },
        PgsSegmentType::WDS => {
            PgsSegment::Wds(PgsWdsSegment::from_data(header, &buffer)?)
        },
        PgsSegmentType::PDS => {
            PgsSegment::Pds(PgsPdsSegment::from_data(header, &buffer)?)
        },
        PgsSegmentType::ODS => {
            PgsSegment::Ods(PgsOdsSegment::from_data(header, &buffer)?)
        },
        _ => PgsSegment::End
    };

    Ok(segment)
}

pub fn main() {
    init_logging();
    
    let mut segments: Vec<PgsSegment> = Vec::new();

    match PgsReader::open("./examples/data/BluRay.sup").as_mut() {
        Ok(file) => {
            debug!("{:?}", file);

            let mut i = 1;
            let mut running = true;
            while running {
                match read_segment(file) {
                    Ok(segment) => {
                        i += 1;
                        debug!("{:?}: {:?}", i, segment);
                        segments.push(segment);
                    },
                    Err(error) => {
                        debug!("{:?}", error);
                        running = false;
                    }
                }
            }
        },
        Err(error) => {
            error!("{:?}", error)   
        }      
    }
}