use std::io::{Write, Result as IoResult};

use super::WriteMethod;
use super::Serializer;
use super::WriteMode;

pub struct Single;
impl WriteMode for Single {
    const IS_BATCH: bool = false;
}

impl<'a, T, W, S, const N: usize> WriteMethod<'a, T, W, Single, S, N>
where
    W: Write + ?Sized,
    S: Serializer<T>,
{
    pub fn write(&mut self, value: T) -> IoResult<()> {
        self.write_byte(value)
    }
}