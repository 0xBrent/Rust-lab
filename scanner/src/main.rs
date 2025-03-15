use std::net::{TcpListener,TcpStream};
use std::io::{self, stdin, Write};

pub fn stream(site: TcpListener)  {
    for _clients in site.incoming() { 
    println!("Connection attempt");
    match site.accept() {
        Ok((socket, addr)) => 
        println!("SERVER MSG- Client: {addr:?} Socket:{socket:?}"), 
        Err(e) => println!("Socket error: {e:?}")
    }

    }
}


pub fn client() {
    let mut client_tcp = TcpStream::connect("127.0.0.1:8080");
    for stream in client_tcp {
        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
        println!("CLIENT MSG: {:?}", stream)
        
    }
}

fn main() {
    let site = TcpListener::bind("127.0.0.1:8080").unwrap();
    client();
    stream(site);
}