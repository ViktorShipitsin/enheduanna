//-------------------------------------\\
//               Импорты               \\
//-------------------------------------\\

use std::io::{Write};
use std::marker::PhantomData;

use super::serializer::{Serializer};

pub mod modes;
#[macro_use] mod macros;

//-------------------------------------\\
//            Режим записи             \\
//-------------------------------------\\

/// Маркерный трейт для режимов записи.
pub trait WriteMode {
    const IS_BATCH: bool;
}

//-------------------------------------\\
//             WriteMethod             \\
//-------------------------------------\\

/// Конвенция для методов записи.
pub struct WriteMethod<'a, T, W: Write + ?Sized, M: WriteMode, S: Serializer<T>, const N: usize> {
    writer: &'a mut W,
    _mode: PhantomData<M>,
    _ser: PhantomData<S>,
    _t: PhantomData<T>,
}

/// Базовая реализация фундаментального функционала WriteMethod.
impl<'a, T, W, M, S, const N: usize> WriteMethod<'a, T, W, M, S, N>
where
    W: Write + ?Sized,
    M: WriteMode,
    S: Serializer<T>,
{
    /// Создание нового экземпляра WriteMethod.
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer, _mode: PhantomData, _ser: PhantomData, _t: PhantomData }
    }

    /// Функция, которая заимствует writer у WriteMethod.
    pub fn writer(&mut self) -> &mut W {
        self.writer
    }

    /// Функция, которая забирает writer у WriteMethod.
    pub fn into_writer(self) -> &'a mut W {
        self.writer
    }
}