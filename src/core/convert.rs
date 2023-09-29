use super::atom::AtomVec;

use super::FromBuf;
use super::FromBufError;
use std::num::{ParseFloatError, ParseIntError};
impl FromBuf for String {
    type Error = ();
    fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>> {
        match String::from_utf8(buf) {
            Ok(value) => Ok(value.trim().to_owned()),
            _ => Err(FromBufError::InvalidUtf8String),
        }
    }
}

impl FromBuf for Vec<u8> {
    type Error = ();
    fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>> {
        Ok(buf)
    }
}

impl FromBuf for char {
    type Error = ();

    fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>> {
        let Ok(buf) = String::from_buf(buf) else {
            return Err(FromBufError::InvalidUtf8String);
        };
        if buf.is_empty() {
            Err(FromBufError::Empty)
        } else {
            Ok(buf.chars().next().expect("Unreachable!"))
        }
    }
}

impl FromBuf for Vec<char> {
    type Error = ();

    fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>> {
        let Ok(buf) = String::from_buf(buf) else {
            return Err(FromBufError::InvalidUtf8String)
        };
        if buf.is_empty() {
            Err(FromBufError::Empty)
        } else {
            Ok(buf.chars().collect())
        }
    }
}

impl<T: AtomVec> FromBuf for Vec<T> {
    type Error = T::Error;

    fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>> {
        let Ok(buf) = String::from_buf(buf) else {
            return Err(FromBufError::InvalidUtf8String);
        };
        let splits: Vec<&str> = buf.split(" ").collect();
        let mut values = Vec::new();
        for item in splits {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            let value = T::from_buf(item.as_bytes().to_vec())?;
            values.push(value);
        }
        Ok(values)
    }
}

macro_rules! convert_int {
    ($($ty:ty), *) => {
        $(
            impl FromBuf for $ty {
                type Error = ParseIntError;

                fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>> {
                    let Ok(buf) = String::from_buf(buf) else {
                        return Err(FromBufError::InvalidUtf8String);
                    };
                    match buf.parse() {
                        Ok(value) => Ok(value),
                        Err(e) => {
                            let mut number = 0;
                            match Base::new(&buf) {
                                Some(base) => match base {
                                    Base::Binary(binary) => {
                                        for item in binary.chars() {
                                            if item == '1' {
                                                number = number * 2 + 1;
                                            } else {
                                                number *= 2;
                                            }
                                        }
                                        Ok(number)
                                    }
                                    Base::Octal(oct) => {
                                        for item in oct.as_bytes().iter() {
                                            let value = (*item - b'0') as Self;
                                            number = number * 8 + value;
                                        }
                                        Ok(number)
                                    }
                                    Base::Hex(hex) => {
                                        for item in hex.as_bytes().iter() {
                                            number = number * 16 + to_hex(*item) as Self;
                                        }
                                        Ok(number)
                                    }
                                },
                                _ => Err(FromBufError::ConvertError(e)),
                            }
                        }
                    }
                }
            }

        )*
    };
}

convert_int!(u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128);

macro_rules! convert_float {
    ($($ty:ty), *) => {
        $(
            impl FromBuf for $ty {
                type Error = ParseFloatError;
                fn from_buf(buf: Vec<u8>) -> Result<Self, FromBufError<Self::Error>> {
                    let Ok(buf) = String::from_buf(buf) else {
                        return Err(FromBufError::InvalidUtf8String);
                    };
                    match buf.parse() {
                        Ok(value) => Ok(value),
                        Err(e) => Err(FromBufError::ConvertError(e))
                    }
                }
            }
        )*
    };
}
convert_float!(f32, f64);

enum Base {
    Binary(String),
    Octal(String),
    Hex(String),
}

impl Base {
    fn new(value: &str) -> Option<Base> {
        let value = value.to_ascii_lowercase();
        if value.starts_with("0b") {
            if is_binary(&value[2..]) {
                Some(Base::Binary((&value[2..]).to_owned()))
            } else {
                None
            }
        } else if value.ends_with("b") {
            if is_binary(&value[..value.len() - 1]) {
                Some(Base::Binary((&value[..value.len() - 1]).to_owned()))
            } else {
                None
            }
        } else if value.starts_with("0x") {
            if is_hex(&value[2..]) {
                Some(Base::Hex((&value[2..]).to_owned()))
            } else {
                None
            }
        } else if value.ends_with("h") {
            if is_hex(&value[..value.len() - 1]) {
                Some(Base::Hex((&value[..value.len() - 1]).to_owned()))
            } else {
                None
            }
        } else if value.starts_with("0o") {
            if is_oct(&value[2..]) {
                Some(Base::Octal((&value[2..]).to_owned()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn is_binary(value: &str) -> bool {
    !value.as_bytes().iter().any(|f| *f != b'0' && *f != b'1')
}

fn is_oct(value: &str) -> bool {
    !value
        .as_bytes()
        .iter()
        .any(|value| !(*value >= b'0' && *value <= b'7'))
}

fn is_hex(value: &str) -> bool {
    !value.as_bytes().iter().any(|value| match *value {
        b'0'..=b'7' => false,
        b'a'..=b'f' => false,
        _ => true,
    })
}

fn to_hex(byte: u8) -> i32 {
    match byte {
        b'0'..=b'9' => (byte - b'0') as i32,
        b'a'..=b'f' => (byte - b'a' + 10) as i32,
        b'A'..=b'F' => (byte - b'A' + 10) as i32,
        _ => unreachable!(),
    }
}
