//------------------------------------\\
//               Макрос               \\
//------------------------------------\\

/// Макрос объявления метода записи.
//#[macro_export]
//macro_rules! impl_inline_write_methods {
//    ($writer:ident;
//        $(
//            $vis:vis fn $method:ident : $ty:ty => $ser:ty => $mode:ident
//        ),* $(,)?
//    ) => {
//        impl $writer {
//            $(
//                $crate::impl_inline_write_methods!(@emit $vis, $method, $ty, $ser, $mode);
//            )*
//        }
//    };
//
//    (@emit $vis:vis, $method:ident, $ty:ty, $ser:ty, single) => {
//        #[inline]
//        $vis fn $method(&mut self, value: $ty) -> std::io::Result<()> {
//            let mut wm = $crate::shared::write::write_method::WriteMethod::<
//                $ty,
//                _,
//                $crate::shared::write::write_method::single::Single,
//                $ser,
//                { <$ser as $crate::shared::write::serializer::Serializer<$ty>>::MAX_BYTES }
//            >::new(&mut self.writer);
//            wm.write(value)
//        }
//    };
//
//    (@emit $vis:vis, $method:ident, $ty:ty, $ser:ty, serial) => {
//        #[inline]
//        $vis fn $method<I>(&mut self, values: I) -> std::io::Result<()>
//        where
//            I: IntoIterator,
//            I::Item: Into<$ty>,
//        {
//            let mut wm = $crate::shared::write::write_method::WriteMethod::<
//                $ty,
//                _,
//                $crate::shared::write::write_method::serial::Serial,
//                $ser,
//                { <$ser as $crate::shared::write::serializer::Serializer<$ty>>::MAX_BYTES }
//            >::new(&mut self.writer);
//            wm.write(values)
//        }
//    };
//}