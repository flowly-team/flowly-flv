use std::pin::pin;

use bytes::Bytes;
use error::Error;
use flowly::{Fourcc, Frame, FrameFlags, Service};
use futures::{Stream, TryStreamExt};
use header::FlvHeader;
use parser::{FlvParser, Parser};
use tag::{FlvTag, FlvTagHeader, FlvTagType};
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use tokio_util::io::StreamReader;

pub mod error;
pub mod header;
pub mod parser;
pub mod reader;
pub mod tag;

pub const DEMUX_ALL_TYPES: u64 = -1i64 as u64;

#[cfg(feature = "serde")]
pub mod serde;

pub struct FlvFrame {
    dts: u64,
    track_id: u32,
    flags: flowly::FrameFlags,
    pts_offset: i32,
    codec: Fourcc,
    params_count: u32,
    payload: Vec<Bytes>,
}

impl Frame for FlvFrame {
    fn pts(&self) -> i64 {
        self.dts as i64 + self.pts_offset as i64
    }

    fn dts(&self) -> u64 {
        self.dts
    }

    fn timestamp(&self) -> Option<u64> {
        None
    }

    fn codec(&self) -> flowly::Fourcc {
        self.codec
    }

    fn flags(&self) -> flowly::FrameFlags {
        self.flags
    }

    fn track(&self) -> u32 {
        self.track_id
    }

    fn params(&self) -> impl Iterator<Item = &[u8]> {
        self.payload
            .iter()
            .map(|x| x.as_ref())
            .take(self.params_count as usize)
    }

    fn units(&self) -> impl Iterator<Item = &[u8]> {
        self.payload
            .iter()
            .map(|x| x.as_ref())
            .skip(self.params_count as usize)
    }
}

#[inline]
pub fn demux_flv_stream<R: AsyncRead>(
    reader: R,
    tag_types: u64,
) -> impl Stream<Item = Result<FlvTag, error::Error>> {
    demux_flv_stream_inner(reader, tag_types)
}

#[derive(Debug)]
pub struct FlvDemuxer {
    flags_filter: FrameFlags,
    tracks_filter: u64,
}

impl FlvDemuxer {
    pub fn new(flags_filter: FrameFlags, tracks_filter: u64) -> Self {
        Self {
            flags_filter,
            tracks_filter,
        }
    }
}

impl Default for FlvDemuxer {
    fn default() -> Self {
        Self {
            flags_filter: FrameFlags::VIDEO_STREAM,
            tracks_filter: !0,
        }
    }
}

impl<F: AsRef<[u8]> + Send + 'static, E: std::error::Error + Send + Sync + 'static>
    Service<Result<F, E>> for FlvDemuxer
{
    type Out = Result<FlvFrame, Error<E>>;

    fn handle(
        self,
        input: impl Stream<Item = Result<F, E>> + Send,
    ) -> impl Stream<Item = Self::Out> + Send {
        let mut tag_type_filter = 0u64;

        if self.flags_filter.contains(FrameFlags::AUDIO_STREAM) {
            tag_type_filter |= 0b1 << u8::from(FlvTagType::Audio);
        }

        if self.flags_filter.contains(FrameFlags::VIDEO_STREAM) {
            tag_type_filter |= 0b1 << u8::from(FlvTagType::Video);
        }

        if self.flags_filter.contains(FrameFlags::METADATA_STREAM) {
            tag_type_filter |= 0b1 << u8::from(FlvTagType::Metadata);
        }

        let reader = StreamReader::new(input.map_ok(std::io::Cursor::new).map_err(Error::Other));

        demux_flv_stream_inner(reader, tag_type_filter).try_filter_map(move |tag| async move {
            Ok(match tag.data {
                tag::FlvTagData::Video(vtag) => {
                    if vtag.track_id < 64 && (self.tracks_filter << vtag.track_id) == 0 {
                        None
                    } else {
                        Some(FlvFrame {
                            dts: tag.header.timestamp as u64 * 1000,
                            track_id: vtag.track_id as _,
                            flags: FrameFlags::empty(),
                            pts_offset: vtag.body.pts_offset * 1000,
                            codec: vtag.header.fourcc,
                            params_count: vtag.body.param_count,
                            payload: vtag.body.nalus,
                        })
                    }
                }
                tag::FlvTagData::Audio(_) => None,
                tag::FlvTagData::Meta(_) => None,
                tag::FlvTagData::Unknown => None,
            })
        })
    }
}

fn demux_flv_stream_inner<E, R: AsyncRead>(
    reader: R,
    tag_types: u64,
) -> impl Stream<Item = Result<FlvTag, error::Error<E>>> {
    async_stream::stream! {
        let mut buff = vec![0u8; 1024];
        let mut reader = pin!(BufReader::new(reader));
        let mut parser = FlvParser::default();

        // reading flv header
        reader.read_exact(&mut buff[0..9]).await?;
        let header: FlvHeader = parser.parse(&mut &buff[0..9])?;

        // skipping offset if present
        if header.remaining > 0 {
            if buff.len() < header.remaining as usize {
                buff.resize(header.remaining as usize, 0);
            }

            reader.read_exact(&mut buff[0..header.remaining as usize]).await?;
        }

        while let Ok(_) = reader.read_u32().await {
            match reader.read_exact(&mut buff[0..11]).await {
                Ok(_) => (),
                Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(err) => yield Err(err.into())
            };

            let header: FlvTagHeader = parser.parse(&mut &buff[0..11])?;
            if buff.len() < header.data_size as usize {
                buff.resize(header.data_size as usize, 0);
            }

            match reader.read_exact(&mut buff[0..header.data_size as usize]).await {
                Ok(_) => (),
                Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(err) => yield Err(err.into())
            }

            if tag_types & (1u64 << u8::from(header.tag_type)) > 0 {
                let mut tag_data = Bytes::from(buff[0..header.data_size as usize].to_vec());

                yield Ok(FlvTag {
                    data: parser.parse_flv_data(&mut tag_data, header.tag_type)?,
                    header,
                });
            }
        }

    }
}
