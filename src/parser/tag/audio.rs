use crate::{
    error::Error,
    parser::{FlvParser, Parser},
    reader::FlvReader,
    tag::audio::{
        AudioTag, AudioTagBody, AudioTagHeader, SoundFormat, SoundRate, SoundSize, SoundType,
    },
};

impl<E> Parser<E, AudioTagHeader> for FlvParser {
    type Error = Error<E>;

    /// Parse audio tag data header.
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<AudioTagHeader, Self::Error> {
        let header = reader.read_u8()?;

        Ok(AudioTagHeader {
            sound_format: SoundFormat::from(header >> 4),
            sound_rate: SoundRate::from((header >> 2) & 0b11),
            sound_size: SoundSize::from((header >> 1) & 1),
            sound_type: SoundType::from(header & 1),
        })
    }
}

impl<E> Parser<E, AudioTagBody> for FlvParser {
    type Error = Error<E>;

    /// Parse audio tag data body.
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<AudioTagBody, Self::Error> {
        Ok(AudioTagBody {
            data: reader.read_to_end()?,
        })
    }
}

impl<E> Parser<E, AudioTag> for FlvParser {
    type Error = Error<E>;
    /// Parse audio tag data.
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<AudioTag, Self::Error> {
        let header: AudioTagHeader = self.parse(reader)?;
        let body = self.parse(reader)?;

        Ok(AudioTag { header, body })
    }
}
