use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, write, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// NOTE: Result<> expects to return itself and an error message
impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // sample request:
    // GET /search HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // unimplemented!()
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // unimplemented!()
        write!(f, "{}", self.message())
    }
}


impl ParseError{
    fn message(&self) -> &str {
        // Because 'match' is the last expression in the fxn body,
        // it is automatically returned!
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
impl Error for ParseError {

}