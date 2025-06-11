use crate::{
    error::Error,
    fourcc::Fourcc,
    parser::{FlvParser, Parser},
    reader::FlvReader,
    tag::video::{
        AvMultitrackType, AvcPacketType, CodecID, PacketExData, VideoFrameType, VideoPacketType,
        VideoTag, VideoTagBody, VideoTagHeader,
        mpeg4_avc::{Mpeg4AvcNALUSeq, Mpeg4AvcRecord},
    },
};

impl Parser<VideoTagHeader> for FlvParser {
    type Error = Error;

    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<VideoTagHeader, Self::Error> {
        let header = reader.read_u8()?;
        let frame_type = VideoFrameType::from(header & 0x70 >> 4);
        let codec_id = CodecID::from(header & 0x0F);
        let enhanced = (header >> 7) & 1 > 0;

        if enhanced {
            let mut fourcc = None;
            let mut multitrack = false;
            let mut has_body = true;
            let mut multitrack_type = AvMultitrackType::OneTrack;
            let mut video_command = None;
            let mut pkt_type = VideoPacketType::from(header & 0x0F);

            while let VideoPacketType::ModEx = pkt_type {
                let ex_data: PacketExData = self.parse(reader)?;

                pkt_type = ex_data.pkt_type;
            }

            if pkt_type != VideoPacketType::Metadata && frame_type == VideoFrameType::Command {
                video_command = Some(reader.read_u8()?);

                //   ExVideoTagBody has no payload if we got here.
                //   Set boolean to not try to process the video body.
                has_body = false;
            } else if pkt_type == VideoPacketType::Multitrack {
                multitrack = true;
                // Fetch VideoPacketType for all video tracks in the video message.
                // This fetch MUST not result in a VideoPacketType.Multitrack
                multitrack_type = AvMultitrackType::from(reader.read_u8()?);

                if multitrack_type != AvMultitrackType::ManyTracksManyCodecs {
                    fourcc = Some(Fourcc::from(reader.read_u32()?));
                }
            } else {
                fourcc = Some(Fourcc::from(reader.read_u32()?));
            }

            Ok(VideoTagHeader::Enhanced {
                pkt_type,
                multitrack,
                has_body,
                video_command,
                multitrack_type,
                fourcc,
            })
        } else {
            Ok(VideoTagHeader::Original {
                frame_type: VideoFrameType::from(frame_type),
                codec_id: CodecID::from(codec_id),
            })
        }
    }
}

impl Parser<VideoTag> for FlvParser {
    type Error = Error;

    fn parse(&mut self, reader: &mut impl FlvReader) -> Result<VideoTag, Self::Error> {
        let header: VideoTagHeader = self.parse(reader)?;

        Ok(match header {
            VideoTagHeader::Enhanced {
                pkt_type,
                multitrack: _,
                fourcc,
                video_command: _,
                has_body: _,
                multitrack_type,
            } => {
                let fourcc = if multitrack_type == AvMultitrackType::ManyTracksManyCodecs {
                    // Each track has a codec assigned to it. Fetch the FOURCC for the next track.
                    Fourcc::from(reader.read_u32()?)
                } else {
                    fourcc.unwrap_or_default()
                };

                // Track Ordering:
                //
                // For identifying the highest priority (a.k.a., default track)
                // or highest quality track, it is RECOMMENDED to use trackId
                // set to zero. For tracks of lesser priority or quality, use
                // multiple instances of trackId with ascending numerical values.
                // The concept of priority or quality can have multiple
                // interpretations, including but not limited to bitrate,
                // resolution, default angle, and language. This recommendation
                // serves as a guideline intended to standardize track numbering
                // across various applications.
                let track_id = reader.read_u8()?;
                let mut pts_offset = 0;
                let mut params = None;
                let mut nalus = Vec::new();

                if multitrack_type != AvMultitrackType::OneTrack {
                    // The `sizeOfVideoTrack` specifies the size in bytes of the
                    // current track that is being processed. This size starts
                    // counting immediately after the position where the `sizeOfVideoTrack`
                    // value is located. You can use this value as an offset to locate the
                    // next video track in a multitrack system. The data pointer is
                    // positioned immediately after this field. Depending on the MultiTrack
                    // type, the offset points to either a `fourCc` or a `trackId.`
                    let _size_of_video_track = reader.read_u24()?;
                }

                match pkt_type {
                    VideoPacketType::SequenceStart => {
                        // if self.mpeg4_avc_parser.is_none() {
                        //     self.mpeg4_avc_parser = Some(Default::default());
                        // }

                        // let x: Mpeg4AvcRecord =
                        //     self.mpeg4_avc_parser.as_mut().unwrap().parse(reader)?;

                        // params = Some([x.sps, x.pps].concat());
                    }
                    VideoPacketType::CodedFrames => {
                        match fourcc {
                            Fourcc::VIDEO_AVC => {
                                // See ISO/IEC 14496-12:2015, 8.6.1 for the description of the composition
                                // time offset. The offset in an FLV file is always in milliseconds
                                pts_offset = reader.read_i24()? as i64;

                                // let x: Mpeg4AvcNALUSeq =
                                //     self.mpeg4_avc_parser.as_mut().unwrap().parse(reader)?;

                                // nalus = x.nalus;
                                nalus = vec![reader.read_to_end()?];
                            }
                            Fourcc::VIDEO_HEVC => {
                                // body contains a configuration record to start the sequence.
                                // See ISO/IEC 14496-15:2022, 8.3.3.2 for the description of
                                // HEVCDecoderConfigurationRecord
                                pts_offset = reader.read_i24()? as i64;
                                // let x: Mpeg4AvcNALUSeq =
                                //     self.mpeg4_avc_parser.as_mut().unwrap().parse(reader)?;

                                // nalus = x.nalus;
                                nalus = vec![reader.read_to_end()?];
                            }
                            _ => {
                                nalus = vec![reader.read_to_end()?];
                            }
                        }
                    }
                    VideoPacketType::SequenceEnd => {}
                    VideoPacketType::CodedFramesX => {
                        nalus = vec![reader.read_to_end()?];
                    }
                    VideoPacketType::Metadata => {}
                    VideoPacketType::MPEG2TSSequenceStart => {}
                    VideoPacketType::Multitrack => {}
                    VideoPacketType::ModEx => {}
                }

                VideoTag {
                    header,
                    track_id,
                    body: VideoTagBody {
                        pts_offset,
                        params,
                        nalus,
                    },
                }
            }
            VideoTagHeader::Original {
                frame_type: _,
                codec_id,
            } => {
                let mut pts_offset = 0;
                let mut params = None;
                let mut nalus = Vec::new();

                if codec_id == CodecID::AVC || codec_id == CodecID::Hevc {
                    if self.mpeg4_avc_parser.is_none() {
                        self.mpeg4_avc_parser = Some(Default::default());
                        println!("create avc parser");
                    }

                    let packet_type = AvcPacketType::from(reader.read_u8()?);
                    pts_offset = reader.read_i24()? as i64;

                    match packet_type {
                        AvcPacketType::SequenceHeader => {
                            let x: Mpeg4AvcRecord =
                                self.mpeg4_avc_parser.as_mut().unwrap().parse(reader)?;

                            params = Some([x.sps, x.pps].concat());
                        }
                        AvcPacketType::NALU => {
                            let x: Mpeg4AvcNALUSeq =
                                self.mpeg4_avc_parser.as_mut().unwrap().parse(reader)?;

                            nalus = x.nalus;
                        }
                        _ => {}
                    }
                } else {
                    // effectively not implemented: just pass raw paylod
                    nalus = vec![reader.read_to_end()?];
                }

                VideoTag {
                    header,
                    track_id: 0,
                    body: VideoTagBody {
                        pts_offset,
                        params,
                        nalus,
                    },
                }
            }
        })
    }
}
