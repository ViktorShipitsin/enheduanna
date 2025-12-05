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

    // Single mode
    (@emit $vis:vis, $method:ident, $ty:ty, $ser:ty, single) => {
        #[inline]
        $vis fn $method(&mut self, value: $ty) -> std::io::Result<()> {
            let mut buf = [0u8; <$ser as $crate::Serializer<$ty>>::MAX_BYTES];
            let mut wm = $crate::WriteMethod::<
                $ty,
                _,
                $crate::WriteMode::Single,
                $ser,
                { <$ser as $crate::Serializer<$ty>>::MAX_BYTES }
            >::new(&mut self.writer);
            wm.write(value, &mut buf)
        }
    };

    // Serial mode
    (@emit $vis:vis, $method:ident, $ty:ty, $ser:ty, serial) => {
        #[inline]
        $vis fn $method<I>(&mut self, values: I) -> std::io::Result<()>
        where
            I: IntoIterator,
            I::Item: Into<$ty>,
        {
            let mut buf = [0u8; <$ser as $crate::Serializer<$ty>>::MAX_BYTES];
            let mut wm = $crate::WriteMethod::<
                $ty,
                _,
                $crate::WriteMode::Serial,
                $ser,
                { <$ser as $crate::Serializer<$ty>>::MAX_BYTES }
            >::new(&mut self.writer);
            for v in values {
                wm.write(v.into(), &mut buf)?;
            }
            Ok(())
        }
    };
}