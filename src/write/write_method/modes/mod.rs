use std::io::{Result as IoResult, Write};

use super::Serializer;
use super::WriteMethod;
use super::WriteMode;

pub mod single;
pub mod serial;

/// Имплементация базового функционала.
impl<'a, T, W, M, S, const N: usize> WriteMethod<'a, T, W, M, S, N>
where
    W: Write + ?Sized,
    M: WriteMode,
    S: Serializer<T>,
{
    /// Базовая функция записи (побайтовая).
    fn write_value(&mut self, value: T, buf: &mut [u8; N]) -> IoResult<()> {
        debug_assert!(N >= S::MAX_BYTES);
        let ser_bytes: usize = S::serialize(value, buf);
        debug_assert!(ser_bytes <= N);
        self.writer.write_all(&buf[..ser_bytes])
    }
}