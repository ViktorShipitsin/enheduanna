pub mod base;

//--------------------------------------\\
//             Сериализатор             \\
//--------------------------------------\\

pub trait Serializer<T> {
    const MAX_BYTES: usize;
    type Error: std::error::Error + Send + Sync + 'static;
    fn serialize(value: T, buf: &mut [u8]) -> Result<usize, Self::Error>;
}