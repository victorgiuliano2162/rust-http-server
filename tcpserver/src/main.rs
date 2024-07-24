use std::net::TcpListener;

fn main() {
    let connetion_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000");

    for stream in connetion_listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connetion established");
    }

    
}
