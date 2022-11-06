use std::str::Utf8Error;
use std::str;

use crate::http::request;

use super::method::{Method, MethodError};
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
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        
        let request = str::from_utf8(buf)?;
        // Shadowing request - request
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        

        let method: Method = method.parse()?;
        
        unimplemented!()
    }
}

// return a tuple. First part is the first word,
// the second part is the rest of the string.
// We wrap the return part in Option to be able to return NOTHING.
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // return a tuple:
            // first word, skip the space ' ' and then the rest
            return Some((&request[..i], &request[i + 1..]));
        }
    }   

    None
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
 impl Error for ParseError {

 }