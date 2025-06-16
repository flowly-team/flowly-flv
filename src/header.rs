#[derive(Default, Debug, Clone, PartialEq)]
pub struct FlvHeader {
    /// The version of the FLV file.
    pub version: u8,

    /// Whether the FLV file contains audio tags.
    pub is_audio_present: bool,

    /// Whether the FLV file contains video tags.
    pub is_video_present: bool,

    pub remaining: u32,
}
