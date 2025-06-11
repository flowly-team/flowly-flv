use crate::{
    error::Error,
    parser::{FlvParser, Parser},
    reader::FlvReader,
    tag::video::{PacketExData, VideoPacketModExType, VideoPacketType},
};

impl Parser<PacketExData> for FlvParser {
    type Error = Error;

    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<PacketExData, Error> {
        // Determine the size of the packet ModEx data (ranging from 1 to 256 bytes)
        let mut ex_size = reader.read_u8()? as u16 + 1;

        // If maximum 8-bit size is not sufficient, use a 16-bit value
        if ex_size == 256 {
            ex_size = reader.read_u16()? + 1;
        }

        // Fetch the packet ModEx data based on its determined size
        let mut ex_data = reader.read_to_bytes(ex_size as usize)?;

        let ex_hdr = reader.read_u8()?;

        // fetch the VideoPacketOptionType
        let ex_type = VideoPacketModExType::from(ex_hdr & 0xf0);

        // fetch videoPacketType
        let pkt_type = ex_hdr & 0x0f;

        let mut dts_offset_ns = 0;

        if let VideoPacketModExType::TimestampOffsetNano = ex_type {
            // This block processes TimestampOffsetNano to enhance RTMP timescale
            // accuracy and compatibility with formats like MP4, M2TS, and Safari's
            // Media Source Extensions. It ensures precise synchronization without
            // altering core RTMP timestamps, applying only to the current media
            // message. These adjustments enhance synchronization and timing
            // accuracy in media messages while preserving the core RTMP timestamp
            // integrity.
            //
            // NOTE:
            // - 1 millisecond (ms) = 1,000,000 nanoseconds (ns).
            // - Maximum value representable with 20 bits is 1,048,575 ns
            //   (just over 1 ms), allowing precise sub-millisecond adjustments.
            // - modExData must be at least 3 bytes, storing values up to 999,999 ns.
            if ex_size != 3 {
                log::warn!("Invalid ModEx size for Type TimestampOffsetNano!");
            } else {
                dts_offset_ns = (&mut ex_data).read_u24()?;
            }
        } else {
            log::info!("Unknown ModEx type: {:?}", ex_type);
        }

        Ok(PacketExData {
            dts_offset_ns,
            ex_data,
            pkt_type: VideoPacketType::from(pkt_type),
            ex_type,
        })
    }
}
