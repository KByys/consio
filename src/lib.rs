pub mod core;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error;
/// input a line and convert it to any type which implement trait `FromBuf`
/// 
/// Support Type:
/// 
/// (unsigned) `integer` (*binary, octal, decimal, hex*)
/// 
/// `float`, `char`, `String` and their `Vec<T>`
/// 
/// # Usage
/// ```no_run
/// use consio::input;
/// 
/// let str = input!(String).unwrap(); // Console input "Hello World!"
/// assert_eq!(str.as_str(), "Hello World!");
/// 
/// let n = input!(i32).unwrap();  // Console input "11"
/// assert_eq!(n, 11);
///
/// let hex = input!(i32).unwrap(); // Console input "11h" or "0x11"
/// assert_eq!(hex, 17);
///
/// // print something before input
/// let value = input!(print "Input a string: ").unwrap();
///
/// // input with a default value
/// let value = input!(default); // Console input any invalid number
/// assert_eq!(value, Default::default());
/// ```
#[macro_export]
macro_rules! input {
    () => {
        $crate::core::input()
    };
    (default) => {
        $crate::core::input().unwrap_or_default()
    };
    (default => $ty:ty) => {
        $crate::core::input::<$ty>().unwrap_or_default()
    };
    ($ty:ty) => {
        $crate::core::input::<$ty>()
    };
    ($ty:ty, print $($arg:tt)*) => { {
        print!($($arg)*);
        use std::io::Write;
        match std::io::stdout().flush() {
            Ok(_) => $crate::core::input::<$ty>(),
            Err(e) => Err(e.into()),
        } }
    };
    (print $($arg:tt)*) => { {
        print!($($arg)*);
        use std::io::Write;
        std::io::stdout().flush().unwrap_or(());
        $crate::core::input::<String>()
    } };
    (or $def:expr) => {
        $crate::core::input().unwrap_or($def)
    };
    (or $def:expr, print $($arg:tt)*) => { {
        print!($($arg)*);
        use std::io::Write;
        std::io::stdout().flush().unwrap_or(());
        $crate::core::input().unwrap_or($def)
    } };
}

pub enum FromBufError<T> {
    ConvertError(T),
    IoError(Error),
    InvalidUtf8String,
    Empty,
}
impl<T: Debug + Display> std::error::Error for FromBufError<T> {}
impl<T: Debug> Debug for FromBufError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvertError(arg0) => f.debug_tuple("ConvertError").field(arg0).finish(),
            Self::IoError(arg0) => f.debug_tuple("IoError").field(arg0).finish(),
            Self::InvalidUtf8String => f.write_str("InvalidUtf8String"),
            Self::Empty => f.write_str("Empty"),
        }
    }
}

impl<T: Display> Display for FromBufError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvertError(e) => f.write_str(e.to_string().as_str()),
            Self::IoError(e) => f.write_str(e.to_string().as_str()),
            Self::InvalidUtf8String => f.write_str("InvalidUtf8String"),
            Self::Empty => f.write_str("Empty"),
        }
    }
}

impl<T> From<Error> for FromBufError<T> {
    fn from(value: Error) -> Self {
        FromBufError::IoError(value)
    }
}

pub trait FromBuf: Sized {
    type Error;
    fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>>;
}
