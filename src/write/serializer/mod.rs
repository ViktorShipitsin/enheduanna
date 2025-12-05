pub mod base;

//--------------------------------------\\
//             Сериализатор             \\
//--------------------------------------\\

pub trait Serializer<T> {
    const MAX_BYTES: usize;
    fn serialize(value: T, buf: &mut [u8]) -> usize;
}