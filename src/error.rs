use std::{io, num};
#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    ParseFloat(num::ParseFloatError),
    ParseInt(num::ParseIntError),
    OutOfRange,
}

impl From<num::ParseFloatError> for MyError {
    fn from(err: num::ParseFloatError) -> MyError {
        MyError::ParseFloat(err)
    }
}

impl From<num::ParseIntError> for MyError {
    fn from(err: num::ParseIntError) -> MyError {
        MyError::ParseInt(err)
    }
}
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}
