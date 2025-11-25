//-------------------------------------\\
//               Импорты               \\
//-------------------------------------\\

use std::io::{Write};
use std::marker::PhantomData;

use super::serializer::{Serializer};

pub mod modes;

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

//------------------------------------\\
//               Макрос               \\
//------------------------------------\\

#[macro_export]
macro_rules! impl_inline_write_methods {
    ($writer:ident;
        $(
            $vis:vis fn $method:ident : $ty:ty => $ser:ty => $mode:ident
        ),* $(,)?
    ) => {
        impl $writer {
            $(
                $crate::impl_inline_write_methods!(@emit $vis, $method, $ty, $ser, $mode);
            )*
        }
    };

    (@emit $vis:vis, $method:ident, $ty:ty, $ser:ty, single) => {
        #[inline]
        $vis fn $method(&mut self, value: $ty) -> std::io::Result<()> {
            let mut wm = $crate::shared::write::write_method::WriteMethod::<
                $ty,
                _,
                $crate::shared::write::write_method::single::Single,
                $ser,
                { <$ser as $crate::shared::write::serializer::Serializer<$ty>>::MAX_BYTES }
            >::new(&mut self.writer);
            wm.write(value)
        }
    };

    (@emit $vis:vis, $method:ident, $ty:ty, $ser:ty, serial) => {
        #[inline]
        $vis fn $method<I>(&mut self, values: I) -> std::io::Result<()>
        where
            I: IntoIterator,
            I::Item: Into<$ty>,
        {
            let mut wm = $crate::shared::write::write_method::WriteMethod::<
                $ty,
                _,
                $crate::shared::write::write_method::serial::Serial,
                $ser,
                { <$ser as $crate::shared::write::serializer::Serializer<$ty>>::MAX_BYTES }
            >::new(&mut self.writer);
            wm.write_many(values)
        }
    };
}