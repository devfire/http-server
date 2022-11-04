fn main() {
    
    let server = Server::new("0.0.0.0:8080".to_string());
    // server.run();
    server.run();
}

struct Server {
    addr: String,
}

impl Server{
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}