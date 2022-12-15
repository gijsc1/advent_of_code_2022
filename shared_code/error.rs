use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

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

#[allow(dead_code)]
pub fn parse_prefix<'a>(s:&'a str,prefix:&str)->Result<&'a str,Error>{
    if let Some(rem) = s.strip_prefix(prefix){
        Ok(rem)
    } else {
        Err(dyn_parse_error(format!("Expected: '{}'",prefix)))
    }
}


#[allow(dead_code)]
pub fn parse_posnum(mut s:&str)->Result<(&str,i32),Error>{
    let mut is_neg = false;
    if let Ok(rem) = parse_prefix(s,"-"){
        is_neg = true;
        s = rem;
    }
    let i = s.find(|c:char|!c.is_ascii_digit()).unwrap_or(s.len());
    let mut val:i32 = s[0..i].parse()?;
    if is_neg{
        val= val*-1;
    }
    Ok((&s[i..],val))
    // if let Some(rem) = s.strip_prefix(prefix){
    //     Ok(rem)
    // } else {
    //     Err(parse_error(format!("Expected: '{}'",prefix).as_str()))
    // }
}
