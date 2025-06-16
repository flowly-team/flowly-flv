pub mod audio;
pub mod meta;
pub mod video;

/// The FLV tag has three types: `script tag`, `audio tag` and `video tag`.
/// Each tag contains tag header and tag data.
/// The structure of each type of tag header is the same.
#[derive(Clone, Debug, PartialEq)]
pub struct FlvTag {
    /// The header part of FLV tag.
    pub header: FlvTagHeader,

    /// Data specific for each media type:
    /// * 8 = audio data.
    /// * 9 = video data.
    /// * 18 = script data.
    pub data: FlvTagData,
}

/// The type of FLV tag.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FlvTagType {
    /// Audio tag type.
    Audio,

    /// Video tag type.
    Video,

    /// Script tag type.
    Metadata,

    // Unknown
    Unknown(u8),
}

impl From<u8> for FlvTagType {
    fn from(value: u8) -> Self {
        match value {
            8 => FlvTagType::Audio,
            9 => FlvTagType::Video,
            18 => FlvTagType::Metadata,
            t => FlvTagType::Unknown(t),
        }
    }
}

impl From<FlvTagType> for u8 {
    fn from(value: FlvTagType) -> Self {
        match value {
            FlvTagType::Audio => 8,
            FlvTagType::Video => 9,
            FlvTagType::Metadata => 18,
            FlvTagType::Unknown(v) => v,
        }
    }
}

/// The tag header part of FLV tag.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FlvTagHeader {
    /// Reserved    2 bits  Reserved for FMS, should be 0.
    /// Filter      1 bit   Indicates if packets are filtered.
    ///                     0 = No pre-processing required
    ///                     1 = Pre-processing (Such as decryption) of the packet
    ///                         is required before it can be rendered.
    /// TagType     5 bits  The type of contents in this tag,
    ///                     8 = audio, 9 = video, 18 = script.
    pub tag_type: FlvTagType,

    /// The size of the tag's data part, 3 bytes.
    pub data_size: u32,

    /// The timestamp (in milliseconds) of the tag, Timestamp (3 bytes) + TimestampExtended (1 byte).
    pub timestamp: u32,

    /// The id of stream is always 0, 3 bytes.
    pub stream_id: u32,
}

/// The tag data part of FLV tag.
#[derive(Clone, Debug, PartialEq)]
pub enum FlvTagData {
    /// Audio tag data.
    Audio(audio::AudioTag),

    /// Video tag data.
    Video(video::VideoTag),

    /// Script tag data.
    Meta(meta::MetaTag),

    /// Unknown
    Unknown,
}
