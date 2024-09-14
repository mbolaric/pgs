//! This module defines the `PgsParser` struct and its associated methods for parsing and handling PGS (Presentation Graphics Stream) files.

use log::{debug, error, trace};

use crate::{pgs_reader::PgsReader, pgs_segment::PgsSegment, Error, PgsDisplaySet, PgsFile, PgsOdsSegment, PgsPcsSegment, PgsPdsSegment, PgsSegmentHeader, PgsSegmentType, PgsWdsSegment, Result};

/// A parser for PGS files.
///
/// This struct is responsible for parsing PGS files into segments and creating display sets from those segments.
///
/// # Fields
/// - `sup_file_path`: The path to the SUP file to be parsed.
/// - `segments`: A vector storing the parsed PGS segments.
/// - `display_sets`: A vector of display sets created from the parsed segments.
#[derive(Debug)]
pub struct PgsParser<'a> {
    sup_file_path: &'a str,
    segments: Vec<PgsSegment>,
    display_sets: Vec<PgsDisplaySet>
}

impl<'a> PgsParser<'a> {
    /// Creates a new `PgsParser` instance.
    ///
    /// # Arguments
    /// * `sup_file_path` - The path to the SUP file to be parsed.
    ///
    /// # Returns
    /// A new `PgsParser` instance with empty segments and display sets.
    fn new(sup_file_path: &'a str) -> Self {
        PgsParser {
            segments: Vec::new(),
            display_sets: Vec::new(),
            sup_file_path
        }
    }

    /// Returns a reference to the vector of display sets.
    ///
    /// # Returns
    /// A reference to the `Vec<PgsDisplaySet>`.
    pub fn get_display_sets(&self) -> &Vec<PgsDisplaySet> {
        self.display_sets.as_ref()
    }

    /// Reads and parses a segment from the given file.
    ///
    /// # Arguments
    /// * `file` - A mutable reference to the `PgsFile` from which to read the segment.
    ///
    /// # Returns
    /// A `Result` containing either a `PgsSegment` or an `Error` if reading or parsing fails.
    fn read_segment(&mut self, file: &mut PgsFile) -> Result<PgsSegment> {
        let buffer = file.read_n_bytes::<13>()?;
        let header = PgsSegmentHeader::from_data(&buffer)?;
        
        let pg = header.segment_type;
        if pg == PgsSegmentType::ERR {
            return Err(Error::ReadInvalidSegment);
        }
    
        if header.segment_type == PgsSegmentType::END {
            return Ok(PgsSegment::End);
        }

        let mut buffer = vec![0; header.segment_length as usize];
        file.read_bytes(buffer.as_mut_slice())?;
    
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
    
    /// Parses the PGS file and reads all segments.
    ///
    /// This method continuously reads segments from the file until the end of the file is reached.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the parsing process.    
    fn parse_inner(&mut self) -> Result<()> {
        let mut file = PgsReader::open(self.sup_file_path)?;
        debug!("{:?}", file);
    
        loop {
            match self.read_segment(&mut file) {
                Ok(segment) => {
                    trace!("{:?}", segment);
                    self.segments.push(segment);
                    if file.is_eof()? {
                        return Ok(());
                    }
                },
                Err(error) => {
                    error!("{:?}", error);
                    return Err(error)                   
                }
            }
        }
    }

    /// Creates display sets from the parsed segments.
    ///
    /// This method processes the segments to construct display sets and adds them to the `display_sets` vector.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the display set creation process.
    fn create_display_sets(&mut self) -> Result<()> {
        let mut ds = PgsDisplaySet::new();
        self.segments.iter().for_each(|segment| {
            match segment {
                PgsSegment::Pcs(pcs) => ds.pcs = Some(pcs.clone()),
                PgsSegment::Wds(wds) => ds.wds = Some(wds.clone()),
                PgsSegment::Ods(ods) => ds.ods = Some(ods.clone()),
                PgsSegment::Pds(pds) => ds.pds = Some(pds.clone()),
                PgsSegment::End => {
                    self.display_sets.push(ds.clone());
                    ds.clean();
                }
            }
        });

        Ok(())
    }

    /// Parses a PGS file and creates display sets.
    ///
    /// # Arguments
    /// * `sup_file_path` - The path to the SUP file to be parsed.
    ///
    /// # Returns
    /// A `Result` containing either the `PgsParser` instance or an `Error` if the parsing fails.
    pub fn parse(sup_file_path: &'a str) -> Result<PgsParser<'a>> {
        let mut parser = PgsParser::new(sup_file_path);
        parser.parse_inner()?;
        parser.create_display_sets()?;
        Ok(parser)
    }
}