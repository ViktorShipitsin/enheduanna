use std::io::{Result as IoResult, Write};

use super::Serializer;
use super::WriteMethod;
use super::WriteMode;

pub struct Serial;
impl WriteMode for Serial {
    const IS_BATCH: bool = true;
}

/// Имплементация функционала для режима записи Serial.
impl<'a, T, W, S, const N: usize> WriteMethod<'a, T, W, Serial, S, N>
where
    W: Write + ?Sized,
    S: Serializer<T>,
{
    /// Функция записи.
    pub fn write<I>(&mut self, buf: &mut [u8; N], iter: I) -> IoResult<()>
    where
        I: IntoIterator,
        I::Item: Into<T>,
    {
        for item in iter {
            let v: T = item.into();
            self.write_value(v, buf)?;
        }
        Ok(())
    }
}