use std::pin::pin;

use flowly_flv::{demux_flv_stream, tag::FlvTagData};
use futures::TryStreamExt;

#[tokio::main]
pub async fn main() {
    let file = tokio::fs::File::open("/home/andrey/demo/h264/test.flv")
        // let file = tokio::fs::File::open("/home/andrey/demo/h265/street.flv")
        // let file = tokio::fs::File::open("/home/andrey/demo/h264/terminator.flv")
        // let file = tokio::fs::File::open("/home/andrey/demo/vp9/forest.flv")
        .await
        .unwrap();

    let mut stream = pin!(demux_flv_stream(file, flowly_flv::DEMUX_ALL_TYPES));

    while let Some(tag) = stream.try_next().await.unwrap() {
        match tag.data {
            FlvTagData::Video(video) => {
                println!(
                    "{:?} {:?} {} {:?} {} {:?}",
                    &tag.header,
                    &video.header,
                    video.body.pts_offset,
                    video.body.param_count,
                    video.body.nalus.len(),
                    video
                        .body
                        .nalus
                        .iter()
                        .map(|x| &x[0..4])
                        .collect::<Vec<_>>()
                );
            }

            FlvTagData::Meta(meta) => {
                println!("{:#?}", meta);
            }

            _ => (),
        }
    }
}
