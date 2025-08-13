use std::net::TcpStream;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Terminating due to insufficent number of arguments.");
        println!("usage: ./app <ipaddr> <port>");
        process::exit(1);
    }

    let ipaddr = &args[1];
    let port = &args[2];
    let socket = format!("{}:{}", ipaddr, port);

    match TcpStream::connect(&socket) {
        Ok(mut stream) => {
            let mut buff = [0; 1024];
            match stream.read(&mut buff) {
                Ok(bytes_read) => {
                    let response = String::from_utf8_lossy(&buff[0..bytes_read]);
                    println!("{}", response);
                },
                Err(e) => { // not tested
                    eprintln!("Couldn't read from stream. {}", e);
                }
            }
        },
        Err(e) => { // not tested
            println!("Couldn't connect to {}. {}", socket, e);
        }
    }
}

