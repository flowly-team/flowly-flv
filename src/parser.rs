use crate::{
    error::Error, header::FlvHeader, reader::FlvReader, tag::video::mpeg4_avc::Mpeg4AvcParser,
};

mod tag;

pub trait Parser<T> {
    type Error;
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<T, Self::Error>;
}

#[derive(Debug, Default, Clone)]
pub struct FlvParser {
    mpeg4_avc_parser: Option<Mpeg4AvcParser>,
}

impl Parser<FlvHeader> for FlvParser {
    type Error = Error;

    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<FlvHeader, Self::Error> {
        let mut signature = [0u8; 3];
        reader.read_to_slice(&mut signature)?;

        if &signature != b"FLV" {
            return Err(Error::InvalidSignature);
        }

        let version = reader.read_u8()?;
        let flags = reader.read_u8()?;

        let is_audio_present = (flags & 0b00000100) != 0;
        let is_video_present = (flags & 0b00000001) != 0;

        let data_offset = reader.read_u32()?;

        let remaining = data_offset.saturating_sub(9); // header size 3{FLV} + 1{version} + 1{flags} + 4{data_offset}

        Ok(FlvHeader {
            version,
            is_audio_present,
            is_video_present,
            remaining,
        })
    }
}
