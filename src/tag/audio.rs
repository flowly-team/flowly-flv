use bytes::{Buf, Bytes};

/// The tag data part of `audio` FLV tag, including `tag data header` and `tag data body`.
#[derive(Clone, Debug, PartialEq)]
pub struct AudioTag {
    /// The header part of `audio` FLV tag.
    pub header: AudioTagHeader, // 8 bits.

    /// The body part of `audio` FLV tag.
    pub body: AudioTagBody,
}

/// The `tag data header` part of `audio` FLV tag data.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AudioTagHeader {
    /// The format of sound, 4 bits.
    pub sound_format: SoundFormat,

    /// The rate of sound, 2 bits.
    pub sound_rate: SoundRate,

    /// The sample size of sound, 1 bit.
    pub sound_size: SoundSize,

    /// The type of sound, 1 bit.
    pub sound_type: SoundType,
}

/// The `tag data body` part of `audio` FLV tag data.
#[derive(Clone, Debug, PartialEq)]
pub struct AudioTagBody {
    /// The actual `tag data body` of `audio` FLV tag data.
    pub data: Bytes,
}

/// The audio format.
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundFormat {
    /// 0, PcmPlatformEndian
    PcmPlatformEndian,

    /// 1, ADPCM
    ADPCM,

    /// 2, MP3
    MP3,

    /// 3, PcmLittleEndian
    PcmLittleEndian,

    /// 4, Nellymoser16kHzMono
    Nellymoser16kHzMono,

    /// 5, Nellymoser8kHzMono
    Nellymoser8kHzMono,

    /// 6, Nellymoser
    Nellymoser,

    /// 7, PcmALaw
    PcmALaw,

    /// 8, PcmMuLaw
    PcmMuLaw,

    /// 9, Reserved
    ExHeader,

    /// 10, MPEG-4 Part3 AAC
    AAC,

    /// 11, Speex
    Speex,

    /// 14, MP3_8kHz
    MP3_8kHz,

    /// 15, DeviceSpecific
    DeviceSpecific,
}

impl From<u8> for SoundFormat {
    fn from(value: u8) -> Self {
        match value {
            0 => SoundFormat::PcmPlatformEndian,
            1 => SoundFormat::ADPCM,
            2 => SoundFormat::MP3,
            3 => SoundFormat::PcmLittleEndian,
            4 => SoundFormat::Nellymoser16kHzMono,
            5 => SoundFormat::Nellymoser8kHzMono,
            6 => SoundFormat::Nellymoser,
            7 => SoundFormat::PcmALaw,
            8 => SoundFormat::PcmMuLaw,
            9 => SoundFormat::ExHeader,
            10 => SoundFormat::AAC,
            11 => SoundFormat::Speex,
            14 => SoundFormat::MP3_8kHz,
            15 => SoundFormat::DeviceSpecific,
            _ => unreachable!(),
        }
    }
}

/// The audio sampling rate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundRate {
    /// 0, 5.5 KHz.
    _5_5KHZ,

    /// 1, 11 KHz.
    _11KHZ,

    /// 2, 22 KHz.
    _22KHZ,

    /// 3, 44 KHz.
    _44KHZ,
}

impl From<u8> for SoundRate {
    fn from(value: u8) -> Self {
        match value {
            0 => SoundRate::_5_5KHZ,
            1 => SoundRate::_11KHZ,
            2 => SoundRate::_22KHZ,
            3 => SoundRate::_44KHZ,
            _ => unreachable!(),
        }
    }
}

/// The size of each audio sample.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundSize {
    /// 0, 8 bit.
    _8Bit,

    /// 1, 16 bit.
    _16Bit,
}

impl From<u8> for SoundSize {
    fn from(value: u8) -> Self {
        match value {
            0 => SoundSize::_8Bit,
            1 => SoundSize::_16Bit,
            _ => unreachable!(),
        }
    }
}

/// The type of audio, including mono and stereo.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundType {
    /// 0, Mono sound.
    Mono,

    /// 1, Stereo sound.
    Stereo,
}

impl From<u8> for SoundType {
    fn from(value: u8) -> Self {
        match value {
            0 => SoundType::Mono,
            1 => SoundType::Stereo,
            _ => unreachable!(),
        }
    }
}

/// The `tag data body` part of `audio` FLV tag data whose `SoundFormat` is 10 -- AAC.
#[derive(Clone, Debug, PartialEq)]
pub struct AACAudioPacket {
    /// Only useful when sound format is 10 -- AAC, 1 byte.
    pub packet_type: AACPacketType,

    /// The actual AAC data.
    pub aac_data: Bytes,
}

/// The type of AAC packet.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AACPacketType {
    /// 0, SequenceHeader.
    SequenceHeader,

    /// 1, Raw.
    Raw,
}

impl From<u8> for AACPacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => AACPacketType::SequenceHeader,
            _ => AACPacketType::Raw,
        }
    }
}

/// Parse AAC audio packet.
pub fn aac_audio_packet(mut reader: Bytes) -> AACAudioPacket {
    let packet_type = reader.get_u8();

    AACAudioPacket {
        packet_type: AACPacketType::from(packet_type),
        aac_data: reader,
    }
}
