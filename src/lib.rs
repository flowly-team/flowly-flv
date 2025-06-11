use std::pin::pin;

use bytes::Bytes;
use futures::Stream;
use header::FlvHeader;
use parser::{FlvParser, Parser};
use tag::{FlvTag, FlvTagHeader};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};

pub mod error;
pub mod fourcc;
pub mod header;
pub mod parser;
pub mod reader;
pub mod tag;

#[cfg(feature = "serde")]
pub mod serde;

pub fn demux_flv_stream<R: AsyncRead>(
    reader: R,
) -> impl Stream<Item = Result<FlvTag, error::Error>> {
    async_stream::stream! {
        let mut buff = vec![0u8; 1024];
        let mut reader = pin!(BufReader::new(reader));
        let mut parser = FlvParser::default();

        // reading flv header
        reader.read_exact(&mut buff[0..9]).await?;
        let header: FlvHeader = parser.parse(&mut &buff[0..9])?;

        // skipping offset if present
        if header.remaining > 0 {
            reader.consume(header.remaining as _);
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

            let mut tag_data = Bytes::from(buff[0..header.data_size as usize].to_vec());

            yield Ok(FlvTag {
                data: parser.parse_flv_data(&mut tag_data, header.tag_type)?,
                header,
            });
        }

    }
}
