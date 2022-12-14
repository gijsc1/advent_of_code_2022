use std::fmt::{Display, Formatter};
use std::num::{ParseIntError, TryFromIntError};

/// A type to combine multiple possible errors, because apparently this is not part of the stdlib?
#[derive(Debug)]
pub enum  Error{
    ParseIntError(ParseIntError),
    IOError(std::io::Error),
    #[allow(dead_code)]
    LocalError{error_type:&'static str,msg:&'static str},
    DynamicError{error_type:&'static str,msg:String},
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<TryFromIntError> for Error{
    fn from(_err: TryFromIntError) -> Self {
        Error::LocalError { error_type: "Conversion error", msg: "Failed to convert signed to unsigned, value was probably negative" }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseIntError(err) => write!(f,"{err}"),
            Error::IOError(err) => write!(f,"{err}"),
            Error::LocalError {error_type,msg} => write!(f,"{error_type}: {msg}"),
            Error::DynamicError {error_type,msg} => write!(f,"{error_type}: {msg}")
        }
    }
}

#[allow(dead_code)]
pub fn parse_error(msg: &'static str) ->Error{
    return Error::LocalError {error_type:"parse error",msg:msg}
}

#[allow(dead_code)]
pub fn dyn_parse_error(msg: String)-> Error{
    return Error::DynamicError {error_type:"parse error",msg:msg};
}

