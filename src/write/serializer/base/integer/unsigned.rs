use crate::{impl_int_serializer};
use super::{Serializer};

// Big-endian сериализаторы.
impl_int_serializer!(U16BE, u16, to_be_bytes);
impl_int_serializer!(U32BE, u32, to_be_bytes);
impl_int_serializer!(U64BE, u64, to_be_bytes);

// Little-endian сериализаторы.
impl_int_serializer!(U16LE, u16, to_le_bytes);
impl_int_serializer!(U32LE, u32, to_le_bytes);
impl_int_serializer!(U64LE, u64, to_le_bytes);

// Native-endian сериализаторы.
impl_int_serializer!(U8NE, u8, to_ne_bytes);
impl_int_serializer!(I16NE, u16, to_ne_bytes);
impl_int_serializer!(I32NE, u32, to_ne_bytes);
impl_int_serializer!(I64NE, u64, to_ne_bytes);