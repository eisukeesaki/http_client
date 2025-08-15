use std::net::TcpStream;
use std::io::Read;
use std::process;
use std::io::Write;

const DEBUG: &str = "[DEBUG]";

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
            #[cfg(feature="debug")]
            {
                let mut buff = [0; 1024];
                match stream.read(&mut buff) {
                    Ok(bytes_read) => {
                        let response =
                            String::from_utf8_lossy(&buff[0..bytes_read]);
                        println!("[server]\n{}", response);
                    },
                    Err(e) => { // not tested
                        eprintln!("Couldn't read from stream. {}", e);
                    }
                }
            }

            loop {
                println!("\n[Your message]");
                let mut user_input = String::new();
                match std::io::stdin().read_line(&mut user_input) {
                    Ok(_) => {
                        match stream.write_all(user_input.as_bytes()) {
                            Ok(_) => {
                                #[cfg(feature="debug")]
                                {
                                    println!("\n{} Sent message to server.", DEBUG);
                                }
                            },
                            Err(e) => eprintln!("Failed to send \
                                messgage to server. {}", e),
                        }
                    },
                    Err(e) => println!("Failed to read from stdin {}", e),
                }
            }
        },
        Err(e) => { // not tested
            println!("Couldn't connect to {}. {}", socket, e);
        }
    }
}

