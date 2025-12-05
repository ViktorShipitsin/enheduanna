use std::io::{Write, Result as IoResult};

use super::WriteMethod;
use super::Serializer;
use super::WriteMode;

pub struct Single;
impl WriteMode for Single {
    const IS_BATCH: bool = false;
}

/// Имплементация функционала для режима записи Single.
impl<'a, T, W, S, const N: usize> WriteMethod<'a, T, W, Single, S, N>
where
    W: Write + ?Sized,
    S: Serializer<T>,
{
    /// Функция записи.
    pub fn write(&mut self, value: T, buf: &mut [u8; N]) -> IoResult<()> {
        self.write_value(value, buf)
    }
}