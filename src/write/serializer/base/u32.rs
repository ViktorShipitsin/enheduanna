use super::Serializer;

pub struct U32Le;
impl Serializer<u32> for U32Le {
    const MAX_BYTES: usize = 4;
    type Error = std::io::Error;

    fn serialize(value: u32, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Buffer too small for u32"
            ));
        }
        let b = value.to_le_bytes();
        buf[..4].copy_from_slice(&b);
        Ok(4)
    }
}

pub struct U32Be;
impl Serializer<u32> for U32Be {
    const MAX_BYTES: usize = 4;
    type Error = std::io::Error;

    fn serialize(value: u32, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Buffer too small for u32"
            ));
        }
        let b = value.to_be_bytes();
        buf[..4].copy_from_slice(&b);
        Ok(4)
    }
}