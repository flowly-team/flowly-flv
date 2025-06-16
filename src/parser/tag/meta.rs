use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    error::Error,
    parser::{FlvParser, Parser},
    reader::FlvReader,
    tag::meta::{MetaDataDate, MetaDataValue, MetaTag},
};

const OBJECT_END_MARKER: [u8; 3] = [0x00, 0x00, 0x09];

impl<E> Parser<E, MetaTag> for FlvParser {
    type Error = Error<E>;

    /// Parse script tag data.
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<MetaTag, Self::Error> {
        let _tag = reader.read_u8()?;
        let name = self.parse_meta_string(reader)?;

        // AMF arguments or object properties.
        Ok(MetaTag {
            name,
            value: self.parse(reader)?,
        })
    }
}

impl FlvParser {
    #[inline]
    fn parse_meta_string<E>(&mut self, reader: &mut impl FlvReader) -> Result<Bytes, Error<E>> {
        let len = reader.read_u16()? as usize;

        Ok(reader.read_to_bytes(len as usize)?)
    }

    fn parse_meta_object<E>(
        &mut self,
        reader: &mut impl FlvReader,
    ) -> Result<HashMap<Bytes, MetaDataValue>, Error<E>> {
        let mut props = HashMap::new();

        while reader.available() >= 3 && reader.peek(0..3)? != OBJECT_END_MARKER {
            props.insert(self.parse_meta_string(reader)?, self.parse(reader)?);
        }

        Ok(props)
    }
}

impl<E> Parser<E, MetaDataValue> for FlvParser {
    type Error = Error<E>;

    /// Parse script tag data value.
    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<MetaDataValue, Self::Error> {
        Ok(match reader.read_u8()? {
            0 => MetaDataValue::Number(reader.read_f64()?),
            1 => MetaDataValue::Boolean(reader.read_u8()? != 0),
            2 => MetaDataValue::String(self.parse_meta_string(reader)?),
            3 => MetaDataValue::Object(self.parse_meta_object(reader)?),
            5 => MetaDataValue::Null,
            6 => MetaDataValue::Undefined,
            7 => MetaDataValue::Reference(reader.read_u16()?),
            8 => {
                let _len = reader.read_u32()?;
                MetaDataValue::ECMAArray(self.parse_meta_object(reader)?)
            }
            10 => {
                let len = reader.read_u32()?;
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(self.parse(reader)?);
                }

                MetaDataValue::StrictArray(arr)
            }
            11 => MetaDataValue::Date(MetaDataDate {
                date_time: reader.read_f64()?,
                local_date_time_offset: reader.read_i16()?,
            }),
            12 => {
                let len = reader.read_u32()? as usize;

                MetaDataValue::LongString(reader.read_to_bytes(len)?)
            }
            id => MetaDataValue::Unknown(id),
        })
    }
}
