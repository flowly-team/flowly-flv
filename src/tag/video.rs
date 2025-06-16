use bytes::Bytes;
use flowly::Fourcc;

pub mod mpeg4_avc;

/// The tag data part of `video` FLV tag, including `tag data header` and `tag data body`.
#[derive(Clone, Debug, PartialEq)]
pub struct VideoTag {
    /// The header part of `video` FLV tag.
    pub header: VideoTagHeader, // 8 bits.

    /// The body part of `video` FLV tag.
    pub body: VideoTagBody,

    /// Track ID
    pub track_id: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VideoTagBody {
    pub pts_offset: i32,
    pub param_count: u32,
    pub nalus: Vec<Bytes>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum VideoPacketType {
    SequenceStart = 0,
    CodedFrames = 1,
    SequenceEnd = 2,

    /// CompositionTime Offset is implicitly set to zero. This optimization
    /// avoids transmitting an SI24 composition time value of zero over the wire.
    /// See the ExVideoTagBody section below for corresponding pseudocode.
    CodedFramesX = 3,

    /// ExVideoTagBody does not contain video data. Instead, it contains
    /// an AMF-encoded metadata. Refer to the Metadata Frame section for
    /// an illustration of its usage. For example, the metadata might include
    /// HDR information. This also enables future possibilities for expressing
    /// additional metadata meant for subsequent video sequences.
    ///
    /// If VideoPacketType.Metadata is present, the FrameType flags
    /// at the top of this table should be ignored.
    Metadata = 4,

    /// Carriage of bitstream in MPEG-2 TS format
    ///
    /// PacketTypeSequenceStart and PacketTypeMPEG2TSSequenceStart are mutually exclusive
    MPEG2TSSequenceStart = 5,

    /// Turns on video multitrack mode
    Multitrack = 6,

    /// ModEx is a special signal within the VideoPacketType enum that                 
    /// serves to both modify and extend the behavior of the current packet.           
    /// When this signal is encountered, it indicates the presence of                  
    /// additional modifiers or extensions, requiring further processing to            
    /// adjust or augment the packet's functionality. ModEx can be used to             
    /// introduce new capabilities or modify existing ones, such as                    
    /// enabling support for high-precision timestamps or other advanced               
    /// features that enhance the base packet structure.
    ModEx = 7,

    Unknown(u8),
}

impl From<u8> for VideoPacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => VideoPacketType::SequenceStart,
            1 => VideoPacketType::CodedFrames,
            2 => VideoPacketType::SequenceEnd,
            3 => VideoPacketType::CodedFramesX,
            4 => VideoPacketType::Metadata,
            5 => VideoPacketType::MPEG2TSSequenceStart,
            6 => VideoPacketType::Multitrack,
            7 => VideoPacketType::ModEx,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VideoPacketModExType {
    TimestampOffsetNano = 0,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AvMultitrackType {
    OneTrack = 0,
    ManyTracks = 1,
    ManyTracksManyCodecs = 2,
    Unknown(u8),
}

impl From<u8> for AvMultitrackType {
    fn from(value: u8) -> Self {
        match value {
            0 => AvMultitrackType::OneTrack,
            1 => AvMultitrackType::ManyTracks,
            2 => AvMultitrackType::ManyTracksManyCodecs,
            t => AvMultitrackType::Unknown(t),
        }
    }
}

/// The `tag data header` part of `video` FLV tag data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoTagHeader {
    pub frame_type: VideoFrameType,
    pub pkt_type: VideoPacketType,
    pub multitrack: bool,
    pub fourcc: Fourcc,
    pub has_body: bool,
    pub multitrack_type: AvMultitrackType,
    pub(crate) enhanced: bool,
}

impl From<u8> for VideoPacketModExType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::TimestampOffsetNano,
            v => Self::Unknown(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketExData {
    pub pkt_type: VideoPacketType,
    pub ex_type: VideoPacketModExType,
    pub dts_offset_ns: u32,
    pub ex_data: Bytes,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum VideoCommand {
    StartSeek = 0,
    EndSeek = 1,
    Unknown(u8), // 0x03..0xff = reserved
}

/// The type of video frame.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VideoFrameType {
    /// 1, Key frame.
    Key,

    /// 2, Inter frame.
    Inter,

    /// 3, DisposableInter frame.
    DisposableInter,

    /// 4, Generated frame.
    Generated,

    /// 5, Command frame.
    Command,

    /// Unknown frame.
    Unknown(u8),
}

impl From<u8> for VideoFrameType {
    fn from(value: u8) -> Self {
        match value {
            1 => VideoFrameType::Key,
            2 => VideoFrameType::Inter,
            3 => VideoFrameType::DisposableInter,
            4 => VideoFrameType::Generated,
            5 => VideoFrameType::Command,
            t => VideoFrameType::Unknown(t),
        }
    }
}

/// The code identifier of video.
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CodecID {
    /// 2, SorensonH263
    SorensonH263,

    /// 3, Screen1
    Screen1,

    /// 4, VP6
    VP6,

    /// 5, VP6Alpha
    VP6Alpha,

    /// 6, Screen2
    Screen2,

    /// 7, MPEG-4 Part 10 AVC / H.264
    AVC,

    /// 8, REAL H263
    RealH263,

    // 12, HEVC experimental non-standart codec
    Hevc,

    /// Unknown codec ID.
    Unknown(u8),
}

impl From<u8> for CodecID {
    fn from(value: u8) -> Self {
        match value {
            2 => CodecID::SorensonH263,
            3 => CodecID::Screen1,
            4 => CodecID::VP6,
            5 => CodecID::VP6Alpha,
            6 => CodecID::Screen2,
            7 => CodecID::AVC,
            8 => CodecID::RealH263,
            12 => CodecID::Hevc,
            c => CodecID::Unknown(c),
        }
    }
}

/// The type of AVC packet.
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AvcPacketType {
    /// 0, SequenceHeader.
    SequenceHeader,

    /// 1. NALU.
    NALU,

    /// 2, EndOfSequence.
    EndOfSequence,

    /// Unknown
    Unknown(u8),
}

impl From<u8> for AvcPacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => AvcPacketType::SequenceHeader,
            1 => AvcPacketType::NALU,
            2 => AvcPacketType::EndOfSequence,
            t => AvcPacketType::Unknown(t),
        }
    }
}
