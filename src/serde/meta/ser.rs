pub struct Serializer;

impl serde::Serializer for Serializer {
    type Ok = MetaDataValue;
    type Error = Error;
    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<MetaDataValue> {
        Ok(MetaDataValue::Bool(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<MetaDataValue> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<MetaDataValue> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<MetaDataValue> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<MetaDataValue> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_i128(self, value: i128) -> Result<MetaDataValue> {
        Ok(Value::Number(value.into()))
    }

    #[inline]

    fn serialize_u8(self, value: u8) -> Result<MetaDataValue> {
        self.serialize_u64(value as u64)
    }

    #[inline]

    fn serialize_u16(self, value: u16) -> Result<MetaDataValue> {
        self.serialize_u64(value as u64)
    }

    #[inline]

    fn serialize_u32(self, value: u32) -> Result<MetaDataValue> {
        self.serialize_u64(value as u64)
    }

    #[inline]

    fn serialize_u64(self, value: u64) -> Result<MetaDataValue> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_u128(self, value: u128) -> Result<MetaDataValue> {
        Ok(Value::Number(value.into()))
    }

    #[inline]
    fn serialize_f32(self, float: f32) -> Result<MetaDataValue> {
        Ok(Value::from(float))
    }

    #[inline]
    fn serialize_f64(self, float: f64) -> Result<MetaDataValue> {
        Ok(Value::from(float))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<MetaDataValue> {
        let mut s = String::new();

        s.push(value);

        Ok(Value::String(s))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<MetaDataValue> {
        Ok(Value::String(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<MetaDataValue> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();

        Ok(Value::Array(vec))
    }

    #[inline]
    fn serialize_unit(self) -> Result<MetaDataValue> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<MetaDataValue> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<MetaDataValue> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<MetaDataValue>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<MetaDataValue>
    where
        T: ?Sized + Serialize,
    {
        let mut values = Map::new();
        values.insert(String::from(variant), tri!(to_value(value)));
        Ok(Value::Object(values))
    }

    #[inline]
    fn serialize_none(self) -> Result<MetaDataValue> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<MetaDataValue>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,

        _name: &'static str,

        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            name: String::from(variant),
            vec: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap::Map {
            map: Map::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(SerializeMap::Number { out_value: None }),

            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(SerializeMap::RawValue { out_value: None }),

            _ => self.serialize_map(Some(len)),
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            name: String::from(variant),
            map: Map::new(),
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<MetaDataValue>
    where
        T: ?Sized + Display,
    {
        Ok(Value::String(value.to_string()))
    }
}
