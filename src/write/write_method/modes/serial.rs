use std::io::{Write, Result as IoResult};

use super::WriteMethod;
use super::Serializer;
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
    pub fn write<I>(&mut self, iter: I) -> IoResult<()>
    where
        I: IntoIterator,
        I::Item: Into<T>,
    {
        for item in iter {
            let v: T = item.into();
            self.write_byte(v)?;
        }
        Ok(())
    }
}