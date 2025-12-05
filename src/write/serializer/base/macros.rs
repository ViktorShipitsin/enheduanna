/// Макрос для генерации сериализаторов с заданным порядком байт.
#[macro_export]
macro_rules! impl_int_serializer {
    ($name:ident, $type:ty, $endian_fn:ident) => {
        pub struct $name;

        impl Serializer<$type> for $name {
            const MAX_BYTES: usize = std::mem::size_of::<$type>();

            fn serialize(value: $type, buf: &mut [u8]) -> usize {
                buf[..2].copy_from_slice(&value.to_le_bytes());
                return Self::MAX_BYTES
            }
        }
    };
}