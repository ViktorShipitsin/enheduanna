use crate::{impl_int_serializer};
use super::{Serializer};

// Big-endian сериализаторы.
impl_int_serializer!(I16BE, i16, to_be_bytes);
impl_int_serializer!(I32BE, i32, to_be_bytes);
impl_int_serializer!(I64BE, i64, to_be_bytes);

// Little-endian сериализаторы.
impl_int_serializer!(I16LE, i16, to_le_bytes);
impl_int_serializer!(I32LE, i32, to_le_bytes);
impl_int_serializer!(I64LE, i64, to_le_bytes);

// Native-endian сериализаторы.
impl_int_serializer!(I8NE, i8, to_ne_bytes);
impl_int_serializer!(I16NE, i16, to_ne_bytes);
impl_int_serializer!(I32NE, i32, to_ne_bytes);
impl_int_serializer!(I64NE, i64, to_ne_bytes);