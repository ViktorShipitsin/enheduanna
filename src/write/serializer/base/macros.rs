/// Макрос для генерации сериализаторов с заданным порядком байт.
#[macro_export]
macro_rules! impl_int_serializer {
    ($name:ident, $type:ty, $endian_fn:ident) => {
        pub struct $name;

        impl Serializer<$type> for $name {
            const MAX_BYTES: usize = std::mem::size_of::<$type>();

            fn serialize(value: $type, buf: &mut [u8]) -> usize {
                buf.copy_from_slice(&value.$endian_fn());
                return Self::MAX_BYTES
            }
        }
    };
}