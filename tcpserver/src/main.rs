use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    let connetion_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000");

    for stream in connetion_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connetion established");
        let mut buffer =  [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }


}
