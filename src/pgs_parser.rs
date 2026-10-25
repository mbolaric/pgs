//! This module defines the `PgsParser` struct and its associated methods for parsing and handling PGS (Presentation Graphics Stream) files.

use log::{debug, error, trace};

use crate::{
    pgs_reader::PgsReader, pgs_segment::PgsSegment, Error, PgsDisplaySet, PgsFile, PgsOdsSegment,
    PgsPcsSegment, PgsPdsSegment, PgsSegmentHeader, PgsSegmentType, PgsWdsSegment, Result,
};

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
    display_sets: Vec<PgsDisplaySet>,
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
            sup_file_path,
        }
    }

    /// Returns a reference to the vector of display sets.
    ///
    /// # Returns
    /// A reference to the `Vec<PgsDisplaySet>`.
    pub fn get_display_sets(&self) -> &Vec<PgsDisplaySet> {
        self.display_sets.as_ref()
    }

    fn parse_segment_payload(header: PgsSegmentHeader, buffer: &[u8]) -> Result<PgsSegment> {
        let segment = match header.segment_type {
            PgsSegmentType::PCS => PgsSegment::Pcs(PgsPcsSegment::from_data(header, buffer)?),
            PgsSegmentType::WDS => PgsSegment::Wds(PgsWdsSegment::from_data(header, buffer)?),
            PgsSegmentType::PDS => PgsSegment::Pds(PgsPdsSegment::from_data(header, buffer)?),
            PgsSegmentType::ODS => PgsSegment::Ods(PgsOdsSegment::from_data(header, buffer)?),
            _ => PgsSegment::End,
        };

        Ok(segment)
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

        Self::parse_segment_payload(header, &buffer)
    }

    fn read_segment_from_bytes(data: &[u8], offset: &mut usize) -> Result<PgsSegment> {
        if *offset + 13 > data.len() {
            return Err(Error::File(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "end of file",
            )));
        }

        let header = PgsSegmentHeader::from_data(&data[*offset..*offset + 13])?;
        *offset += 13;

        let pg = header.segment_type;
        if pg == PgsSegmentType::ERR {
            return Err(Error::ReadInvalidSegment);
        }

        if header.segment_type == PgsSegmentType::END {
            return Ok(PgsSegment::End);
        }

        let length = header.segment_length as usize;
        if *offset + length > data.len() {
            return Err(Error::File(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "end of file",
            )));
        }

        let buffer = &data[*offset..*offset + length];
        *offset += length;

        Self::parse_segment_payload(header, buffer)
    }

    fn parse_bytes_inner(&mut self, data: &[u8]) -> Result<()> {
        let mut offset = 0;
        while offset < data.len() {
            match Self::read_segment_from_bytes(data, &mut offset) {
                Ok(segment) => {
                    trace!("{:?}", segment);
                    self.segments.push(segment);
                }
                Err(error) => {
                    error!("{:?}", error);
                    return Err(error);
                }
            }
        }
        Ok(())
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
                }
                Err(error) => {
                    error!("{:?}", error);
                    return Err(error);
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
        self.segments.iter().for_each(|segment| match segment {
            PgsSegment::Pcs(pcs) => ds.pcs = Some(pcs.clone()),
            PgsSegment::Wds(wds) => ds.wds = Some(wds.clone()),
            PgsSegment::Ods(ods) => ds.ods = Some(ods.clone()),
            PgsSegment::Pds(pds) => ds.pds = Some(pds.clone()),
            PgsSegment::End => {
                self.display_sets.push(ds.clone());
                ds.clean();
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

    /// Parses PGS data from a byte slice and creates display sets.
    ///
    /// # Arguments
    /// * `bytes` - The byte slice or container representing the raw PGS data.
    ///
    /// # Returns
    /// A `Result` containing either the `PgsParser` instance or an `Error` if the parsing fails.
    ///
    /// # Example
    /// ```rust
    /// use pgs_parse::PgsParser;
    ///
    /// let bytes: Vec<u8> = Vec::new();
    /// let parser = PgsParser::parse_bytes(&bytes).unwrap();
    /// assert_eq!(parser.get_display_sets().len(), 0);
    /// ```
    pub fn parse_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let mut parser = PgsParser {
            sup_file_path: "",
            segments: Vec::new(),
            display_sets: Vec::new(),
        };
        parser.parse_bytes_inner(bytes.as_ref())?;
        parser.create_display_sets()?;
        Ok(parser)
    }
}
