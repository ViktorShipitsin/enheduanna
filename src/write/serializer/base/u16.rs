use super::{Serializer};

pub struct U16Le;
impl Serializer<u16> for U16Le {
    const MAX_BYTES: usize = 2;
    type Error = std::io::Error;

    fn serialize(value: u16, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Buffer too small for u16"
            ));
        }
        let b = value.to_le_bytes();
        buf[..2].copy_from_slice(&b);
        Ok(2)
    }
}

pub struct U16Be;
impl Serializer<u16> for U16Be {
    const MAX_BYTES: usize = 2;
    type Error = std::io::Error;

    fn serialize(value: u16, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Buffer too small for u16"
            ));
        }
        let b = value.to_be_bytes();
        buf[..2].copy_from_slice(&b);
        Ok(2)
    }
}