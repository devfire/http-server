use super::method::Method;
use std::convert::TryFrom;

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
    type Error = String;

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
}