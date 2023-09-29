mod convert;
mod atom;

use std::io::{stdin, BufRead};

use crate::{FromBuf, FromBufError};

pub fn input<T: FromBuf>() -> Result<T, FromBufError<T::Error>> {
    let mut buf = Vec::new();
    stdin().lock().read_until( b'\n', &mut buf)?;
    T::from_buf(buf)
}
