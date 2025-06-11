use serde::de::Visitor;

use crate::{error::Error, tag::meta::MetaDataValue};

struct SeqRefDeserializer<'de> {
    iter: std::slice::Iter<'de, MetaDataValue>,
}

impl<'de> SeqRefDeserializer<'de> {
    fn new(slice: &'de [MetaDataValue]) -> Self {
        SeqRefDeserializer { iter: slice.iter() }
    }
}

impl<'de> serde::de::SeqAccess<'de> for SeqRefDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),

            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),

            _ => None,
        }
    }
}

fn visit_array_ref<'de, V>(array: &'de [MetaDataValue], visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = array.len();
    let mut deserializer = SeqRefDeserializer::new(array);
    let seq = match visitor.visit_seq(&mut deserializer) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let remaining = deserializer.iter.len();

    if remaining == 0 {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in array",
        ))
    }
}

impl<'de> serde::Deserializer<'de> for &'de MetaDataValue {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Null => visitor.visit_unit(),
            MetaDataValue::Boolean(v) => visitor.visit_bool(*v),
            MetaDataValue::Number(n) => visitor.visit_f64(*n),
            MetaDataValue::String(v) => {
                visitor.visit_borrowed_str(unsafe { str::from_utf8_unchecked(v) })
            }
            MetaDataValue::StrictArray(v) => visit_array_ref(v, visitor),
            // MetaDataValue::Object(v) => v.deserialize_any(visitor),
            // MetaDataValue::ECMAArray(items) => v.deserialize_any(visitor),
            MetaDataValue::Date(_) => todo!(),
            MetaDataValue::LongString(v) => {
                visitor.visit_borrowed_str(unsafe { str::from_utf8_unchecked(v) })
            }

            MetaDataValue::Reference(_) => unimplemented!(),
            MetaDataValue::MovieClip => unimplemented!(),
            MetaDataValue::Undefined => unimplemented!(),
            _ => unimplemented!(),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match *self {
            MetaDataValue::Null => visitor.visit_none(),
            MetaDataValue::Undefined => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        let _ = variants;
        let _ = name;
        unimplemented!()
        // match self {
        //     MetaDataValue::Object(value) => value.deserialize_enum(name, variants, visitor),
        //     MetaDataValue::ECMAArray(value) => value.deserialize_enum(name, variants, visitor),
        //     MetaDataValue::String(variant) => visitor.visit_enum(EnumRefDeserializer {
        //         variant,
        //         value: None,
        //     }),

        //     other => Err(serde::de::Error::invalid_type(
        //         other.unexpected(),
        //         &"string or map",
        //     )),
        // }
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let _ = name;
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match *self {
            MetaDataValue::Boolean(v) => visitor.visit_bool(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::String(v) => {
                visitor.visit_borrowed_str(unsafe { str::from_utf8_unchecked(v) })
            }
            MetaDataValue::LongString(v) => {
                visitor.visit_borrowed_str(unsafe { str::from_utf8_unchecked(v) })
            }
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::String(v) => {
                visitor.visit_borrowed_str(unsafe { str::from_utf8_unchecked(v) })
            }
            MetaDataValue::LongString(v) => {
                visitor.visit_borrowed_str(unsafe { str::from_utf8_unchecked(v) })
            }
            MetaDataValue::StrictArray(v) => visit_array_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match *self {
            MetaDataValue::Null => visitor.visit_unit(),
            MetaDataValue::Undefined => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::StrictArray(v) => visit_array_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        unimplemented!()
        // match self {
        //     MetaDataValue::Object(v) => v.deserialize_any(visitor),
        //     MetaDataValue::ECMAArray(v) => v.deserialize_any(visitor),
        //     _ => Err(self.invalid_type(&visitor)),
        // }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        unimplemented!()
        // match self {
        //     MetaDataValue::StrictArray(v) => visit_array_ref(v, visitor),
        //     MetaDataValue::Object(v) => v.deserialize_any(visitor),
        //     MetaDataValue::ECMAArray(v) => v.deserialize_any(visitor),
        //     _ => Err(self.invalid_type(&visitor)),
        // }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_i8(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_i16(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_i32(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_i64(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_u8(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_u16(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_u32(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_u64(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_f32(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MetaDataValue::Number(v) => visitor.visit_f64(*v as _),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}
