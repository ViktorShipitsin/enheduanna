// Реэкспорт макроса для объявления нового WriteMethod.
#[macro_use] mod macros;

// Публичные модули.
pub mod write;

// Публичные утилиты (предполагаются к использованию).
pub use write::serializer::{Serializer};
pub use write::write_method::{WriteMethod};
pub use write::write_method::{WriteMode};