use std::collections::HashMap;

use bytes::Bytes;

/// The tag data part of `script` FLV tag, including `name` and `value`.
/// The `name` is a `ScriptDataValue` enum whose type is `String`.
/// The `value` is a `ScriptDataValue` enum whose type is `ECMAArray`.
#[derive(Clone, Debug, PartialEq)]
pub struct MetaTag {
    /// Method or object name.
    /// ScriptTagValue.Type = 2 (String)
    pub name: Bytes,

    /// AMF arguments or object properties.
    /// ScriptTagValue.Type = 8 (ECMAArray)
    pub value: MetaDataValue,
}

/// The `ScriptDataValue` enum.
#[derive(Debug, Clone, PartialEq)]
pub enum MetaDataValue {
    /// 0, Number value.
    Number(f64),

    /// 1, Boolean value.
    Boolean(bool),

    /// 2, String value.
    String(Bytes),

    /// 3, Object value.
    Object(HashMap<Bytes, MetaDataValue>),

    /// 4, MovieClip value.
    MovieClip,

    /// 5, Null value.
    Null,

    /// 6, Undefined value.
    Undefined,

    /// 7, Reference value.
    Reference(u16),

    /// 8, ECMA Array value.
    ECMAArray(HashMap<Bytes, MetaDataValue>),

    /// 10, Strict Array value.
    StrictArray(Vec<MetaDataValue>),

    /// 11, Date value.
    Date(MetaDataDate),

    /// 12, Long String value.
    LongString(Bytes),

    Unknown(u8),
}

/// The `ScriptDataObjectProperty` is the component of `Object` and `ECMAArray`,
/// which are a kind of `ScriptDataValue`.
#[derive(Clone, Debug, PartialEq)]
pub struct MetaDataObjectProperty {
    /// Object property name.
    pub key: Bytes,

    /// Object property value.
    pub value: MetaDataValue,
}

/// The `ScriptDataDate` is a kind of `ScriptDataValue`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MetaDataDate {
    /// Number of milliseconds since UNIX_EPOCH.
    // SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
    pub date_time: f64,

    /// Local time offset in minutes from UTC.
    /// For time zones located west of Greenwich, this value is a negative number.
    /// Time zones east of Greenwich are positive.
    pub local_date_time_offset: i16,
}
