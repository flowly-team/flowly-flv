use bytes::Bytes;

use crate::{error::Error, parser::Parser, reader::FlvReader};

#[derive(Default, Clone, Debug)]
pub struct Mpeg4AvcRecord {
    pub profile: u8,
    pub compatibility: u8,
    pub level: u8,
    pub nalu_length: u8,
    pub sps: Vec<Bytes>,
    pub pps: Vec<Bytes>,
}

#[derive(Clone, Debug)]
pub struct Mpeg4AvcParser {
    pub nalu_length: u8,
}

impl Default for Mpeg4AvcParser {
    fn default() -> Self {
        Self {
            nalu_length: Default::default(),
        }
    }
}

impl<E> Parser<E, Mpeg4AvcRecord> for Mpeg4AvcParser {
    type Error = Error<E>;

    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<Mpeg4AvcRecord, Self::Error> {
        let (mut pps, mut sps) = (Vec::new(), Vec::new());

        /*version */
        reader.read_u8()?;

        /*avc profile*/
        let profile = reader.read_u8()?;

        /*avc compatibility*/
        let compatibility = reader.read_u8()?;

        /*avc level*/
        let level = reader.read_u8()?;

        /*nalu length*/
        let nalu_length = (reader.read_u8()? & 0x03) + 1;

        self.nalu_length = nalu_length;

        /*number of SPS NALUs */
        let nb_sps = reader.read_u8()? & 0x1F;
        for _ in 0..nb_sps as usize {
            /*SPS size*/
            let sps_data_size = reader.read_u16()?;
            let sps_data = reader.read_to_bytes(sps_data_size as usize)?;

            sps.push(sps_data);
        }

        /*number of PPS NALUs*/
        let nb_pps = reader.read_u8()?;
        for _ in 0..nb_pps as usize {
            let pps_data_size = reader.read_u16()?;
            let pps_data = reader.read_to_bytes(pps_data_size as usize)?;
            pps.push(pps_data);
        }

        Ok(Mpeg4AvcRecord {
            profile,
            compatibility,
            level,
            nalu_length,
            sps,
            pps,
        })
    }
}

impl Mpeg4AvcParser {
    #[inline]
    fn read_nalu_size(&mut self, reader: &mut impl FlvReader) -> std::io::Result<usize> {
        Ok(match self.nalu_length {
            1 => reader.read_u8()? as _,
            2 => reader.read_u16()? as _,
            3 => reader.read_u24()? as _,
            4 => reader.read_u32()? as _,
            _ => unreachable!(),
        })
    }
}

pub struct Mpeg4AvcNALUSeq {
    pub(crate) nalus: Vec<Bytes>,
}

impl<E> Parser<E, Mpeg4AvcNALUSeq> for Mpeg4AvcParser {
    type Error = Error<E>;

    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<Mpeg4AvcNALUSeq, Self::Error> {
        let mut nalus = Vec::new();
        while let Ok(size) = self.read_nalu_size(reader) {
            nalus.push(reader.read_to_bytes(size)?);
        }

        Ok(Mpeg4AvcNALUSeq { nalus })
    }
}
