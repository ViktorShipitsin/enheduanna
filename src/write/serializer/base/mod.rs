use super::{Serializer};

mod  macros;

pub mod integer;

pub use integer::signed::{
    I8NE,
    I16BE, I16LE, I16NE,
    I32BE, I32LE, I32NE,
    I64BE, I64LE, I64NE
};

pub use integer::unsigned::{
    U8NE,
    U16BE, U16LE, U16NE,
    U32BE, U32LE, U32NE,
    U64BE, U64LE, U64NE
};