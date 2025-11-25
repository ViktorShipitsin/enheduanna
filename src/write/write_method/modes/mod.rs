use std::io::{Write, Result as IoResult};

use super::WriteMethod;
use super::Serializer;
use super::WriteMode;

pub mod single;
pub mod serial;

impl<'a, T, W, M, S, const N: usize> WriteMethod<'a, T, W, M, S, N>
where
    W: Write + ?Sized,
    M: WriteMode,
    S: Serializer<T>,
{
    fn write_byte(&mut self, value: T) -> IoResult<()> {
        let mut buf = [0u8; N];
        let n = S::serialize(value, &mut buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        debug_assert!(n <= N);
        self.writer.write_all(&buf[..n])
    }
}