use std::net::TcpStream;

fn main() {
    let socket = "127.0.0.1:4242";
    if let Ok(_) = TcpStream::connect(socket) {
        println!("Connected to {}", socket);
    }
    else {
        println!("Couldn't connect to {}", socket);
    }
}

