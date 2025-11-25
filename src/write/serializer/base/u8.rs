use super::{Serializer};

pub struct U8Serializer;

impl Serializer<u8> for U8Serializer {
    const MAX_BYTES: usize = 1;
    type Error = std::io::Error;

    fn serialize(value: u8, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Buffer too small for u8"
            ));
        }
        buf[0] = value;
        Ok(1)
    }
}