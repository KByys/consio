use crate::FromBuf;

pub trait Atom: FromBuf { }
macro_rules! atom {
    ($($arg:ty), *) => {
        $(
            impl Atom for $arg {}
        )*
    };
}

atom!(i8, i16, i32, isize, i64, i128, u8, u16, u32, usize, u64, u128, f32, f64, char, String);
pub trait AtomVec: FromBuf { }
macro_rules! atom_vec {
    ($($arg:ty), *) => {
        $(
            impl AtomVec for $arg {}
        )*
    };
}
atom_vec!(i8, i16, i32, isize, i64, i128, u16, u32, usize, u64, u128, f32, f64, String);