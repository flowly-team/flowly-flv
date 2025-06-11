use std::fmt;

use crate::{error::Error, tag::meta::MetaDataValue};

pub mod meta;

impl MetaDataValue {
    #[cold]
    fn invalid_type<E>(&self, exp: &dyn serde::de::Expected) -> E
    where
        E: serde::de::Error,
    {
        serde::de::Error::invalid_type(self.unexpected(), exp)
    }

    #[cold]
    fn unexpected(&self) -> serde::de::Unexpected {
        use serde::de::Unexpected;
        match self {
            MetaDataValue::Null => Unexpected::Unit,
            MetaDataValue::Boolean(b) => Unexpected::Bool(*b),
            MetaDataValue::Number(n) => Unexpected::Float(*n),
            MetaDataValue::String(s) => Unexpected::Str(unsafe { str::from_utf8_unchecked(s) }),
            MetaDataValue::StrictArray(_) => Unexpected::Seq,
            MetaDataValue::Object(_) => Unexpected::Map,
            MetaDataValue::MovieClip => Unexpected::Other("movie"),
            MetaDataValue::Undefined => Unexpected::Other("undefined"),
            MetaDataValue::Reference(_) => todo!(),
            MetaDataValue::ECMAArray(_) => Unexpected::Map,
            MetaDataValue::Date(_) => Unexpected::Other("date"),
            MetaDataValue::LongString(s) => Unexpected::Str(unsafe { str::from_utf8_unchecked(s) }),
            MetaDataValue::Unknown(_) => Unexpected::Other("unknown"),
        }
    }
}

struct MetaDataUnexpected<'a>(serde::de::Unexpected<'a>);

impl<'a> fmt::Display for MetaDataUnexpected<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            serde::de::Unexpected::Unit => formatter.write_str("null"),

            unexp => fmt::Display::fmt(&unexp, formatter),
        }
    }
}

impl serde::de::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::ParseMetaError(msg.to_string())
    }

    #[cold]
    fn invalid_type(unexp: serde::de::Unexpected, exp: &dyn serde::de::Expected) -> Self {
        Error::ParseMetaError(format!(
            "invalid type: {}, expected {}",
            MetaDataUnexpected(unexp),
            exp,
        ))
    }

    #[cold]
    fn invalid_value(unexp: serde::de::Unexpected, exp: &dyn serde::de::Expected) -> Self {
        Error::ParseMetaError(format!(
            "invalid value: {}, expected {}",
            MetaDataUnexpected(unexp),
            exp,
        ))
    }
}
