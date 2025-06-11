#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fourcc([u8; 4]);

impl Fourcc {
    pub const AUDIO_AC3: Fourcc = Fourcc(*b"ac-3");
    pub const AUDIO_EC3: Fourcc = Fourcc(*b"ec-3");
    pub const AUDIO_OPUS: Fourcc = Fourcc(*b"Opus");
    pub const AUDIO_MP3: Fourcc = Fourcc(*b".mp3");
    pub const AUDIO_FLAC: Fourcc = Fourcc(*b"fLaC");
    pub const AUDIO_AAC: Fourcc = Fourcc(*b"mp4a");

    pub const VIDEO_VP8: Fourcc = Fourcc(*b"vp08");
    pub const VIDEO_VP9: Fourcc = Fourcc(*b"vp09");
    pub const VIDEO_AV1: Fourcc = Fourcc(*b"av01");
    pub const VIDEO_AVC: Fourcc = Fourcc(*b"avc1");
    pub const VIDEO_HEVC: Fourcc = Fourcc(*b"hvc1");
}

impl From<u32> for Fourcc {
    fn from(value: u32) -> Self {
        Self(value.to_be_bytes())
    }
}

impl std::fmt::Debug for Fourcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fourcc(\"{}\")",
            str::from_utf8(&self.0).ok().unwrap_or("<ERR>")
        )
    }
}
impl std::fmt::Display for Fourcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).ok().unwrap_or("<ERR>"))
    }
}
