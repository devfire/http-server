use std::net::TcpListener;
use crate::http::Request;
use std::io::Read;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                // NOTE: listener.accept() returns a tuple of stream, address.
                // We need both.
                Ok((mut stream, address)) => {
                    println!("OK!");
                    // allocates a buffer array of 1024 bytes, init to 0
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received: {}", String::from_utf8_lossy(&buffer));
                            // two .. mean "entire array"
                            // compiler cannot do auto-conversion, we must explicitly convert
                            // [..] -- byte slice, entire array
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => {println!("ERROR: failed to parse the request, {}", e)}
                            }
                        }
                        Err(e) => println!("ERROR: failed to read from conn {}", e),
                    }
                },

                // Something bad happened but we move on.
                Err(e) => println!("ERROR: {}", e),
            }
        } 


    }
}