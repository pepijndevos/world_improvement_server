use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io;
use std::io::Write;
use std::fs::File;
use std::error::Error;

fn handle_client(mut stream: TcpStream) {
    stream.write(b"\nIf everyone would just [...]\n\
                   the world would be a much better place!\n\n\
                   Submit global world improvement using rwall.\n\n");
    File::open("../motd.txt")
        .and_then(|mut f| io::copy(&mut f, &mut stream))
        .unwrap_or_else(|e| stream.write(e.description().as_bytes()).unwrap() as u64);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:17").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}
