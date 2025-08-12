use std::net::TcpStream;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Terminating due to insufficent number of arguments.");
        println!("usage: ./app <ipaddr> <port>");
        std::process::exit(1);
    }

    let ipaddr = &args[1];
    let port = &args[2];
    let socket = format!("{}:{}", ipaddr, port);
    if let Ok(_) = TcpStream::connect(&socket) {
        println!("Connected to {}", socket);
    }
    else {
        println!("Couldn't connect to {}", socket);
    }
}

