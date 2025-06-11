mod audio;
mod ex_data;
mod meta;
mod video;

use crate::{
    error::Error,
    reader::FlvReader,
    tag::{FlvTag, FlvTagData, FlvTagHeader, FlvTagType},
};

use super::{FlvParser, Parser};

impl Parser<FlvTagHeader> for FlvParser {
    type Error = Error;

    /// Parse FLV tag header.
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<FlvTagHeader, Self::Error> {
        let flags = reader.read_u8()?;

        let tag_type = FlvTagType::from(flags & 0b11111);

        // The size of the tag's data part
        let data_size = reader.read_u24()?;

        // The timestamp (in milliseconds) of the tag
        let timestamp = reader.read_u24()?;

        // Extension of the timestamp field to form a SI32 value
        let timestamp_extended = reader.read_u8()?;

        // The id of stream
        let stream_id = reader.read_u24()?;

        Ok(FlvTagHeader {
            tag_type,
            data_size,
            timestamp: (u32::from(timestamp_extended) << 24) + timestamp,
            stream_id,
        })
    }
}

impl Parser<FlvTag> for FlvParser {
    type Error = Error;

    /// Parse FLV tag data.
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<FlvTag, Self::Error> {
        let header: FlvTagHeader = self.parse(reader)?;

        Ok(FlvTag {
            header,
            data: match header.tag_type {
                FlvTagType::Audio => FlvTagData::Audio(self.parse(reader)?),
                FlvTagType::Video => FlvTagData::Video(self.parse(reader)?),
                FlvTagType::Meta => FlvTagData::Meta(self.parse(reader)?),
                FlvTagType::Unknown => FlvTagData::Unknown,
            },
        })
    }
}

impl FlvParser {
    /// Parse FLV tag data.
    pub(crate) fn parse_flv_data(
        &mut self,
        reader: &mut impl FlvReader,
        tag_type: FlvTagType,
    ) -> Result<FlvTagData, Error> {
        Ok(match tag_type {
            FlvTagType::Audio => FlvTagData::Audio(self.parse(reader)?),
            FlvTagType::Video => FlvTagData::Video(self.parse(reader)?),
            FlvTagType::Meta => FlvTagData::Meta(self.parse(reader)?),
            FlvTagType::Unknown => FlvTagData::Unknown,
        })
    }
}
